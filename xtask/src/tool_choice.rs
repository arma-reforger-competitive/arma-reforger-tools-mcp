//! `cargo xtask tool-choice`: runs every Tool-choice test in `tests/tool-choice/cases.jsonl`.
//!
//! Each test asks Claude Code one question with only the reforger server. A `PreToolUse` hook
//! blocks every call except `ToolSearch` (Claude Code loads MCP Tools with it), so no Tool runs.
//! The built-in Tools stay on, as for a real user, so a test also fails when Claude reaches for
//! Bash instead of a Tool. The first call that is not `ToolSearch` is the answer.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;

use serde::Deserialize;
use serde_json::{Map, Value, json};

use crate::cmd::ToolChoiceArgs;
use crate::{Error, Result};

/// The run fails when fewer of the tests pass.
const PASS_FLOOR: f64 = 0.9;
/// How many Claude Code runs go at the same time.
const PARALLEL: usize = 4;
const CASES: &str = "tests/tool-choice/cases.jsonl";
const TOOL_PREFIX: &str = "mcp__reforger__";

/// One line of `cases.jsonl`.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Case {
    question: String,
    /// The first call must use one of these Tools.
    tools: Vec<String>,
    /// Inputs that the first call must have. A text input matches when it contains the expected
    /// text, in any case. Other inputs must be equal. Inputs that are not listed are not checked.
    #[serde(default)]
    args: Map<String, Value>,
}

/// What one Claude Code run did.
struct Trial {
    /// The first call that is not `ToolSearch`: the Tool name and its inputs.
    call: Option<(String, Value)>,
    cost_usd: f64,
}

pub fn run(args: &ToolChoiceArgs) -> Result<()> {
    let root = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/.."))
        .canonicalize()
        .map_err(|source| Error::Read {
            path: PathBuf::from(".."),
            source,
        })?;
    let cases = for_tool(load_cases(&root.join(CASES))?, args.tool.as_deref())?;
    build_server(&root)?;
    let work = prepare_folder(&root)?;

    let jobs: Vec<&Case> = cases
        .iter()
        .flat_map(|case| std::iter::repeat_n(case, args.runs))
        .collect();
    eprintln!(
        "tool-choice: {} tests x {} runs on {}",
        cases.len(),
        args.runs,
        args.model
    );
    let mut trials = Vec::with_capacity(jobs.len());
    for chunk in jobs.chunks(PARALLEL) {
        let results: Vec<Result<Trial>> = thread::scope(|scope| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|case| scope.spawn(|| ask_claude(&work, &args.model, &case.question)))
                .collect();
            handles
                .into_iter()
                .map(|handle| handle.join().unwrap_or(Err(Error::ClaudeRunStopped)))
                .collect()
        });
        for (case, result) in chunk.iter().zip(results) {
            trials.push((*case, result?));
        }
    }

    let mut passed = 0;
    let mut cost_usd = 0.0;
    for (case, trial) in &trials {
        cost_usd += trial.cost_usd;
        if matches(case, trial.call.as_ref()) {
            passed += 1;
        } else {
            let got = match &trial.call {
                Some((tool, input)) => format!("{tool} {input}"),
                None => "no Tool call".to_string(),
            };
            eprintln!(
                "FAIL  {}\n      want {} {}\n      got  {got}",
                case.question,
                case.tools.join(" or "),
                Value::Object(case.args.clone())
            );
        }
    }
    let total = trials.len();
    #[expect(clippy::cast_precision_loss, reason = "the counts are far below 2^52")]
    let rate = passed as f64 / total.max(1) as f64;
    eprintln!(
        "\ntool-choice: {passed} of {total} passed ({:.0} %), cost ${cost_usd:.2}",
        rate * 100.0
    );
    if rate < PASS_FLOOR {
        return Err(Error::ToolChoiceBelowFloor { passed, total });
    }
    Ok(())
}

// region:    --- Support

/// The tests that accept `tool`, or every test when `tool` is `None`.
fn for_tool(cases: Vec<Case>, tool: Option<&str>) -> Result<Vec<Case>> {
    let Some(tool) = tool else {
        return Ok(cases);
    };
    let cases: Vec<Case> = cases
        .into_iter()
        .filter(|case| case.tools.iter().any(|name| name == tool))
        .collect();
    if cases.is_empty() {
        return Err(Error::NoToolChoiceCase {
            tool: tool.to_owned(),
        });
    }
    Ok(cases)
}

