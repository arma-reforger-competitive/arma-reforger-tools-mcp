use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::*;

#[test]
fn sync_adds_missing_features_as_not_yet_keeps_decisions_and_gives_every_row_a_suggestion() {
    // -- Setup & Fixtures
    let features = [
        feature(Kind::Tool, "api_search"),
        feature(Kind::Tool, "mod"),
        feature(Kind::Pattern, "game-mode"),
    ];
    let rows = vec![row(
        "mod",
        Status::NotNeeded,
        "Claude writes it",
        Kind::Tool,
    )];

    // -- Exec
    let synced = sync(rows, &features, &four_sessions());

    // -- Check
    let added: Vec<&str> = synced.added.iter().map(|f| f.name.as_str()).collect();
    assert_eq!(added, ["api_search", "game-mode"]);
    let rows: Vec<(&str, Status, &str, &str)> = synced
        .rows
        .iter()
        .map(|r| {
            (
                r.name.as_str(),
                r.status,
                r.detail.as_str(),
                r.suggestion.as_str(),
            )
        })
        .collect();
    assert_eq!(
        rows,
        [
            (
                "api_search",
                Status::NotYet,
                "",
                "never used in 4 sessions, suggest: not needed"
            ),
            (
                "mod",
                Status::NotNeeded,
                "Claude writes it",
                "never used in 4 sessions, suggest: not needed"
            ),
            (
                "game-mode",
                Status::NotYet,
                "",
                "its tools (mod) were never used in 4 sessions, suggest: not needed"
            ),
        ]
    );
}

#[test]
fn sync_sorts_names_like_the_javascript_locale_compare() {
    // -- Setup & Fixtures
    // The order that Node 26 gives with `localeCompare` (ICU root order).
    let expected = [
        "a",
        "a b",
        "a_b",
        "a-b",
        "a.b",
        "a/b",
        "a1",
        "ab",
        "Ab",
        "AB",
        "core-concepts",
        "INDEX",
        "z~",
        "z$",
        "z9",
    ];
    let mut features: Vec<Feature> = expected
        .iter()
        .map(|name| feature(Kind::Knowledge, name))
        .collect();
    features.reverse();

    // -- Exec
    let synced = sync(Vec::new(), &features, &four_sessions());

    // -- Check
    let names: Vec<&str> = synced.rows.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, expected);
}

#[test]
fn a_decision_for_many_rows_updates_every_named_row_and_writes_the_reason() {
    // -- Setup & Fixtures
    let rows = [
        not_yet("config_create"),
        not_yet("layout_create"),
        not_yet("script_create"),
        not_yet("api_search"),
    ];
    let reason = "Claude writes these files itself";

    // -- Exec
    let changed = set_status(&rows, &["*_create".to_owned()], Status::NotNeeded, reason).unwrap();

    // -- Check
    assert_eq!(changed.changed.len(), 3);
    for name in ["config_create", "layout_create", "script_create"] {
        let found = changed.rows.iter().find(|r| r.name == name).unwrap();
        assert_eq!(*found, row(name, Status::NotNeeded, reason, Kind::Tool));
    }
    let other = changed
        .rows
        .iter()
        .find(|r| r.name == "api_search")
        .unwrap();
    assert_eq!(*other, not_yet("api_search"));
}

#[test]
fn a_not_needed_row_can_go_back_to_not_yet() {
    // -- Setup & Fixtures
    let rows = [row(
        "mod",
        Status::NotNeeded,
        "Claude writes it",
        Kind::Tool,
    )];

    // -- Exec
    let changed = set_status(&rows, &["mod".to_owned()], Status::NotYet, "").unwrap();

    // -- Check
    assert_eq!(changed.rows, [not_yet("mod")]);
}

#[test]
fn set_refuses_unknown_names_names_of_two_kinds_and_a_not_needed_row_without_a_reason() {
    // -- Setup & Fixtures
    let rows = [
        not_yet("prefab"),
        row("prefab", Status::NotYet, "", Kind::Template),
    ];
    let error = |selector: &str, status, detail| {
        set_status(&rows, &[selector.to_owned()], status, detail)
            .err()
            .unwrap()
            .to_string()
    };

    // -- Exec & Check
    assert_eq!(
        error("nothing", Status::NotYet, ""),
        "No feature matches \"nothing\"."
    );
    assert_eq!(
        error("prefab", Status::NotYet, ""),
        "\"prefab\" matches features of more than one kind. Write tool:prefab or template:prefab."
    );
    assert_eq!(
        error("tool:prefab", Status::NotNeeded, ""),
        "\"not needed\" needs a reason."
    );
    assert_eq!(
        error("x:prefab", Status::NotYet, ""),
        "\"x\" is not a kind. Use one of: tool, prompt, resource, template, recipe, pattern, \
         knowledge, animation."
    );
    let changed = set_status(
        &rows,
        &["template:prefab".to_owned()],
        Status::NotNeeded,
        "x",
    )
    .unwrap();
    assert_eq!(changed.changed[0].kind, Kind::Template);
}

#[test]
fn set_keeps_the_order_in_which_the_selectors_found_the_rows_once_each() {
    // -- Setup & Fixtures
    let rows = [not_yet("a_x"), not_yet("b_x"), not_yet("c")];
    let selectors = ["c", "*_x", "tool:b_x:more"].map(str::to_owned);

    // -- Exec
    let changed = set_status(&rows, &selectors, Status::NotYet, "").unwrap();

    // -- Check
    let names: Vec<&str> = changed.changed.iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["c", "a_x", "b_x"]);
}

