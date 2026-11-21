//! One Tool call to `target/release/reforger` over stdio, with the rmcp client.

use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;

use rmcp::model::{CallToolRequestParams, CallToolResult, ClientConfig, JsonObject};
use rmcp::service::{RoleClient, RunningService};
use rmcp::transport::TokioChildProcess;
use rmcp::{ServiceError, ServiceExt as _};
use serde_json::Value;
use tokio::process::Command;

use super::{Error, Result};
use crate::cmd::CallToolArgs;

/// How long one call can take. `workbench_launch` can wait 180 seconds, plus the Steam start.
const CALL_WAIT: Duration = Duration::from_secs(300);

/// The start of the names of the plugin's settings.
const SETTING_PREFIX: &str = "REFORGER_";

pub fn run(args: &CallToolArgs) -> Result<()> {
    let inputs = parse_inputs(&args.inputs)?;
    let file_env = read_settings_env(&settings_path()?)?;
    let env = settings(&file_env, std::env::vars_os());
    let server = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../target/release/reforger"
    ));
    if !server.is_file() {
        return Err(Error::ServerNotBuilt {
            path: server.to_owned(),
        });
    }

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(Error::Runtime)?;
    let answer = runtime
        .block_on(async {
            tokio::time::timeout(CALL_WAIT, call(server, &env, &args.tool, &inputs)).await
        })
        .map_err(|_| Error::NoAnswer {
            tool: args.tool.clone(),
            secs: CALL_WAIT.as_secs(),
        })??;

    let (text, is_error) = answer_text(&answer);
    println!("{text}");
    if is_error {
        return Err(Error::ToolFailed {
            tool: args.tool.clone(),
        });
    }
    Ok(())
}

// region:    --- Support

/// The path of the settings file of Claude Code: `~/.claude/settings.json`.
fn settings_path() -> Result<PathBuf> {
    let home = std::env::var_os("HOME").ok_or(Error::HomeNotSet)?;
    Ok(Path::new(&home).join(".claude/settings.json"))
}

/// The `env` object of the settings file. A file with no `env` object gives an empty object.
fn read_settings_env(path: &Path) -> Result<JsonObject> {
    let text = std::fs::read_to_string(path).map_err(|source| Error::ReadSettings {
        path: path.to_owned(),
        source,
    })?;
    let file: Value = serde_json::from_str(&text).map_err(|source| Error::BadSettings {
        path: path.to_owned(),
        source,
    })?;
    Ok(match file.get("env") {
        Some(Value::Object(env)) => env.clone(),
        _ => JsonObject::new(),
    })
}

/// The `REFORGER_*` settings: the string values of the settings file, then the variables of the
/// shell, which win. A shell variable that is not UTF-8 is skipped.
fn settings(
    from_file: &JsonObject,
    from_shell: impl IntoIterator<Item = (OsString, OsString)>,
) -> BTreeMap<String, String> {
    let file = from_file
        .iter()
        .filter_map(|(name, value)| Some((name.clone(), value.as_str()?.to_owned())));
    let shell = from_shell
        .into_iter()
        .filter_map(|(name, value)| Some((name.into_string().ok()?, value.into_string().ok()?)));
    file.chain(shell)
        .filter(|(name, _)| name.starts_with(SETTING_PREFIX))
        .collect()
}

/// Splits each `name=value` input at the first `=`. The name and the value must not be empty.
fn parse_inputs(inputs: &[String]) -> Result<Vec<(&str, &str)>> {
    inputs
        .iter()
        .map(|input| match input.split_once('=') {
            Some((name, value)) if !name.is_empty() && !value.is_empty() => Ok((name, value)),
            _ => Err(Error::BadInput {
                input: input.clone(),
            }),
        })
        .collect()
}

/// The arguments of the call. A property of type "string" in the input schema gets the raw text.
/// Any other value is read as JSON, and the raw text is the fallback when it is not JSON.
fn arguments(inputs: &[(&str, &str)], schema: Option<&JsonObject>) -> JsonObject {
    inputs
        .iter()
        .map(|&(name, raw)| {
            let is_string = schema
                .and_then(|schema| schema.get("properties"))
                .and_then(|properties| properties.get(name))
                .and_then(|property| property.get("type"))
                .and_then(Value::as_str)
                == Some("string");
            let value = if is_string {
                Value::from(raw)
            } else {
                serde_json::from_str(raw).unwrap_or_else(|_| Value::from(raw))
            };
            (name.to_owned(), value)
        })
        .collect()
}

/// The text of each content block, joined with a new line, and whether the answer is an error.
/// A block that is not text gives an empty line.
fn answer_text(answer: &CallToolResult) -> (String, bool) {
    let text = answer
        .content
        .iter()
        .map(|block| block.as_text().map_or("", |text| text.text.as_str()))
        .collect::<Vec<_>>()
        .join("\n");
    (text, answer.is_error == Some(true))
}