fn load_cases(path: &Path) -> Result<Vec<Case>> {
    let text = fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_path_buf(),
        source,
    })?;
    text.lines()
        .enumerate()
        .filter(|(_, line)| !line.trim().is_empty())
        .map(|(index, line)| {
            serde_json::from_str(line).map_err(|source| Error::BadCase {
                line: index + 1,
                source,
            })
        })
        .collect()
}

fn build_server(root: &Path) -> Result<()> {
    let status = Command::new("cargo")
        .args(["build", "--release", "--quiet", "--package", "reforger"])
        .current_dir(root)
        .status()
        .map_err(|source| Error::CiStepNotStarted {
            step: "cargo build --release",
            source,
        })?;
    if !status.success() {
        return Err(Error::CiStepFailed {
            step: "cargo build --release",
            status,
        });
    }
    Ok(())
}

/// Writes the hook settings and the server config into an empty folder, and returns the folder
/// that Claude Code runs in. It is empty, so a search in it finds nothing.
fn prepare_folder(root: &Path) -> Result<PathBuf> {
    let base = std::env::temp_dir().join("reforger-tool-choice");
    let project = base.join("project");
    fs::create_dir_all(&project).map_err(|source| Error::Write {
        path: project.clone(),
        source,
    })?;
    let deny = json!({
        "hookSpecificOutput": {
            "hookEventName": "PreToolUse",
            "permissionDecision": "deny",
            "permissionDecisionReason": "Tool calls are off in this test. Say which Tool you would call, with which inputs."
        }
    });
    let hook = format!("grep -Eq '\"tool_name\": *\"ToolSearch\"' || echo '{deny}'");
    let settings = json!({
        "hooks": { "PreToolUse": [{ "matcher": "*", "hooks": [{ "type": "command", "command": hook }] }] }
    });
    let server = root.join("target/release/reforger");
    let mcp = json!({ "mcpServers": { "reforger": { "command": server } } });
    for (name, value) in [("settings.json", settings), ("mcp.json", mcp)] {
        let path = base.join(name);
        fs::write(&path, value.to_string()).map_err(|source| Error::Write { path, source })?;
    }
    Ok(project)
}

/// Asks Claude Code one question and returns its first call.
fn ask_claude(project: &Path, model: &str, question: &str) -> Result<Trial> {
    let base = project.parent().unwrap_or(project);
    let output = Command::new("claude")
        .arg("-p")
        .arg(question)
        .args(["--model", model, "--setting-sources", ""])
        .arg("--settings")
        .arg(base.join("settings.json"))
        .arg("--mcp-config")
        .arg(base.join("mcp.json"))
        .args([
            "--strict-mcp-config",
            "--max-turns",
            "3",
            "--output-format",
            "stream-json",
            "--verbose",
        ])
        .current_dir(project)
        .output()
        .map_err(|source| Error::ClaudeNotStarted { source })?;
    Ok(read_trial(&String::from_utf8_lossy(&output.stdout)))
}

/// Reads the first call that is not `ToolSearch`, and the cost, from Claude Code's stream-json
/// output.
fn read_trial(stream: &str) -> Trial {
    let mut call = None;
    let mut cost_usd = 0.0;
    for message in stream
        .lines()
        .filter_map(|line| serde_json::from_str::<Value>(line).ok())
    {
        match message.get("type").and_then(Value::as_str) {
            Some("assistant") if call.is_none() => {
                let mut blocks = message
                    .pointer("/message/content")
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten();
                call = blocks.find_map(tool_call);
            }
            Some("result") => {
                cost_usd = message
                    .get("total_cost_usd")
                    .and_then(Value::as_f64)
                    .unwrap_or(0.0);
            }
            _ => {}
        }
    }
    Trial { call, cost_usd }
}

