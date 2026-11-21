//! The `api_search` Tool: finds classes and methods by their name.

use std::fmt::Write as _;

use rmcp::model::{CallToolResult, ContentBlock};
use schemars::JsonSchema;
use serde::Deserialize;

use super::classes::{Class, Classes};

const DEFAULT_LIMIT: usize = 10;
const MAX_LIMIT: usize = 50;

#[derive(Deserialize, JsonSchema)]
pub(super) struct SearchInput {
    /// The full name or a part of the name of a class or a method, for example `SCR_BaseGameMode`,
    /// `GetFocusFOVScale`, or `CharacterCamera`. Give more names with spaces between them.
    #[schemars(length(min = 1))]
    query: String,

    /// How many results to list, from 1 to 50, for example 20. The default is 10. A class page for
    /// a full class name does not count.
    #[schemars(range(min = 1, max = 50))]
    limit: Option<usize>,

    /// Search only classes or only methods: "class" or "method". Leave it out to search both.
    #[serde(rename = "type")]
    kind: Option<Kind>,
}

/// Classes come before methods in a list.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Deserialize, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub(super) enum Kind {
    Class,
    Method,
}

/// The name of every class and every method, in lowercase, ready for search.
pub(super) struct Index {
    entries: Vec<Entry>,
}

/// Answers one `api_search` call.
pub(super) fn search(classes: &Classes, index: &Index, input: &SearchInput) -> CallToolResult {
    let names: Vec<String> = input
        .query
        .split_whitespace()
        .map(str::to_lowercase)
        .collect();
    if names.is_empty() {
        let text = "The query is empty. Give the name or a part of the name of a class or a method, \
                    for example \"SCR_BaseGameMode\" or \"GetFocusFOVScale\".";
        return CallToolResult::error(vec![ContentBlock::text(text)]);
    }
    let limit = input.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let entries = index
        .entries
        .iter()
        .filter(|entry| input.kind.is_none_or(|kind| kind == entry.kind()));
    let mut pages = String::new();
    let mut hits: Vec<Hit> = Vec::new();
    for entry in entries.clone() {
        let Some(score) = names.iter().filter_map(|name| entry.score(name)).max() else {
            continue;
        };
        if score == Score::FullName
            && entry.target == Target::Class
            && let Some(class) = classes.all().get(entry.class)
        {
            pages.push_str(&class_page(classes, class));
            continue;
        }
        hits.push(Hit { entry, score });
    }
    let found_names = !pages.is_empty() || !hits.is_empty();
    if !found_names {
        hits = entries
            .filter_map(|entry| {
                let distance = names
                    .iter()
                    .filter_map(|name| entry.typo_distance(name))
                    .min()?;
                Some(Hit {
                    entry,
                    score: Score::Typo(std::cmp::Reverse(distance)),
                })
            })
            .collect();
    }
    hits.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(a.entry.kind().cmp(&b.entry.kind()))
            .then(a.entry.key.len().cmp(&b.entry.key.len()))
            .then(a.entry.key.cmp(&b.entry.key))
            .then(a.entry.class.cmp(&b.entry.class))
    });
    let mut text = pages;
    if !text.is_empty() && !hits.is_empty() {
        text.push('\n');
    }
    let query = input.query.trim();
    let (one, many) = match input.kind {
        Some(Kind::Class) => ("class", "classes"),
        Some(Kind::Method) => ("method", "methods"),
        None => ("class or method", "classes and methods"),
    };
    let not_found = format!("No {one} has \"{query}\" in its name");
    if !found_names && hits.is_empty() {
        let text = format!(
            "{not_found}, and no name is spelled almost the same. Give a shorter part of the name, \
             for example \"Camera\" for \"CharacterCameraBase\". To search the wiki by words, use \
             wiki_search."
        );
        return CallToolResult::success(vec![ContentBlock::text(text)]);
    }
    if !hits.is_empty() {
        let intro = if found_names {
            let more = if text.is_empty() { "" } else { "more " };
            match hits.len() {
                1 => format!("1 {more}{one} matches \"{query}\"."),
                count => format!("{count} {more}{many} match \"{query}\"."),
            }
        } else {
            format!("{not_found}. These names are spelled almost the same.")
        };
        text.push_str(&list_header(&intro, input, hits.len(), limit));
    }
    for (number, hit) in hits.iter().take(limit).enumerate() {
        let _ = write!(text, "\n{}. {}", number + 1, hit.entry.line(classes));
    }
    CallToolResult::success(vec![ContentBlock::text(text)])
}

impl Index {
    pub(super) fn new(classes: &Classes) -> Self {
        let mut entries = Vec::new();
        for (class_index, class) in classes.all().iter().enumerate() {
            entries.push(Entry {
                key: class.name.to_lowercase(),
                class: class_index,
                target: Target::Class,
            });
            for (group, (_, methods)) in class.method_groups().iter().enumerate() {
                for (method_index, method) in methods.iter().enumerate() {
                    entries.push(Entry {
                        key: method.name.to_lowercase(),
                        class: class_index,
                        target: Target::Method {
                            group,
                            index: method_index,
                        },
                    });
                }
            }
        }
        Self { entries }
    }
}

// region:    --- Support

/// The first lines of a list: `intro`, then how to narrow the search when the list is cut.
fn list_header(intro: &str, input: &SearchInput, total: usize, limit: usize) -> String {
    let mut text = intro.to_owned();
    if total > limit {
        let _ = write!(
            text,
            " This list shows the first {limit}. To narrow the search, give a longer name"
        );
        if input.kind.is_none() {
            text.push_str(", or set `type` to \"class\" or \"method\"");
        }
        text.push('.');
        if limit < MAX_LIMIT {
            let _ = write!(text, " To see more, set `limit` up to {MAX_LIMIT}.");
        }
    }
    text.push('\n');
    text
}

