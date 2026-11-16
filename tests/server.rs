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
