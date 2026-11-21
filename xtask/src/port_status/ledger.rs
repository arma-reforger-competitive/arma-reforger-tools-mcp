//! The Port ledger (`docs/port-ledger.md`): its rows, how they are read and written, and the
//! changes that `report` and `set` make.

use std::collections::{HashMap, HashSet};
use std::fmt;
use std::path::Path;

use super::files::{files_under, read_text, relative};
use super::old_project::{Feature, Usage, suggest};
use super::{Error, Result, collate};

/// The kind of a feature of the old project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    Tool,
    Prompt,
    Resource,
    Template,
    Recipe,
    Pattern,
    Knowledge,
    Animation,
}

/// Every kind, with its name in a selector and its ledger heading, in the order of the ledger.
pub const KINDS: [(Kind, &str, &str); 8] = [
    (Kind::Tool, "tool", "Tools"),
    (Kind::Prompt, "prompt", "Prompts"),
    (Kind::Resource, "resource", "Resources"),
    (Kind::Template, "template", "Templates"),
    (Kind::Recipe, "recipe", "Recipes"),
    (Kind::Pattern, "pattern", "Patterns"),
    (Kind::Knowledge, "knowledge", "Knowledge pages"),
    (Kind::Animation, "animation", "Animation graph code"),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Ported,
    NotNeeded,
    NotYet,
}

/// One row of the ledger.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row {
    pub kind: Kind,
    pub name: String,
    pub status: Status,
    pub detail: String,
    pub suggestion: String,
}

/// A "ported" row whose link does not lead to a Tool or skill of this repo.
#[derive(Debug, Clone)]
pub struct Problem {
    pub row: Row,
    pub problem: String,
}

/// The result of `sync`.
pub struct Synced {
    pub rows: Vec<Row>,
    /// The features that the ledger did not list.
    pub added: Vec<Feature>,
    /// The rows whose feature is not in the old project.
    pub stale: Vec<Row>,
}

/// The result of `set_status`.
pub struct Changed {
    pub rows: Vec<Row>,
    /// The rows that the selectors named, in the order in which the selectors found them.
    pub changed: Vec<Row>,
}

const STATUSES: [Status; 3] = [Status::Ported, Status::NotNeeded, Status::NotYet];

/// Reads the rows of a ledger.
pub fn parse_ledger(text: &str) -> Result<Vec<Row>> {
    let mut rows = Vec::new();
    let mut kind = None;
    for (index, line) in text.split('\n').enumerate() {
        let number = index + 1;
        if let Some(heading) = heading(line) {
            kind = KINDS
                .iter()
                .find(|(_, _, text)| *text == heading.trim())
                .map(|(kind, _, _)| *kind);
        }
        if !line.starts_with("| `") {
            continue;
        }
        let cells = cells(line);
        let Some(kind) = kind else {
            return Err(Error::NoKindHeading { line: number });
        };
        let [name, status, detail, suggestion] = cells.as_slice() else {
            return Err(Error::NotFourCells { line: number });
        };
        let Some(status) = STATUSES.into_iter().find(|s| s.to_string() == *status) else {
            return Err(Error::BadRowStatus {
                line: number,
                status: status.clone(),
            });
        };
        rows.push(Row {
            kind,
            name: name.replace('`', ""),
            status,
            detail: detail.clone(),
            suggestion: suggestion.clone(),
        });
    }
    Ok(rows)
}

/// The history line of the ledger and of the report.
pub fn history_line(usage: &Usage) -> String {
    let day = |time: &str| match time.chars().take(10).collect::<String>() {
        day if day.is_empty() => "-".to_owned(),
        day => day,
    };
    format!(
        "History: {} sessions with the old server, from {} to {}.",
        usage.sessions.len(),
        day(&usage.first),
        day(&usage.last)
    )
}