/// How many letters a name may differ from a name of the query and still count as a typo.
const MAX_TYPO_DISTANCE: usize = 2;

/// How well a name matches a name of the query, the best last.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    /// Spelled almost the same: the number of letters to change, add, or remove. Fewer is better.
    Typo(std::cmp::Reverse<usize>),
    PartOfName,
    StartOfName,
    FullName,
}

struct Hit<'a> {
    entry: &'a Entry,
    score: Score,
}

/// One class or one method of a class, with its name in lowercase.
struct Entry {
    key: String,

    /// The position in `Classes::all`.
    class: usize,
    target: Target,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Target {
    Class,

    /// The position in `Class::method_groups`: the group, then the method in it.
    Method {
        group: usize,
        index: usize,
    },
}

impl Entry {
    /// The Levenshtein distance to `name`, when it is at most `MAX_TYPO_DISTANCE`.
    fn typo_distance(&self, name: &str) -> Option<usize> {
        // Most names differ in length by more than the limit, and a count is cheaper than the
        // distance.
        if self.key.chars().count().abs_diff(name.chars().count()) > MAX_TYPO_DISTANCE {
            return None;
        }
        Some(strsim::levenshtein(&self.key, name)).filter(|distance| *distance <= MAX_TYPO_DISTANCE)
    }

    fn kind(&self) -> Kind {
        match self.target {
            Target::Class => Kind::Class,
            Target::Method { .. } => Kind::Method,
        }
    }

    /// `name` is a name of the query in lowercase.
    fn score(&self, name: &str) -> Option<Score> {
        if self.key == name {
            Some(Score::FullName)
        } else if self.key.starts_with(name) {
            Some(Score::StartOfName)
        } else if self.key.contains(name) {
            Some(Score::PartOfName)
        } else {
            None
        }
    }

    /// One result in a list: the name and kind, then the signature or the parent, then the
    /// summary.
    fn line(&self, classes: &Classes) -> String {
        let Some(class) = classes.all().get(self.class) else {
            return String::new();
        };
        if let Target::Method { group, index } = self.target
            && let Some((_, methods)) = class.method_groups().get(group)
            && let Some(method) = methods.get(index)
        {
            let mut text = format!(
                "{}.{} (method)\n   {}\n",
                class.name, method.name, method.signature
            );
            if let Some(summary) = summary(&method.description) {
                let _ = writeln!(text, "   {summary}");
            }
            return text;
        }
        let mut text = format!("{} (class)\n", class.name);
        if let Some(parent) = class.parents.first() {
            let _ = writeln!(text, "   Inherits from: {parent}");
        }
        if !class.brief.is_empty() {
            let _ = writeln!(text, "   {}", class.brief);
        }
        text
    }
}

/// How many subclasses a class page names.
const MAX_CHILDREN: usize = 10;

/// All that the docs say about one class: where it comes from, the classes it inherits from, its
/// methods, and its properties.
fn class_page(classes: &Classes, class: &Class) -> String {
    let mut text = format!("## {}\n{} class", class.name, class.source.label());
    if !class.group.is_empty() {
        let _ = write!(text, ", group {}", class.group);
    }
    text.push_str(".\n");
    let ancestors = classes.ancestors(class);
    if !ancestors.is_empty() {
        let _ = writeln!(text, "Inherits from: {}", ancestors.join(" > "));
    }
    if !class.children.is_empty() {
        let shown = class.children.iter().take(MAX_CHILDREN).map(String::as_str);
        let _ = write!(text, "Subclasses: {}", shown.collect::<Vec<_>>().join(", "));
        if class.children.len() > MAX_CHILDREN {
            let _ = write!(text, ", and {} more", class.children.len() - MAX_CHILDREN);
        }
        text.push('\n');
    }
    if !class.brief.is_empty() {
        let _ = write!(text, "\n{}\n", class.brief);
    }
    if !class.description.is_empty() && class.description != class.brief {
        let _ = write!(text, "\n{}\n", class.description);
    }
    for (heading, methods) in class.method_groups() {
        if methods.is_empty() {
            continue;
        }
        let _ = write!(text, "\n### {heading} ({})\n", methods.len());
        for method in methods {
            let _ = writeln!(
                text,
                "- {}{}",
                method.signature,
                after_dash(&method.description)
            );
        }
    }
    for (heading, properties) in class.property_groups() {
        if properties.is_empty() {
            continue;
        }
        let _ = write!(text, "\n### {heading} ({})\n", properties.len());
        for property in properties {
            let _ = writeln!(
                text,
                "- {} {}{}",
                property.kind,
                property.name,
                after_dash(&property.description)
            );
        }
    }
    let _ = write!(
        text,
        "\nSource file: {}\nDocs: {}\n",
        class.source_file, class.docs_url
    );
    text
}

/// The first paragraph of a description. A table of parameters or a note on the return value is
/// left out: the signature has the parameters.
fn summary(description: &str) -> Option<String> {
    let first = description.split("\n\n").next()?.trim();
    if first.is_empty() || first.starts_with("**") {
        return None;
    }
    Some(first.split_whitespace().collect::<Vec<_>>().join(" "))
}

/// The summary of a description after " -- ", for the end of a line in a class page.
fn after_dash(description: &str) -> String {
    summary(description).map_or_else(String::new, |summary| format!(" -- {summary}"))
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "search_tests.rs"]
mod tests;

// endregion: --- Tests