#[test]
fn a_ported_row_whose_tool_or_skill_is_missing_from_the_repo_is_reported() {
    // -- Setup & Fixtures
    let repo = out_dir("a_ported_row_whose_tool_or_skill_is_missing_from_the_repo_is_reported");
    fs::create_dir_all(repo.join("src/wiki")).unwrap();
    fs::write(
        repo.join("src/wiki/read.rs"),
        "let tool = Tool::new(\n    \"wiki_read\",",
    )
    .unwrap();
    let rows = [
        row(
            "wiki_read",
            Status::Ported,
            "[tool wiki_read](../src/wiki/read.rs)",
            Kind::Tool,
        ),
        row(
            "wiki_search",
            Status::Ported,
            "[tool wiki_search](../src/wiki/read.rs)",
            Kind::Tool,
        ),
        row(
            "create-mod",
            Status::Ported,
            "[skill create-mod](../skills/create-mod/SKILL.md)",
            Kind::Prompt,
        ),
        row("mod", Status::Ported, "see the new mod skill", Kind::Tool),
        row(
            "wiki",
            Status::Ported,
            "[tool wiki](../src/wiki)",
            Kind::Tool,
        ),
    ];

    // -- Exec
    let link = link_to(&repo, LinkKind::Tool, "wiki_read").unwrap();
    let missing = link_to(&repo, LinkKind::Tool, "api_search").unwrap_err();
    let problems = check_ported(&rows, &repo);

    // -- Check
    assert_eq!(link, "[tool wiki_read](../src/wiki/read.rs)");
    assert_eq!(missing.to_string(), "No Tool named api_search in src/.");
    let problems: Vec<String> = problems
        .iter()
        .map(|p| format!("{} {}", p.row.name, p.problem))
        .collect();
    assert_eq!(
        problems,
        [
            "wiki_search links to Tool wiki_search, but src/wiki/read.rs does not declare it",
            "create-mod links to skills/create-mod/SKILL.md, which does not exist",
            "mod has no link to a Tool or skill",
            "wiki links to Tool wiki, but src/wiki does not declare it",
        ]
    );
}

#[test]
fn the_verdict_says_the_port_is_done_only_when_no_row_is_open() {
    // -- Setup & Fixtures
    let done = [
        row("wiki_read", Status::Ported, "link", Kind::Tool),
        row("mod", Status::NotNeeded, "x", Kind::Tool),
    ];
    let broken = row("wiki_read", Status::Ported, "link", Kind::Tool);
    let open = [
        broken.clone(),
        not_yet("api_search"),
        row("game-mode", Status::NotYet, "", Kind::Pattern),
    ];
    let problems = [Problem {
        row: broken,
        problem: String::new(),
    }];

    // -- Exec & Check
    assert_eq!(
        verdict(&done, &[]),
        "Port done: nothing left in the old repo."
    );
    assert_eq!(
        verdict(&open, &problems),
        "Not done: 3 features still open.\n- Tools: wiki_read, api_search\n- Patterns: game-mode"
    );
}

#[test]
fn the_ledger_reads_back_what_it_wrote_also_with_a_pipe_in_a_reason() {
    // -- Setup & Fixtures
    let rows = vec![
        row(
            "wiki_read",
            Status::Ported,
            "[tool wiki_read](../src/wiki/read.rs)",
            Kind::Tool,
        ),
        row("mod", Status::NotNeeded, "a | b", Kind::Tool),
        row("ui/timer_hud", Status::NotYet, "", Kind::Recipe),
    ];
    let usage = Usage {
        sessions: HashSet::from(["s1".to_owned()]),
        first: "2026-11-01T10:00:00Z".to_owned(),
        last: "2026-11-15T10:00:00Z".to_owned(),
        ..Usage::default()
    };

    // -- Exec
    let text = render_ledger(&rows, &history_line(&usage));

    // -- Check
    assert!(
        text.contains(
            "\nHistory: 1 sessions with the old server, from 2026-11-01 to 2026-11-15.\n"
        ),
        "{text}"
    );
    assert_eq!(parse_ledger(&text).unwrap(), rows);
    let wrong = text.replace("| not needed |", "| maybe |");
    let line = text
        .lines()
        .position(|l| l.contains("| not needed |"))
        .unwrap()
        + 1;
    assert_eq!(
        parse_ledger(&wrong).unwrap_err().to_string(),
        format!("Line {line}: \"maybe\" is not a status. Use one of: ported, not needed, not yet.")
    );
}

#[test]
fn the_ledger_refuses_a_row_without_a_kind_heading_and_a_row_without_4_cells() {
    // -- Setup & Fixtures
    let no_heading = "# Port ledger\n\n| `mod` | not yet |  |  |\n";
    let unknown_heading = "## Tools\n\n## Others\n| `mod` | not yet |  |  |\n";
    let three_cells = "## Tools\n| `mod` | not yet |  |\n";

    // -- Exec & Check
    for (text, error) in [
        (
            no_heading,
            "Line 3: a row is not under a known kind heading.",
        ),
        (
            unknown_heading,
            "Line 4: a row is not under a known kind heading.",
        ),
        (three_cells, "Line 2: a row needs 4 cells."),
    ] {
        assert_eq!(parse_ledger(text).unwrap_err().to_string(), error);
    }
}

// -- Test Support

fn feature(kind: Kind, name: &str) -> Feature {
    Feature {
        kind,
        name: name.to_owned(),
        uri: None,
    }
}

fn row(name: &str, status: Status, detail: &str, kind: Kind) -> Row {
    Row {
        kind,
        name: name.to_owned(),
        status,
        detail: detail.to_owned(),
        suggestion: String::new(),
    }
}

fn not_yet(name: &str) -> Row {
    row(name, Status::NotYet, "", Kind::Tool)
}

fn four_sessions() -> Usage {
    Usage {
        sessions: ["s1", "s2", "s3", "s4"].map(str::to_owned).into(),
        ..Usage::default()
    }
}

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