/// The text of the ledger, with `history` as its history line.
pub fn render_ledger(rows: &[Row], history: &str) -> String {
    let cell = |text: &str| text.replace('|', "\\|");
    let mut lines = vec![
        "# Port ledger".to_owned(),
        String::new(),
        "Every feature of the old enfusion-mcp project, and what happens to it in this repo. Run `/port-status` to update this file.".to_owned(),
        String::new(),
        "- **Status**: `ported`, `not needed`, or `not yet`.".to_owned(),
        "- **Detail**: for `ported`, a link to the new Tool or skill. For `not needed`, the reason. For `not yet`, the port ticket, if one exists.".to_owned(),
        "- **Suggestion**: what the use of the old server in Claude Code session history suggests. Only the owner decides.".to_owned(),
        String::new(),
        history.to_owned(),
    ];
    for (kind, _, heading) in KINDS {
        let of_kind: Vec<&Row> = rows.iter().filter(|row| row.kind == kind).collect();
        if of_kind.is_empty() {
            continue;
        }
        lines.push(String::new());
        lines.push(format!("## {heading}"));
        lines.push(String::new());
        lines.push("| Feature | Status | Detail | Suggestion |".to_owned());
        lines.push("| --- | --- | --- | --- |".to_owned());
        for row in of_kind {
            lines.push(format!(
                "| `{}` | {} | {} | {} |",
                row.name,
                row.status,
                cell(&row.detail),
                cell(&row.suggestion)
            ));
        }
    }
    format!("{}\n", lines.join("\n"))
}

/// Adds the features that the ledger does not list yet, and gives every row a new suggestion.
pub fn sync(rows: Vec<Row>, features: &[Feature], usage: &Usage) -> Synced {
    let key = |kind: Kind, name: &str| format!("{}:{name}", kind_id(kind));
    let known: HashMap<String, &Feature> = features
        .iter()
        .map(|feature| (key(feature.kind, &feature.name), feature))
        .collect();
    let listed: HashSet<String> = rows.iter().map(|row| key(row.kind, &row.name)).collect();
    let added: Vec<Feature> = features
        .iter()
        .filter(|feature| !listed.contains(&key(feature.kind, &feature.name)))
        .cloned()
        .collect();
    let stale: Vec<Row> = rows
        .iter()
        .filter(|row| !known.contains_key(&key(row.kind, &row.name)))
        .cloned()
        .collect();
    let new_rows = added.iter().map(|feature| Row {
        kind: feature.kind,
        name: feature.name.clone(),
        status: Status::NotYet,
        detail: String::new(),
        suggestion: String::new(),
    });
    let mut synced: Vec<Row> = rows
        .into_iter()
        .chain(new_rows)
        .map(|row| {
            let suggestion = match known.get(&key(row.kind, &row.name)) {
                Some(feature) => suggest(feature, usage),
                None => "not in the old project".to_owned(),
            };
            Row { suggestion, ..row }
        })
        .collect();
    synced.sort_by(|a, b| {
        kind_order(a.kind)
            .cmp(&kind_order(b.kind))
            .then_with(|| collate::compare(&a.name, &b.name))
    });
    Synced {
        rows: synced,
        added,
        stale,
    }
}