/// The Tool name, without the server prefix, and the inputs of a `tool_use` block that is not
/// `ToolSearch`.
fn tool_call(block: &Value) -> Option<(String, Value)> {
    if block.get("type").and_then(Value::as_str) != Some("tool_use") {
        return None;
    }
    let name = block.get("name").and_then(Value::as_str)?;
    if name == "ToolSearch" {
        return None;
    }
    let name = name.strip_prefix(TOOL_PREFIX).unwrap_or(name);
    let input = block.get("input").cloned().unwrap_or(Value::Null);
    Some((name.to_string(), input))
}

fn matches(case: &Case, call: Option<&(String, Value)>) -> bool {
    let Some((tool, input)) = call else {
        return false;
    };
    case.tools.contains(tool)
        && case
            .args
            .iter()
            .all(|(key, want)| match (want, input.get(key)) {
                (Value::String(want), Some(Value::String(got))) => {
                    got.to_lowercase().contains(&want.to_lowercase())
                }
                (want, got) => Some(want) == got,
            })
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn case(line: &str) -> Case {
        serde_json::from_str(line).expect("the test case is valid JSON")
    }

    #[test]
    fn reads_the_first_call_that_is_not_tool_search_and_the_cost() {
        // -- Setup & Fixtures
        // The shape of Claude Code 2.1.325 stream-json output, cut to the read fields.
        let stream = [
            r#"{"type":"system","subtype":"init"}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"tool_use","name":"ToolSearch","input":{"query":"select:mcp__reforger__api_search"}}]}}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"text","text":"I will search."},{"type":"tool_use","name":"mcp__reforger__api_search","input":{"query":"GetFocusFOVScale","limit":10}}]}}"#,
            r#"{"type":"assistant","message":{"content":[{"type":"tool_use","name":"Bash","input":{"command":"ls"}}]}}"#,
            r#"{"type":"result","total_cost_usd":0.0123}"#,
        ]
        .join("\n");

        // -- Exec
        let trial = read_trial(&stream);

        // -- Check
        assert_eq!(
            trial.call,
            Some((
                "api_search".to_string(),
                json!({"query": "GetFocusFOVScale", "limit": 10})
            ))
        );
        assert!((trial.cost_usd - 0.0123).abs() < f64::EPSILON);
    }

    #[test]
    fn matches_a_text_input_that_contains_the_expected_text_in_any_case() {
        // -- Setup & Fixtures
        let want = case(r#"{"question":"q","tools":["game_find"],"args":{"query":"scr_pregame"}}"#);
        let call = (
            "game_find".to_string(),
            json!({"query": "SCR_PreGameGameModeStateComponent", "limit": 20}),
        );

        // -- Exec & Check
        assert!(matches(&want, Some(&call)));
    }

    #[test]
    fn fails_a_call_to_another_tool_a_missing_input_or_no_call() {
        // -- Setup & Fixtures
        let want =
            case(r#"{"question":"q","tools":["wiki_read"],"args":{"title":"Mod Project Setup"}}"#);

        // -- Exec & Check
        assert!(!matches(
            &want,
            Some(&(
                "wiki_search".to_string(),
                json!({"query": "Mod Project Setup"})
            ))
        ));
        assert!(!matches(&want, Some(&("wiki_read".to_string(), json!({})))));
        assert!(!matches(&want, None));
    }

    #[test]
    fn asks_only_the_tests_that_accept_the_tool_of_the_tool_input() {
        // -- Setup & Fixtures
        let cases = || {
            vec![
                case(r#"{"question":"a","tools":["game_find","game_read"]}"#),
                case(r#"{"question":"b","tools":["wiki_read"]}"#),
                case(r#"{"question":"c","tools":["game_read"]}"#),
            ]
        };
        let questions = |cases: Vec<Case>| {
            cases
                .into_iter()
                .map(|case| case.question)
                .collect::<Vec<_>>()
        };

        // -- Exec & Check
        assert_eq!(
            questions(for_tool(cases(), Some("game_read")).unwrap()),
            ["a", "c"]
        );
        assert_eq!(questions(for_tool(cases(), None).unwrap()), ["a", "b", "c"]);
        assert!(matches!(
            for_tool(cases(), Some("game_reed")),
            Err(Error::NoToolChoiceCase { tool }) if tool == "game_reed"
        ));
    }
}

// endregion: --- Tests
