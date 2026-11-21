use rmcp::{
    ServiceExt,
    model::{CallToolRequestParams, ClientConfig},
    service::{RoleClient, RunningService},
    transport::TokioChildProcess,
};
use serde_json::{Value, json};
use std::process::Stdio;
use tokio::{io::AsyncWriteExt, process::Command};

#[tokio::test]
async fn starts_the_reforger_server_over_stdio_and_answers_wiki_read() {
    // -- Setup & Fixtures
    let client = start().await;

    // -- Exec
    let server_info = client.peer_info().unwrap().server_info.clone().unwrap();
    let (text, _) = read_page(&client, "Arma Reforger:Mod Project Setup").await;

    // -- Check
    assert_eq!(server_info.name, "reforger");
    assert!(text.contains("## Arma Reforger:Mod Project Setup"));

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn answers_api_search_with_the_class_that_owns_a_method() {
    // -- Setup & Fixtures
    let client = start().await;

    // -- Exec
    let (text, is_error) = call(
        &client,
        "api_search",
        json!({ "query": "GetFocusFOVScale" }),
    )
    .await;

    // -- Check
    // From the PlayerCamera page in the Enfusion script API docs.
    assert!(!is_error);
    assert!(
        text.contains("1. PlayerCamera.GetFocusFOVScale (method)"),
        "{text}"
    );

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn answers_game_find_with_the_setting_to_fix_when_the_game_folder_is_wrong() {
    // -- Setup & Fixtures
    let mut command = Command::new(env!("CARGO_BIN_EXE_reforger"));
    command.env("REFORGER_GAME_PATH", r"Q:\No Such Folder");
    let server = TokioChildProcess::new(command).unwrap();
    let client = ClientConfig::default().serve(server).await.unwrap();

    // -- Exec
    let (text, is_error) = call(&client, "game_find", json!({ "query": "Camera" })).await;

    // -- Check
    // In WSL, the Windows path is read as /mnt/q/No Such Folder.
    assert!(is_error);
    assert!(
        text.starts_with(
            r#"REFORGER_GAME_PATH is "Q:\No Such Folder", but /mnt/q/No Such Folder has no addons folder with .pak files."#
        ),
        "{text}"
    );

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn cuts_a_very_long_page_at_100_000_characters_and_says_so() {
    // -- Setup & Fixtures
    let client = start().await;

    // -- Exec
    let (text, is_error) = read_page(&client, "Arma Reforger:Diag Menu").await;

    // -- Check
    assert!(!is_error);
    assert!(text.chars().count() < 100_100);
    // Counted by hand: 164,467 characters of page text, 27 of the title line, 86 of the source
    // line, and 2 of the empty line after it.
    assert!(text.ends_with("\n\n(Cut at 100,000 of 164,582 characters.)"));

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn names_every_served_tool_and_no_other_in_the_server_guide() {
    // -- Setup & Fixtures
    let client = start().await;

    // -- Exec
    let guide = client
        .peer_info()
        .unwrap()
        .instructions
        .clone()
        .expect("the server sends a Server guide");
    let tools = client.list_all_tools().await.unwrap();
    // The guide can name a Tool input in backticks too, so that Claude uses the right name.
    let inputs: Vec<&String> = tools
        .iter()
        .filter_map(|tool| tool.input_schema.get("properties")?.as_object())
        .flat_map(|properties| properties.keys())
        .collect();
    let mut named: Vec<&str> = guide
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|word| !inputs.iter().any(|input| input.as_str() == *word))
        .collect();
    let mut served: Vec<String> = tools.iter().map(|tool| tool.name.to_string()).collect();

    // -- Check
    named.sort_unstable();
    named.dedup();
    served.sort_unstable();
    assert_eq!(named, served);

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn has_a_row_for_every_served_tool_in_the_readme_tools_table() {
    // -- Setup & Fixtures
    let client = start().await;
    let readme = include_str!("../README.md");
    let table = readme
        .split_once("\n## Tools\n")
        .expect("README.md has a Tools section")
        .1
        .split("\n## ")
        .next()
        .unwrap_or_default();

    // -- Exec
    // A row starts with the Tool name in backticks: "| `wiki_read` | ... |".
    let rows: Vec<&str> = table
        .lines()
        .filter_map(|line| line.strip_prefix("| `")?.split_once('`'))
        .map(|(name, _)| name)
        .collect();

    // -- Exec & Check
    for tool in client.list_all_tools().await.unwrap() {
        assert!(
            rows.contains(&tool.name.as_ref()),
            "the server serves {}, but README.md has no row for it: add its row to the Tools \
             table in README.md",
            tool.name
        );
    }

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn names_only_served_tools_in_the_plugin_skills() {
    // -- Setup & Fixtures
    let client = start().await;
    let served: Vec<String> = client
        .list_all_tools()
        .await
        .unwrap()
        .into_iter()
        .map(|tool| tool.name.into_owned())
        .collect();
    let skills = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("skills");

    // -- Exec & Check
    // A Tool name is a word in backticks of lower-case letters and `_`, with a letter first, for
    // example `guid_generate`. Each one in a skill must be a Tool that the server serves.
    for entry in std::fs::read_dir(&skills).expect("the repo has a skills folder") {
        let file = entry.unwrap().path().join("SKILL.md");
        let text = std::fs::read_to_string(&file)
            .unwrap_or_else(|error| panic!("cannot read {}: {error}", file.display()));
        for (index, word) in text.split('`').enumerate() {
            let in_backticks = index % 2 == 1;
            let looks_like_a_tool = word.contains('_')
                && word.starts_with(|c: char| c.is_ascii_lowercase())
                && word.chars().all(|c| c.is_ascii_lowercase() || c == '_');
            if in_backticks && looks_like_a_tool {
                assert!(
                    served.iter().any(|tool| tool == word),
                    "{} names the Tool `{word}`, but the server does not serve it: use the \
                     name of a served Tool, or add the Tool to the server",
                    file.display()
                );
            }
        }
    }

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn gives_every_tool_a_title_and_all_four_tool_labels() {
    // -- Setup & Fixtures
    let client = start().await;

    // -- Exec & Check
    for tool in client.list_all_tools().await.unwrap() {
        let name = &tool.name;
        assert!(
            tool.title.as_deref().is_some_and(|t| !t.trim().is_empty()),
            "{name} has no title"
        );
        let labels = tool
            .annotations
            .unwrap_or_else(|| panic!("{name} has no Tool labels"));
        assert!(
            labels.read_only_hint.is_some(),
            "{name}: say if it is read-only"
        );
        assert!(
            labels.destructive_hint.is_some(),
            "{name}: say if it is destructive"
        );
        assert!(
            labels.idempotent_hint.is_some(),
            "{name}: say if it is safe to call again"
        );
        assert!(
            labels.open_world_hint.is_some(),
            "{name}: say if it uses the internet"
        );
    }

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn describes_every_tool_and_every_tool_input() {
    // -- Setup & Fixtures
    let client = start().await;

    // -- Exec & Check
    for tool in client.list_all_tools().await.unwrap() {
        let description = tool.description.as_deref().unwrap_or_default();
        assert!(
            !description.trim().is_empty(),
            "{} has no description",
            tool.name
        );
        let inputs = tool
            .input_schema
            .get("properties")
            .and_then(serde_json::Value::as_object)
            .into_iter()
            .flatten();
        for (input, schema) in inputs {
            let description = schema
                .get("description")
                .and_then(serde_json::Value::as_str)
                .unwrap_or_default();
            assert!(
                !description.trim().is_empty(),
                "{} has no description for the input {input}",
                tool.name
            );
        }
    }

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn gives_every_tool_an_input_schema_of_type_object() {
    // -- Setup & Fixtures
    let client = start().await;

    // -- Exec & Check
    // The MCP specification says that the input schema of a Tool has "type": "object", also when
    // the Tool has no inputs. MCP Inspector does not list a Tool whose schema is `{}`.
    for tool in client.list_all_tools().await.unwrap() {
        assert_eq!(
            tool.input_schema.get("type"),
            Some(&json!("object")),
            "{}: the input schema needs \"type\": \"object\", or a client leaves the Tool out",
            tool.name
        );
    }

    client.cancel().await.unwrap();
}

#[tokio::test]
async fn writes_logs_to_stderr_and_only_mcp_messages_to_stdout() {
    // -- Setup & Fixtures
    let mut server = Command::new(env!("CARGO_BIN_EXE_reforger"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let initialize = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "protocolVersion": "2025-06-18",
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.0.0" }
        }
    });
    let initialized = json!({ "jsonrpc": "2.0", "method": "notifications/initialized" });

    // -- Exec
    let mut stdin = server.stdin.take().unwrap();
    stdin
        .write_all(format!("{initialize}\n{initialized}\n").as_bytes())
        .await
        .unwrap();
    drop(stdin);
    let output = server.wait_with_output().await.unwrap();

    // -- Check
    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stdout.trim().is_empty(), "the server did not answer");
    for line in stdout.lines() {
        let message: Value = serde_json::from_str(line)
            .unwrap_or_else(|_| panic!("stdout has a line that is not JSON: {line}"));
        assert_eq!(
            message["jsonrpc"], "2.0",
            "stdout has a line that is not MCP: {line}"
        );
    }
    assert!(
        stderr.contains("Service initialized as server"),
        "the rmcp log is not on stderr: {stderr}"
    );
}

// region:    --- Test Support

/// Starts the real program and connects a client to it over stdio.
async fn start() -> RunningService<RoleClient, ClientConfig> {
    let server = TokioChildProcess::new(Command::new(env!("CARGO_BIN_EXE_reforger"))).unwrap();
    ClientConfig::default().serve(server).await.unwrap()
}

/// Calls `wiki_read` with `title` and returns the text of the answer and its error flag.
async fn read_page(
    client: &RunningService<RoleClient, ClientConfig>,
    title: &str,
) -> (String, bool) {
    call(client, "wiki_read", json!({ "title": title })).await
}

/// Calls `tool` with `arguments` and returns the text of the answer and its error flag.
async fn call(
    client: &RunningService<RoleClient, ClientConfig>,
    tool: &'static str,
    arguments: Value,
) -> (String, bool) {
    let result = client
        .call_tool(
            CallToolRequestParams::new(tool).with_arguments(arguments.as_object().unwrap().clone()),
        )
        .await
        .unwrap();
    let text = result.content[0]
        .as_text()
        .unwrap_or_else(|| panic!("{tool} did not answer with text"))
        .text
        .clone();
    (text, result.is_error.unwrap_or(false))
}

// endregion: --- Test Support