/// Gives every row that a selector names the same status and detail. A selector is a name, a
/// glob such as "*_create", or either of them after a kind, such as "template:prefab".
pub fn set_status(
    rows: &[Row],
    selectors: &[String],
    status: Status,
    detail: &str,
) -> Result<Changed> {
    if status == Status::NotNeeded && detail.is_empty() {
        return Err(Error::NoReason);
    }
    if status == Status::Ported && detail.is_empty() {
        return Err(Error::NoLink);
    }
    let mut chosen: Vec<usize> = Vec::new();
    for selector in selectors {
        // Only the first two parts count: "a:b:c" is the kind "a" and the name "b".
        let (kind, pattern) = match selector.split_once(':') {
            Some((kind, rest)) => (kind, rest.split(':').next().unwrap_or_default()),
            None => ("", selector.as_str()),
        };
        let wanted = if kind.is_empty() {
            None
        } else {
            let found = KINDS.iter().find(|(_, id, _)| *id == kind);
            let Some((wanted, _, _)) = found else {
                return Err(Error::BadKind {
                    kind: kind.to_owned(),
                });
            };
            Some(*wanted)
        };
        let found: Vec<usize> = rows
            .iter()
            .enumerate()
            .filter(|(_, row)| wanted.is_none_or(|k| row.kind == k) && glob(pattern, &row.name))
            .map(|(index, _)| index)
            .collect();
        if found.is_empty() {
            return Err(Error::NoMatch {
                selector: selector.clone(),
            });
        }
        let mut kinds: Vec<Kind> = Vec::new();
        for row in found.iter().filter_map(|&index| rows.get(index)) {
            if !kinds.contains(&row.kind) {
                kinds.push(row.kind);
            }
        }
        if kinds.len() > 1 {
            return Err(Error::ManyKinds {
                selector: selector.clone(),
                choices: kinds
                    .iter()
                    .map(|&k| format!("{}:{pattern}", kind_id(k)))
                    .collect(),
            });
        }
        for index in found {
            if !chosen.contains(&index) {
                chosen.push(index);
            }
        }
    }
    let change = |row: &Row| Row {
        status,
        detail: detail.to_owned(),
        ..row.clone()
    };
    let updated = rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            if chosen.contains(&index) {
                change(row)
            } else {
                row.clone()
            }
        })
        .collect();
    let changed = chosen
        .iter()
        .filter_map(|&index| rows.get(index))
        .map(change)
        .collect();
    Ok(Changed {
        rows: updated,
        changed,
    })
}

/// The kinds of link that `link_to` makes.
#[derive(Debug, Clone, Copy)]
pub enum LinkKind {
    Tool,
    Skill,
}

/// Makes the ledger link for a Tool or skill of this repo. It fails when the repo does not have
/// it.
pub fn link_to(new_repo: &Path, kind: LinkKind, name: &str) -> Result<String> {
    match kind {
        LinkKind::Skill => {
            let file = format!("skills/{name}/SKILL.md");
            if !new_repo.join(&file).exists() {
                return Err(Error::NoSkill {
                    name: name.to_owned(),
                    file,
                });
            }
            Ok(format!("[skill {name}](../{file})"))
        }
        LinkKind::Tool => {
            let files = files_under(&new_repo.join("src"), ".rs")?;
            let Some(file) = files.iter().find(|file| declares_tool(file, name)) else {
                return Err(Error::NoTool {
                    name: name.to_owned(),
                });
            };
            Ok(format!("[tool {name}](../{})", relative(new_repo, file)))
        }
    }
}

/// Finds the "ported" rows whose link does not lead to a Tool or skill in this repo.
pub fn check_ported(rows: &[Row], new_repo: &Path) -> Vec<Problem> {
    let mut problems = Vec::new();
    for row in rows.iter().filter(|row| row.status == Status::Ported) {
        let mut report = |problem: String| {
            problems.push(Problem {
                row: row.clone(),
                problem,
            });
        };
        let Some((kind, name, file)) = parse_link(&row.detail) else {
            report("has no link to a Tool or skill".to_owned());
            continue;
        };
        let path = new_repo.join(file);
        if !path.exists() {
            report(format!("links to {file}, which does not exist"));
        } else if kind == "tool" && !declares_tool(&path, name) {
            report(format!(
                "links to Tool {name}, but {file} does not declare it"
            ));
        }
    }
    problems
}

/// The last line of a report: done, or the features that are still open.
pub fn verdict(rows: &[Row], problems: &[Problem]) -> String {
    let open: Vec<&Row> = rows
        .iter()
        .filter(|row| {
            row.status == Status::NotYet || problems.iter().any(|problem| problem.row == **row)
        })
        .collect();
    if open.is_empty() {
        return "Port done: nothing left in the old repo.".to_owned();
    }
    let mut lines = vec![format!("Not done: {} features still open.", open.len())];
    for (kind, _, heading) in KINDS {
        let names: Vec<&str> = open
            .iter()
            .filter(|row| row.kind == kind)
            .map(|row| row.name.as_str())
            .collect();
        if !names.is_empty() {
            lines.push(format!("- {heading}: {}", names.join(", ")));
        }
    }
    lines.join("\n")
}

