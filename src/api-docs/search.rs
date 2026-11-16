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
mod tests {
    use serde_json::json;

    use crate::test_support::{self, Answer, TestClient};

    #[tokio::test]
    async fn gives_a_class_with_its_parent_classes_and_methods_for_its_full_name() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(
            &client,
            json!({ "query": "SCR_CharacterCameraHandlerComponent" }),
        )
        .await;

        // -- Check
        // From the class page in the Arma Reforger script API docs.
        assert!(!answer.is_error);
        assert!(
            answer
                .text
                .starts_with("## SCR_CharacterCameraHandlerComponent\n"),
            "{}",
            answer.text
        );
        assert!(answer.text.contains(
            "Inherits from: CameraHandlerComponent > ScriptComponent > GenericComponent"
        ));
        assert!(answer.text.contains("- override void OnCameraActivate ()"));
        assert!(answer.text.contains("- bool CanUseOverlayCameraFOV ()"));
        assert!(
            answer
                .text
                .contains("- static void SetOverlayCameraFOV (float fov)")
        );
    }

    #[tokio::test]
    async fn finds_the_expected_class_or_method_in_the_first_10_results_for_real_searches() {
        // -- Setup & Fixtures
        // Searches that Claude sent to the old server, with the class page ("## Name") or the
        // result ("Class.Method (method)") that answers them in classes.jsonl.
        let cases: [(&str, Option<&str>, &[&str]); 12] = [
            ("SCR_BaseGameMode", None, &["## SCR_BaseGameMode\n"]),
            (
                "SCR_GroupsManagerComponent",
                None,
                &["## SCR_GroupsManagerComponent\n"],
            ),
            ("WorkshopApi", Some("class"), &["## WorkshopApi\n"]),
            (
                "GetGroupSize",
                None,
                &["SCR_GroupPreset.GetGroupSize (method)"],
            ),
            (
                "InsertStaticMarker",
                Some("method"),
                &["1. SCR_MapMarkerManagerComponent.InsertStaticMarker (method)"],
            ),
            (
                "GetPredefinedGroups",
                None,
                &["SCR_Faction.GetPredefinedGroups (method)"],
            ),
            (
                "IsFocusMode",
                None,
                &["CharacterControllerComponent.IsFocusMode (method)"],
            ),
            (
                "SetFrequency",
                Some("method"),
                &["BaseTransceiver.SetFrequency (method)"],
            ),
            (
                "SetWeaponADSInput GetWeaponADSInput",
                None,
                &[
                    "CharacterControllerComponent.SetWeaponADSInput (method)",
                    "CharacterControllerComponent.GetWeaponADSInput (method)",
                ],
            ),
            (
                "GetFactionLimitMap",
                None,
                &["1. SCR_MissionHeader.GetFactionLimitMap (method)"],
            ),
            (
                "CharacterFocus",
                None,
                &["SCR_CharacterFocusModeCondition (class)"],
            ),
            (
                "PlayerLimit MaxPlayers",
                None,
                &[
                    "SCR_Faction.GetPlayerLimit (method)",
                    "ServerConfig.GetMaxPlayers (method)",
                ],
            ),
        ];
        let client = connect().await;

        // -- Exec
        let mut misses = Vec::new();
        for (query, kind, expected) in cases {
            let mut arguments = json!({ "query": query });
            if let Some(kind) = kind {
                arguments["type"] = json!(kind);
            }
            let answer = search(&client, arguments).await;
            for part in expected {
                if !answer.text.contains(part) {
                    misses.push(format!("{query}: expected {part} in\n{}", answer.text));
                }
            }
        }

        // -- Check
        assert!(misses.is_empty(), "{}", misses.join("\n\n"));
    }

    #[tokio::test]
    async fn gives_a_method_with_the_class_that_owns_it_for_its_full_name() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "GetFocusFOVScale" })).await;

        // -- Check
        // From the PlayerCamera page in the Enfusion script API docs.
        assert!(!answer.is_error);
        assert!(
            answer
                .text
                .starts_with("1 class or method matches \"GetFocusFOVScale\".\n"),
            "{}",
            answer.text
        );
        assert!(
            answer.text.contains(
                "1. PlayerCamera.GetFocusFOVScale (method)\n   proto external float GetFocusFOVScale (float baseFOV, float scalar)\n"
            ),
            "{}",
            answer.text
        );
    }

    #[tokio::test]
    async fn gives_every_class_whose_name_has_a_part_of_a_name_with_the_names_that_start_with_it_first()
     {
        // -- Setup & Fixtures
        // The 28 class names in classes.jsonl that have "CharacterCamera" in them.
        let starts_with = [
            "CharacterCamera1stPerson",
            "CharacterCamera1stPersonBoneTransform",
            "CharacterCamera1stPersonFloat",
            "CharacterCamera1stPersonReady",
            "CharacterCamera1stPersonTurret",
            "CharacterCamera1stPersonTurretTransition",
            "CharacterCamera1stPersonUnconscious",
            "CharacterCamera1stPersonVehicle",
            "CharacterCamera1stPersonVehicleTransition",
            "CharacterCamera3rdPersonBase",
            "CharacterCamera3rdPersonClimb",
            "CharacterCamera3rdPersonCrouch",
            "CharacterCamera3rdPersonErc",
            "CharacterCamera3rdPersonFloat",
            "CharacterCamera3rdPersonProne",
            "CharacterCamera3rdPersonSitting",
            "CharacterCamera3rdPersonTurret",
            "CharacterCamera3rdPersonUnconscious",
            "CharacterCamera3rdPersonVehicle",
            "CharacterCameraADS",
            "CharacterCameraADSVehicle",
            "CharacterCameraBase",
            "CharacterCameraBinoculars",
            "CharacterCameraDebug",
            "CharacterCameraSet",
        ];
        let has_it_inside = [
            "SCR_CharacterCameraHandlerComponent",
            "SCR_CharacterCameraHandlerComponentClass",
            "SCR_CharacterCameraHandlerComponentSerializer",
        ];
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "CharacterCamera", "limit": 50 })).await;

        // -- Check
        let classes: Vec<String> = results(&answer)
            .into_iter()
            .filter(|(_, kind)| kind == "class")
            .map(|(name, _)| name)
            .collect();
        let (first, last) = classes.split_at(starts_with.len().min(classes.len()));
        let mut first = first.to_vec();
        first.sort();
        let mut last = last.to_vec();
        last.sort();
        assert_eq!(first, starts_with, "{}", answer.text);
        assert_eq!(last, has_it_inside);
        assert!(
            answer
                .text
                .contains("CharacterCameraBase (class)\n   Inherits from: ScriptedCameraItem\n"),
            "{}",
            answer.text
        );
    }

    #[tokio::test]
    async fn cuts_a_long_list_at_the_limit_and_says_how_to_narrow_the_search() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "CharacterCamera" })).await;

        // -- Check
        // Counted in classes.jsonl: 28 class names and 22 method names have "charactercamera" in them.
        assert_eq!(results(&answer).len(), 10);
        assert!(
            answer.text.starts_with(
                "50 classes and methods match \"CharacterCamera\". This list shows the first 10. \
             To narrow the search, give a longer name, or set `type` to \"class\" or \"method\". \
             To see more, set `limit` up to 50.\n"
            ),
            "{}",
            answer.text
        );
    }

    #[tokio::test]
    async fn gives_only_classes_or_only_methods_for_that_type() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let classes = search(
            &client,
            json!({ "query": "CharacterCamera", "type": "class", "limit": 50 }),
        )
        .await;
        let methods = search(
            &client,
            json!({ "query": "CharacterCamera", "type": "method", "limit": 50 }),
        )
        .await;

        // -- Check
        let kinds = |answer: &Answer| -> Vec<String> {
            results(answer).into_iter().map(|(_, kind)| kind).collect()
        };
        assert_eq!(kinds(&classes), vec!["class"; 28]);
        assert_eq!(kinds(&methods), vec!["method"; 22]);
    }

    #[tokio::test]
    async fn gives_the_names_that_are_spelled_almost_the_same_when_no_name_matches() {
        // -- Setup & Fixtures
        // The typos of the old TypeScript tests.
        let cases = [
            ("ScriptCompnent", "ScriptComponent", "class"),
            ("GetPositon", "GetPosition", "method"),
        ];
        let client = connect().await;

        // -- Exec & Check
        for (query, expected, kind) in cases {
            let answer = search(&client, json!({ "query": query })).await;
            assert!(!answer.is_error);
            assert!(
                answer.text.starts_with(&format!(
                    "No class or method has \"{query}\" in its name. These names are spelled almost the same."
                )),
                "{}",
                answer.text
            );
            let found = results(&answer).into_iter().any(|(name, found_kind)| {
                found_kind == kind && name.rsplit('.').next() == Some(expected)
            });
            assert!(found, "{query}: no {kind} {expected} in {}", answer.text);
        }
    }

    #[tokio::test]
    async fn says_what_to_try_next_when_no_name_matches_or_is_spelled_almost_the_same() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "xyz123nonexistent" })).await;

        // -- Check
        assert!(!answer.is_error);
        assert_eq!(
            answer.text,
            "No class or method has \"xyz123nonexistent\" in its name, and no name is spelled almost \
             the same. Give a shorter part of the name, for example \"Camera\" for \
             \"CharacterCameraBase\". To search the wiki by words, use wiki_search."
        );
    }

    #[tokio::test]
    async fn asks_for_a_name_when_the_query_is_empty() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = search(&client, json!({ "query": "  " })).await;

        // -- Check
        assert!(answer.is_error);
        assert_eq!(
            answer.text,
            "The query is empty. Give the name or a part of the name of a class or a method, \
             for example \"SCR_BaseGameMode\" or \"GetFocusFOVScale\"."
        );
    }

    // -- Test Support

    async fn connect() -> TestClient {
        test_support::connect(crate::api_docs::tools().unwrap()).await
    }

    async fn search(client: &TestClient, arguments: serde_json::Value) -> Answer {
        test_support::call(client, "api_search", arguments).await
    }

    /// The results in the answer, in order: (name, kind) from lines like
    /// "3. PlayerCamera.GetFocusFOVScale (method)".
    fn results(answer: &Answer) -> Vec<(String, String)> {
        answer
            .text
            .lines()
            .filter_map(|line| line.split_once(". "))
            .filter(|(number, _)| number.parse::<usize>().is_ok())
            .filter_map(|(_, rest)| rest.strip_suffix(')')?.split_once(" ("))
            .map(|(name, kind)| (name.to_owned(), kind.to_owned()))
            .collect()
    }
}

// endregion: --- Tests
