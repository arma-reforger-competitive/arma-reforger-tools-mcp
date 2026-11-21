//! Reads the features of the old project and their use in Claude Code session history.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use regex::Regex;
use serde_json::Value;

use super::files::{files_under, names_under, read_text, relative, without_extension};
use super::ledger::{Kind, kind_id};
use super::{Error, Result};

/// One feature of the old project. `uri` is the start of a resource's URI, for example
/// `enfusion://class/`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    pub kind: Kind,
    pub name: String,
    pub uri: Option<String>,
}

/// The use of one feature.
#[derive(Debug, Default)]
pub struct Count {
    pub calls: usize,
    pub errors: usize,
    pub sessions: HashSet<String>,
    pub last: String,
}

/// Use of the old server in Claude Code session history. Keys of `counts` are "<kind>:<name>".
#[derive(Debug, Default)]
pub struct Usage {
    pub counts: HashMap<String, Count>,
    pub sessions: HashSet<String>,
    pub first: String,
    pub last: String,
}

const OLD_SERVER: &str = "enfusion-mcp";
const TOOL_PREFIX: &str = "mcp__enfusion-mcp__";

const TOOL_CALL: &str = r#"registerTool\(\s*"([a-z_]+)""#;
// `R`: a line can also end with "\r\n".
const TOOL_NAME_LINE: &str = r#"(?mR)^\s*name: "([a-z_]+)",?$"#;
const PROMPT_CALL: &str = r#"registerPrompt\(\s*"([a-z-]+)""#;
const RESOURCE_CALL: &str =
    r#"registerResource\(\s*"([a-z-]+)",\s*new ResourceTemplate\("([^"{]+)"#;
const PROMPT_COMMAND: &str = r"<command-name>/?(?:mcp__enfusion-mcp__|enfusion-mcp:)([a-z-]+)";

/// Lists every feature of the old project, read from its source files.
pub fn inventory(old_repo: &Path) -> Result<Vec<Feature>> {
    let mut features = Vec::new();
    let src = old_repo.join("src");
    let source = files_under(&src, ".ts")?
        .iter()
        .map(|file| read_text(file))
        .collect::<Result<Vec<_>>>()?
        .join("\n");

    // Most tools call registerTool("name"). The Workbench tools give `name: "wb_..."` alone on a
    // line.
    let mut tools = first_groups(&source, TOOL_CALL)?;
    tools.extend(first_groups(&source, TOOL_NAME_LINE)?);
    add(&mut features, Kind::Tool, tools);
    add(
        &mut features,
        Kind::Prompt,
        first_groups(&source, PROMPT_CALL)?,
    );
    for found in regex(RESOURCE_CALL)?.captures_iter(&source) {
        if let (Some(name), Some(uri)) = (found.get(1), found.get(2)) {
            features.push(Feature {
                kind: Kind::Resource,
                name: name.as_str().to_owned(),
                uri: Some(uri.as_str().to_owned()),
            });
        }
    }

    // Files with "recipe" in the name are the code of the recipes, not templates.
    let templates = src.join("templates");
    let template_names = files_under(&templates, ".ts")?
        .iter()
        .map(|file| without_extension(&relative(&templates, file)))
        .filter(|name| !name.contains("recipe"))
        .collect();
    let data = old_repo.join("data");
    let recipes = names_under(&data.join("recipes"), ".json")?;
    let patterns = names_under(&data.join("patterns"), ".json")?;
    let knowledge = names_under(&data.join("kb").join("patterns"), ".md")?;
    let animation = if src.join("animation").exists() {
        vec!["animation-graph-code".to_owned()]
    } else {
        Vec::new()
    };
    add(&mut features, Kind::Template, template_names);
    add(&mut features, Kind::Recipe, recipes);
    add(&mut features, Kind::Pattern, patterns);
    add(&mut features, Kind::Knowledge, knowledge);
    add(&mut features, Kind::Animation, animation);
    Ok(features)
}