/// The name of a kind in a selector and in the report, for example "tool".
pub fn kind_id(kind: Kind) -> &'static str {
    KINDS
        .iter()
        .find(|(k, _, _)| *k == kind)
        .map_or("", |(_, id, _)| id)
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Ported => "ported",
            Self::NotNeeded => "not needed",
            Self::NotYet => "not yet",
        })
    }
}

// region:    --- Support

fn kind_order(kind: Kind) -> usize {
    KINDS.iter().position(|(k, _, _)| *k == kind).unwrap_or(0)
}

/// The text of a `## ` heading line. A line with a line break inside is not a heading.
fn heading(line: &str) -> Option<&str> {
    line.strip_prefix("## ")
        .filter(|text| !text.is_empty() && !text.contains(['\r', '\u{2028}', '\u{2029}']))
}

/// The cells of a table row: the parts between the `|` signs that have no `\` before them,
/// without the part before the first sign and after the last sign.
fn cells(line: &str) -> Vec<String> {
    let mut parts = vec![String::new()];
    let mut previous = None;
    for c in line.chars() {
        if c == '|' && previous != Some('\\') {
            parts.push(String::new());
        } else if let Some(part) = parts.last_mut() {
            part.push(c);
        }
        previous = Some(c);
    }
    let inner = parts.get(1..parts.len().saturating_sub(1)).unwrap_or(&[]);
    inner
        .iter()
        .map(|cell| cell.trim().replace("\\|", "|"))
        .collect()
}

/// True when `name` matches `pattern`, in which `*` stands for any text.
fn glob(pattern: &str, name: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let name: Vec<char> = name.chars().collect();
    let (mut p, mut n) = (0, 0);
    // The last `*` and the name position that it covers up to.
    let mut star: Option<(usize, usize)> = None;
    while n < name.len() {
        match (pattern.get(p), name.get(n)) {
            (Some('*'), _) => {
                star = Some((p, n));
                p += 1;
            }
            (Some(a), Some(b)) if a == b => {
                p += 1;
                n += 1;
            }
            _ => match star {
                Some((star_p, star_n)) => {
                    p = star_p + 1;
                    n = star_n + 1;
                    star = Some((star_p, star_n + 1));
                }
                None => return false,
            },
        }
    }
    pattern
        .get(p..)
        .is_some_and(|rest| rest.iter().all(|&c| c == '*'))
}

/// The kind, name, and file of a link `[tool <name>](../<file>)` or `[skill <name>](../<file>)`.
fn parse_link(detail: &str) -> Option<(&str, &str, &str)> {
    let rest = detail.strip_prefix('[')?;
    let (kind, rest) = rest.split_once(' ')?;
    if kind != "tool" && kind != "skill" {
        return None;
    }
    let (name, rest) = rest.split_once(']')?;
    let file = rest.strip_prefix("(../")?.strip_suffix(')')?;
    if name.is_empty() || file.is_empty() || file.contains(')') {
        return None;
    }
    Some((kind, name, file))
}

/// True when `file` calls `Tool::new("<name>"`. A file that cannot be read, for example a folder,
/// does not declare it.
fn declares_tool(file: &Path, name: &str) -> bool {
    let Ok(text) = read_text(file) else {
        return false;
    };
    let quoted = format!("\"{name}\"");
    text.match_indices("Tool::new(").any(|(index, call)| {
        text.get(index + call.len()..)
            .is_some_and(|rest| rest.trim_start().starts_with(&quoted))
    })
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "ledger_tests.rs"]
mod tests;

// endregion: --- Tests
