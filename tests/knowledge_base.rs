//! Checks the Knowledge base files that `cargo xtask import` writes.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

#[test]
fn server_config_keeps_code_names_and_real_headings() {
    // -- Exec
    let page = read("src/wiki/pages/arma_reforger/server_config.md");

    // -- Check
    assert!(page.contains("### fastValidation\n"));
    assert!(page.contains("\n## gameProperties\n"));
    assert!(page.contains("`-maxFPS`"));
}

#[test]
fn has_every_wiki_page_and_engine_page_with_a_title_and_a_url() {
    // -- Setup & Fixtures
    // Counted in src/wiki/source/export.xml (Arma Reforger namespace, without the empty
    // "Material Editor" placeholder) and in the pages.html list of the Workbench docs.
    let (wiki_pages, engine_pages) = (252, 24);

    // -- Exec
    let files = markdown_files(&root().join("src/wiki/pages"));

    // -- Check
    let headers: Vec<(String, String)> = files.iter().map(|file| header(file)).collect();
    let wiki = headers
        .iter()
        .filter(|(_, url)| url.starts_with("https://community.bistudio.com/wiki/"))
        .count();
    let engine = headers
        .iter()
        .filter(|(_, url)| {
            url.starts_with("https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_")
        })
        .count();
    assert_eq!((wiki, engine), (wiki_pages, engine_pages));
    assert!(headers.iter().all(|(title, _)| !title.is_empty()));
}

#[test]
fn no_page_has_template_braces_or_wiki_markup_outside_code() {
    // -- Exec & Check
    for file in markdown_files(&root().join("src/wiki/pages")) {
        let text = without_code(&fs::read_to_string(&file).unwrap());
        for markup in ["{{", "}}", "<!--", "-->", "[[", "]]", "'''", "{|", "|}"] {
            assert!(!text.contains(markup), "{} has {markup}", file.display());
        }
    }
}

#[test]
fn no_page_has_the_text_of_a_wiki_comment() {
    // -- Setup & Fixtures
    let export = read("src/wiki/source/export.xml");
    let comments: Vec<String> = export
        .split("&lt;!--")
        .skip(1)
        .filter_map(|rest| rest.split_once("--&gt;").map(|(comment, _)| comment))
        .map(|comment| comment.split_whitespace().collect::<Vec<_>>().join(" "))
        // Short comments such as "or 1.0.0?" can be normal words in a page too.
        .filter(|comment| comment.len() >= 40 && !comment.contains('&'))
        .collect();
    let pages: Vec<String> = markdown_files(&root().join("src/wiki/pages"))
        .iter()
        .map(|file| {
            fs::read_to_string(file)
                .unwrap()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect();

    // -- Exec & Check
    assert_ne!(comments, [] as [std::string::String; 0]);
    for comment in &comments {
        assert!(
            !pages.iter().any(|page| page.contains(comment.as_str())),
            "a page has the comment text: {comment}"
        );
    }
}

#[test]
fn has_every_api_class_of_both_folders_without_params_or_empty_fields() {
    // -- Setup & Fixtures
    // Counted as the interface*.html files without -members.html in each Workbench docs folder.
    let (enfusion_classes, arma_classes) = (963, 8156);

    // -- Exec
    let classes: Vec<Value> = read("src/api-docs/classes.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect();

    // -- Check
    let count = |source: &str| {
        classes
            .iter()
            .filter(|class| class["source"] == source)
            .count()
    };
    assert_eq!(
        (count("enfusion"), count("arma")),
        (enfusion_classes, arma_classes)
    );
    for class in &classes {
        let url = class["docsUrl"].as_str().unwrap();
        assert!(
            Path::new(url)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("html")),
            "{url}"
        );
        assert_no_params_or_empty_fields(class);
    }
}

// region:    --- Test Support

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    fs::read_to_string(root().join(path)).unwrap()
}

fn markdown_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(markdown_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    files
}

fn header(file: &Path) -> (String, String) {
    let text = fs::read_to_string(file).unwrap();
    let lines: Vec<&str> = text.lines().take(4).collect();
    assert_eq!(lines[0], "---", "{}", file.display());
    assert_eq!(lines[3], "---", "{}", file.display());
    (
        lines[1].strip_prefix("title: ").unwrap().to_owned(),
        lines[2].strip_prefix("url: ").unwrap().to_owned(),
    )
}

/// Removes code blocks and code spans: code can hold `}}` or `[[` on purpose.
fn without_code(text: &str) -> String {
    let mut out = String::new();
    let mut in_block = false;
    for line in text.lines() {
        if line.trim_start().starts_with("```") {
            in_block = !in_block;
            continue;
        }
        if in_block {
            continue;
        }
        let mut in_span = false;
        let mut ticks = 0;
        for c in line.chars() {
            if c == '`' {
                ticks += 1;
                continue;
            }
            if ticks > 0 {
                in_span = !in_span;
                ticks = 0;
            }
            if !in_span {
                out.push(c);
            }
        }
        out.push('\n');
    }
    out
}

fn assert_no_params_or_empty_fields(value: &Value) {
    match value {
        Value::Object(fields) => {
            assert!(!fields.contains_key("params"), "{value}");
            for field in fields.values() {
                assert!(
                    field != &Value::String(String::new()) && field != &Value::Array(Vec::new()),
                    "{value}"
                );
                assert_no_params_or_empty_fields(field);
            }
        }
        Value::Array(items) => items.iter().for_each(assert_no_params_or_empty_fields),
        _ => {}
    }
}

// endregion: --- Test Support