/// Reads the calls to the old server's tools, prompts, and resources from Claude Code session
/// history.
pub fn read_usage(history_dir: &Path, features: &[Feature]) -> Result<Usage> {
    let mut usage = Usage::default();
    let resources: Vec<(&str, &str)> = features
        .iter()
        .filter_map(|feature| {
            let uri = feature.uri.as_deref().filter(|uri| !uri.is_empty())?;
            Some((feature.name.as_str(), uri))
        })
        .collect();
    let prompt_command = regex(PROMPT_COMMAND)?;
    // The key and the error mark of each call, by call ID. A missing ID is a key of its own.
    let mut calls: HashMap<Option<String>, (String, bool)> = HashMap::new();
    let mut seen: HashSet<Option<String>> = HashSet::new();

    for file in files_under(history_dir, ".jsonl")? {
        let file_key = file.to_string_lossy().into_owned();
        for line in read_text(&file)?.split('\n') {
            let names_old_server = line.contains(OLD_SERVER);
            if !names_old_server && !line.contains("\"tool_result\"") {
                continue;
            }
            let Ok(record) = serde_json::from_str::<Value>(line) else {
                continue;
            };
            let session = match record.get("sessionId") {
                None | Some(Value::Null) => file_key.clone(),
                Some(Value::String(id)) => id.clone(),
                Some(other) => other.to_string(),
            };
            let time = record
                .get("timestamp")
                .and_then(Value::as_str)
                .unwrap_or_default();
            if names_old_server {
                usage.sessions.insert(session.clone());
                if !time.is_empty() && (usage.first.is_empty() || *time < *usage.first) {
                    time.clone_into(&mut usage.first);
                }
                if *time > *usage.last {
                    time.clone_into(&mut usage.last);
                }
            }
            let content = record.get("message").and_then(|m| m.get("content"));

            // An MCP prompt runs as a slash command.
            let text = content.and_then(Value::as_str).unwrap_or_default();
            if let Some(prompt) = prompt_command.captures(text).and_then(|c| c.get(1))
                && seen.insert(id_key(record.get("uuid")))
            {
                count(
                    &mut usage,
                    format!("prompt:{}", prompt.as_str()),
                    &session,
                    time,
                );
            }

            let Some(Value::Array(blocks)) = content else {
                continue;
            };
            for block in blocks {
                let kind = block.get("type").and_then(Value::as_str);
                let id = id_key(block.get("id"));
                if kind == Some("tool_use") && !calls.contains_key(&id) {
                    let Some(key) = call_key(block, &resources) else {
                        continue;
                    };
                    calls.insert(id, (key.clone(), false));
                    count(&mut usage, key, &session, time);
                }
                let is_error = block.get("is_error") == Some(&Value::Bool(true));
                if kind == Some("tool_result") && is_error {
                    let call = calls.get_mut(&id_key(block.get("tool_use_id")));
                    if let Some((key, errors)) = call.filter(|(_, errors)| !*errors) {
                        *errors = true;
                        if let Some(entry) = usage.counts.get_mut(key) {
                            entry.errors += 1;
                        }
                    }
                }
            }
        }
    }
    Ok(usage)
}

/// What the usage suggests for one feature. Only a suggestion: the owner decides.
pub fn suggest(feature: &Feature, usage: &Usage) -> String {
    let sessions = usage.sessions.len();
    let Some(tools) = consumers(feature) else {
        let key = format!("{}:{}", kind_id(feature.kind), feature.name);
        let Some(used) = usage.counts.get(&key) else {
            return format!("never used in {sessions} sessions, suggest: not needed");
        };
        let last: String = used.last.chars().take(10).collect();
        return format!(
            "used {} times in {} of {sessions} sessions, {} errors, last {last}, suggest: port",
            used.calls,
            used.sessions.len(),
            used.errors
        );
    };
    if tools.is_empty() {
        return "no old tool uses it, suggest: not needed".to_owned();
    }
    let used: Vec<(&str, usize)> = tools
        .iter()
        .map(|&tool| {
            let calls = usage
                .counts
                .get(&format!("tool:{tool}"))
                .map_or(0, |c| c.calls);
            (tool, calls)
        })
        .filter(|&(_, calls)| calls > 0)
        .collect();
    if used.is_empty() {
        return format!(
            "its tools ({}) were never used in {sessions} sessions, suggest: not needed",
            tools.join(", ")
        );
    }
    let through: Vec<String> = used
        .iter()
        .map(|(tool, calls)| format!("{tool} ({calls} calls)"))
        .collect();
    let names: Vec<&str> = used.iter().map(|&(tool, _)| tool).collect();
    format!(
        "used through {}, suggest: decide together with {}",
        through.join(", "),
        names.join(", ")
    )
}

