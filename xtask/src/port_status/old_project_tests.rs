use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::json;

use super::*;

#[test]
fn usage_counts_calls_errors_sessions_prompts_and_resources_once_each() {
    // -- Setup & Fixtures
    let call = |id: &str, name: &str, input: Value| {
        json!({
            "type": "assistant",
            "sessionId": "s1",
            "timestamp": "2026-11-10T08:00:00Z",
            "message": { "content": [{ "type": "tool_use", "id": id, "name": name, "input": input }] },
        })
    };
    let result = |id: &str, error: bool| {
        json!({
            "type": "user",
            "sessionId": "s1",
            "message": { "content": [{ "type": "tool_result", "tool_use_id": id, "is_error": error }] },
        })
    };
    let lines = |records: &[Value]| {
        records
            .iter()
            .map(Value::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    };
    let session = lines(&[
        call("t1", "mcp__enfusion-mcp__game_read", json!({})),
        result("t1", true),
        call("t2", "mcp__enfusion-mcp__game_read", json!({})),
        result("t2", false),
        call(
            "t3",
            "ReadMcpResourceTool",
            json!({ "server": "enfusion-mcp", "uri": "enfusion://class/SCR_Foo" }),
        ),
        json!({
            "type": "user",
            "uuid": "u1",
            "sessionId": "s1",
            "message": { "content": "<command-name>/mcp__enfusion-mcp__create-mod</command-name>" },
        }),
    ]);
    // A subagent file repeats call t1: it counts once.
    let subagent = lines(&[
        call("t1", "mcp__enfusion-mcp__game_read", json!({})),
        result("t1", true),
    ]);
    let other_session = lines(&[json!({
        "type": "user",
        "sessionId": "s2",
        "timestamp": "2026-11-12T08:00:00Z",
        "message": { "content": "enfusion-mcp is on" },
    })]);
    let history = out_dir("usage_counts_calls_errors_sessions_prompts_and_resources_once_each");
    write(&history.join("p/s1.jsonl"), &session);
    write(&history.join("p/s1/subagents/a.jsonl"), &subagent);
    write(&history.join("p/s2.jsonl"), &other_session);
    let resources = [Feature {
        kind: Kind::Resource,
        name: "class-info".to_owned(),
        uri: Some("enfusion://class/".to_owned()),
    }];

    // -- Exec
    let usage = read_usage(&history, &resources).unwrap();

    // -- Check
    let counted: BTreeMap<&str, (usize, usize, usize, &str)> = usage
        .counts
        .iter()
        .map(|(key, c)| {
            (
                key.as_str(),
                (c.calls, c.errors, c.sessions.len(), c.last.as_str()),
            )
        })
        .collect();
    assert_eq!(
        counted,
        BTreeMap::from([
            ("tool:game_read", (2, 1, 1, "2026-11-10T08:00:00Z")),
            ("resource:class-info", (1, 0, 1, "2026-11-10T08:00:00Z")),
            ("prompt:create-mod", (1, 0, 1, "")),
        ])
    );
    assert_eq!(
        (
            usage.sessions.len(),
            usage.first.as_str(),
            usage.last.as_str()
        ),
        (2, "2026-11-10T08:00:00Z", "2026-11-12T08:00:00Z")
    );
}

#[test]
fn the_inventory_of_the_old_project_has_all_52_tools_the_prompts_and_the_resources() {
    // -- Setup & Fixtures
    let old_repo = std::env::var_os("PORT_STATUS_OLD_REPO").map_or_else(
        || std::env::home_dir().unwrap().join("www/arma-enfusion-mcp"),
        PathBuf::from,
    );
    if !old_repo.exists() {
        println!("Skipped: no old project at {}.", old_repo.display());
        return;
    }

    // -- Exec
    let features = inventory(&old_repo).unwrap();

    // -- Check
    let names = |kind: Kind| -> Vec<&str> {
        features
            .iter()
            .filter(|f| f.kind == kind)
            .map(|f| f.name.as_str())
            .collect()
    };
    let tools = names(Kind::Tool);
    assert_eq!(tools.len(), 52);
    assert!(tools.contains(&"wb_state") && tools.contains(&"wiki_read"));
    assert_eq!(names(Kind::Prompt), ["create-mod", "modify-mod"]);
    assert_eq!(
        names(Kind::Resource),
        ["class-info", "group-info", "pattern-info"]
    );
    let templates = names(Kind::Template);
    assert!(templates.contains(&"prefab") && !templates.iter().any(|n| n.contains("recipe")));
    assert!(names(Kind::Recipe).contains(&"ui/timer_hud"));
    assert_eq!(names(Kind::Animation), ["animation-graph-code"]);
}

// -- Test Support

/// An empty folder for one test, under xtask/tests/.out/.
fn out_dir(test: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/.out")
        .join(test);
    assert!(dir.to_string_lossy().contains("tests/.out"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn write(path: &Path, text: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}