/// Starts the server with the settings, calls the Tool, and closes the server.
async fn call(
    server: &Path,
    env: &BTreeMap<String, String>,
    tool: &str,
    inputs: &[(&str, &str)],
) -> Result<CallToolResult> {
    let mut command = Command::new(server);
    command.envs(env);
    // The server writes its log lines to stderr. Only the answer goes to the output.
    let (transport, _) = TokioChildProcess::builder(command)
        .stderr(Stdio::null())
        .spawn()
        .map_err(|source| Error::ServerNotStarted {
            path: server.to_owned(),
            source,
        })?;
    let client = ClientConfig::default()
        .serve(transport)
        .await
        .map_err(|err| Error::NoHandshake(Box::new(err)))?;
    let answer = ask(&client, tool, inputs)
        .await
        .map_err(|source| Error::Mcp {
            tool: tool.to_owned(),
            source: Box::new(source),
        });
    // The answer is complete here, so a failure to close the server changes nothing.
    let _ = client.cancel().await;
    answer
}

/// Reads the input schema of the Tool, and calls it. A Tool that the server does not list has no
/// schema: the server answers the call with its own error.
async fn ask(
    client: &RunningService<RoleClient, ClientConfig>,
    tool: &str,
    inputs: &[(&str, &str)],
) -> core::result::Result<CallToolResult, ServiceError> {
    let tools = client.list_all_tools().await?;
    let schema = tools
        .iter()
        .find(|listed| listed.name == tool)
        .map(|listed| listed.input_schema.as_ref());
    let request =
        CallToolRequestParams::new(tool.to_owned()).with_arguments(arguments(inputs, schema));
    client.call_tool(request).await
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use rmcp::model::ContentBlock;
    use serde_json::json;

    use super::*;

    #[test]
    fn the_settings_are_the_reforger_values_of_settings_json_and_the_shell_wins() {
        // -- Setup & Fixtures
        let from_file = json!({
            "REFORGER_GAME_PATH": "/mnt/d/Game",
            "REFORGER_MODS_PATH": "D:\\Mods",
            "OTHER": "x",
        });
        let from_file = from_file.as_object().unwrap();
        let from_shell = [
            ("REFORGER_MODS_PATH", "/tmp/test-lists"),
            ("PATH", "/usr/bin"),
        ]
        .map(|(name, value)| (OsString::from(name), OsString::from(value)));

        // -- Exec
        let settings = settings(from_file, from_shell);

        // -- Check
        assert_eq!(
            settings,
            BTreeMap::from([
                ("REFORGER_GAME_PATH".to_owned(), "/mnt/d/Game".to_owned()),
                (
                    "REFORGER_MODS_PATH".to_owned(),
                    "/tmp/test-lists".to_owned()
                ),
            ])
        );
    }

    #[test]
    fn a_string_property_gets_the_raw_text_and_another_property_gets_the_json_value() {
        // -- Setup & Fixtures
        let schema = json!({
            "type": "object",
            "properties": {
                "query": { "type": "string" },
                "limit": { "type": "number" },
            },
        });
        let schema = schema.as_object().unwrap();
        let inputs = ["query=S3C", "limit=123"].map(str::to_owned);
        let number_as_text = ["query=123"].map(str::to_owned);

        // -- Exec
        let args = arguments(&parse_inputs(&inputs).unwrap(), Some(schema));
        let text_args = arguments(&parse_inputs(&number_as_text).unwrap(), Some(schema));

        // -- Check
        assert_eq!(Value::Object(args), json!({ "query": "S3C", "limit": 123 }));
        assert_eq!(Value::Object(text_args), json!({ "query": "123" }));
    }

    #[test]
    fn an_input_without_a_name_a_value_or_an_equals_sign_is_an_error() {
        // -- Exec & Check
        for input in ["query", "=S3C", "query="] {
            let err = parse_inputs(&[input.to_owned()]).unwrap_err();
            assert_eq!(
                err.to_string(),
                format!(
                    "\"{input}\" is not an input. Write each input as name=value, for example query=S3C."
                )
            );
        }
    }

    #[test]
    fn the_answer_is_the_text_of_the_tool() {
        // -- Setup & Fixtures
        // The text of an answer in the check of #77.
        let answer = CallToolResult::success(vec![ContentBlock::text(
            "S3C_CameraFix (6A09759015FFDA26): present",
        )]);

        // -- Exec
        let (text, is_error) = answer_text(&answer);

        // -- Check
        assert_eq!(text, "S3C_CameraFix (6A09759015FFDA26): present");
        assert!(!is_error);
    }
}

// endregion: --- Tests