// region:    --- Support

/// The old tools that use a feature that Claude cannot call itself, read once from the imports
/// of the old code. The old repo is archived, so this list does not change. `None`: not known.
fn consumers(feature: &Feature) -> Option<&'static [&'static str]> {
    match feature.kind {
        Kind::Template => match feature.name.as_str() {
            "config" => Some(&["config_create"]),
            "gproj" => Some(&["mod"]),
            "layout" => Some(&["layout_create", "layout_recipe"]),
            "localization" => Some(&[]),
            "prefab" => Some(&["prefab"]),
            "scenario" => Some(&["scenario_create"]),
            "script" => Some(&["script_create", "mod"]),
            "server-config" => Some(&["server_config"]),
            _ => None,
        },
        Kind::Recipe if feature.name.starts_with("gamemode/") => Some(&["gamemode_scaffold"]),
        Kind::Recipe if feature.name.starts_with("ui/") => Some(&["layout_recipe"]),
        Kind::Recipe => Some(&["prefab"]),
        Kind::Pattern => Some(&["mod"]),
        Kind::Knowledge => Some(&["wb_knowledge"]),
        Kind::Animation => Some(&["animation_graph"]),
        Kind::Tool | Kind::Prompt | Kind::Resource => None,
    }
}

/// The count key of a `tool_use` block: a tool of the old server, or a read of one of its
/// resources. `None` for every other call.
fn call_key(block: &Value, resources: &[(&str, &str)]) -> Option<String> {
    let name = block
        .get("name")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let input = block.get("input");
    if let Some(tool) = name.strip_prefix(TOOL_PREFIX) {
        return Some(format!("tool:{tool}"));
    }
    let server = input.and_then(|i| i.get("server")).and_then(Value::as_str);
    if name != "ReadMcpResourceTool" || server != Some(OLD_SERVER) {
        return None;
    }
    let uri = input.and_then(|i| i.get("uri")).and_then(Value::as_str)?;
    let (resource, _) = resources.iter().find(|(_, start)| uri.starts_with(start))?;
    Some(format!("resource:{resource}"))
}

/// A JSON value as a map key. A missing value is a key of its own, and a string and a number
/// with the same text are two keys.
fn id_key(value: Option<&Value>) -> Option<String> {
    value.map(Value::to_string)
}

fn count(usage: &mut Usage, key: String, session: &str, time: &str) {
    let entry = usage.counts.entry(key).or_default();
    entry.calls += 1;
    entry.sessions.insert(session.to_owned());
    if *time > *entry.last {
        time.clone_into(&mut entry.last);
    }
}

/// Adds a feature of `kind` for each name, once each, sorted.
fn add(features: &mut Vec<Feature>, kind: Kind, mut names: Vec<String>) {
    names.sort();
    names.dedup();
    features.extend(names.into_iter().map(|name| Feature {
        kind,
        name,
        uri: None,
    }));
}

/// The first group of each match of `pattern` in `text`.
fn first_groups(text: &str, pattern: &'static str) -> Result<Vec<String>> {
    Ok(regex(pattern)?
        .captures_iter(text)
        .filter_map(|found| found.get(1))
        .map(|group| group.as_str().to_owned())
        .collect())
}

fn regex(pattern: &'static str) -> Result<Regex> {
    Regex::new(pattern).map_err(|source| Error::BadRegex { pattern, source })
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "old_project_tests.rs"]
mod tests;

// endregion: --- Tests
