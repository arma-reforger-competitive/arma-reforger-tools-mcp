//! The `wiki_read` Tool.

use std::collections::HashSet;

use rmcp::model::{CallToolResult, ContentBlock};
use schemars::JsonSchema;
use serde::Deserialize;

use super::pages::{Page, Pages, Source};
use super::sections::Section;

#[derive(Deserialize, JsonSchema)]
pub(super) struct ReadInput {
    /// Title of the wiki page, for example "Arma Reforger:Mod Project Setup"
    #[schemars(length(min = 1))]
    title: String,

    /// Only this Section of the page: a heading name, for example "mods", or a path of headings,
    /// for example "game > password". Leave it out to read the whole page.
    section: Option<String>,
}

/// Answers one `wiki_read` call.
pub(super) fn read(pages: &Pages, input: &ReadInput) -> CallToolResult {
    let title = input.title.as_str();
    // Underscores count as spaces in a title, so "_" alone is empty too.
    if title
        .trim_matches(|c: char| c == '_' || c.is_whitespace())
        .is_empty()
    {
        let text = "The title is empty. Give the title of a wiki page, \
                    for example \"Arma Reforger:Mod Project Setup\".";
        return CallToolResult::error(vec![ContentBlock::text(text)]);
    }
    let Some(page) = pages.find(title) else {
        let suggestions = pages
            .closest_titles(title, 3)
            .iter()
            .map(|suggestion| format!("\"{suggestion}\""))
            .collect::<Vec<_>>()
            .join(", ");
        let text = format!(
            "No wiki page has the title \"{title}\". The closest titles are: {suggestions}."
        );
        return CallToolResult::error(vec![ContentBlock::text(text)]);
    };
    let wanted = input.section.as_deref().map_or("", str::trim);
    if wanted.is_empty() {
        return CallToolResult::success(vec![ContentBlock::text(page_text(page))]);
    }
    match find_sections(page, wanted) {
        Ok(sections) => {
            CallToolResult::success(vec![ContentBlock::text(section_text(page, &sections))])
        }
        Err(text) => CallToolResult::error(vec![ContentBlock::text(text)]),
    }
}

// region:    --- Support

fn page_text(page: &Page) -> String {
    format!(
        "## {}\n{}\n\n{}",
        page.title,
        source_line(page),
        page.content
    )
}

/// The text of one Section with the smaller headings under it. Some pages repeat a heading path, for
/// example "Root > address" in Server Config: then every Section with that path is in the answer.
fn section_text(page: &Page, sections: &[&Section]) -> String {
    let path = sections
        .first()
        .map_or_else(String::new, |section| section.path_text());
    let text: Vec<&str> = sections
        .iter()
        .map(|section| section.whole_text(page.content).trim_end())
        .collect();
    format!(
        "## {} > {path}\n{}\n\n{}\n",
        page.title,
        source_line(page),
        text.join("\n\n")
    )
}

fn source_line(page: &Page) -> String {
    let source = match page.source {
        Source::BistudioWiki => "BI Community Wiki",
        Source::Enfusion => "Enfusion engine docs",
    };
    format!("Source: {source}, {}", page.url)
}

/// Finds the Sections that `wanted` names: a full path first, then a path or a heading name at the
/// end of a path. A name that ends more than one path gives an error that lists the paths.
// Reason: the hint for a wrong section name from Claude (docs/agents/rust.md, Errors).
// ast-grep-ignore: string-error
fn find_sections<'a>(page: &'a Page, wanted: &str) -> Result<Vec<&'a Section>, String> {
    let parts: Vec<String> = wanted
        .split('>')
        .map(|part| part.trim().to_lowercase())
        .collect();
    let lower = |section: &Section| -> Vec<String> {
        section
            .path
            .iter()
            .map(|name| name.to_lowercase())
            .collect()
    };
    let named: Vec<&Section> = page
        .sections
        .iter()
        .filter(|s| !s.path.is_empty())
        .collect();
    let exact: Vec<&Section> = named
        .iter()
        .copied()
        .filter(|s| lower(s) == parts)
        .collect();
    if !exact.is_empty() {
        return Ok(exact);
    }
    let ending: Vec<&Section> = named
        .iter()
        .copied()
        .filter(|s| lower(s).ends_with(&parts))
        .collect();
    // Paths that differ only in capital letters are one path, because the match ignores them too.
    let mut seen = HashSet::new();
    let paths: Vec<String> = ending
        .iter()
        .filter(|s| seen.insert(lower(s)))
        .map(|s| s.path_text())
        .collect();
    match paths.len() {
        1 => Ok(ending),
        0 => {
            let all: Vec<String> = named
                .iter()
                .map(|s| format!("\"{}\"", s.path_text()))
                .collect();
            // Reason: the hint for a wrong section name from Claude (docs/agents/rust.md, Errors).
            // ast-grep-ignore: string-error
            Err(format!(
                "The page \"{}\" has no section \"{wanted}\". Its sections are: {}.",
                page.title,
                all.join(", ")
            ))
        }
        _ => {
            let listed: Vec<String> = paths.iter().map(|path| format!("\"{path}\"")).collect();
            // Reason: the hint for a wrong section name from Claude (docs/agents/rust.md, Errors).
            // ast-grep-ignore: string-error
            Err(format!(
                "The page \"{}\" has {} sections named \"{wanted}\". Give one of these paths as the \
                 section: {}.",
                page.title,
                paths.len(),
                listed.join(", ")
            ))
        }
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::test_support::{self, Answer, TestClient};

    #[tokio::test]
    async fn returns_the_page_text_for_a_known_title() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let page = read_page(&client, "Arma Reforger:Mod Project Setup").await;

        // -- Check
        assert!(!page.is_error);
        assert!(page.text.contains("## Arma Reforger:Mod Project Setup"));
        assert!(
            page.text
                .contains("https://community.bistudio.com/wiki/Arma_Reforger:Mod_Project_Setup")
        );
        assert!(page.text.contains("Workbench Launcher Setup"));
    }

    #[tokio::test]
    async fn finds_the_page_when_the_title_has_wrong_capital_letters_or_underscores_from_the_url() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec & Check
        for title in [
            "arma reforger:mod project setup",
            "Arma_Reforger:Mod_Project_Setup",
        ] {
            let page = read_page(&client, title).await;

            assert!(!page.is_error, "{title}");
            assert!(
                page.text.contains("## Arma Reforger:Mod Project Setup"),
                "{title}"
            );
        }
    }

    #[tokio::test]
    async fn names_the_engine_docs_and_their_url_as_the_source_of_an_engine_page() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let page = read_page(&client, "Event System").await;

        // -- Check
        assert!(!page.is_error);
        assert!(page.text.contains(
            "Source: Enfusion engine docs, https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_EventSystem.html\n"
        ));
    }

    #[tokio::test]
    async fn asks_for_a_title_when_the_title_is_empty() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec & Check
        for title in ["", " _ "] {
            let page = read_page(&client, title).await;

            assert!(page.is_error, "{title:?}");
            assert_eq!(
                page.text,
                "The title is empty. Give the title of a wiki page, for example \"Arma Reforger:Mod Project Setup\"."
            );
        }
    }

    #[tokio::test]
    async fn suggests_the_closest_real_titles_when_the_title_is_unknown() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec & Check
        for title in ["Mod Project Setup", "Arma Reforger:Mod Projct Setp"] {
            let page = read_page(&client, title).await;

            assert!(page.is_error, "{title}");
            assert!(
                page.text
                    .contains(&format!("No wiki page has the title \"{title}\""))
            );
            assert!(page.text.contains("\"Arma Reforger:Mod Project Setup\""));
        }
    }

    #[tokio::test]
    async fn returns_only_the_section_with_the_heading_name() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = read_section(&client, "Arma Reforger:Server Config", "mods").await;

        // -- Check
        assert!(!answer.is_error, "{}", answer.text);
        assert!(answer.text.starts_with("## Arma Reforger:Server Config > game > mods\nSource: BI Community Wiki, https://community.bistudio.com/wiki/Arma_Reforger:Server_Config\n\n### mods\n"));
        assert!(answer.text.contains("#### modID"));
        assert!(!answer.text.contains("## gameProperties"));
        assert!(answer.text.len() < 5_000);
    }

    #[tokio::test]
    async fn returns_the_section_with_the_path_when_two_headings_have_the_same_name() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = read_section(&client, "Arma Reforger:Server Config", "Game > Password").await;

        // -- Check
        assert!(!answer.is_error, "{}", answer.text);
        assert!(
            answer
                .text
                .starts_with("## Arma Reforger:Server Config > game > password\n")
        );
        assert!(!answer.text.contains("### passwordAdmin"));
    }

    #[tokio::test]
    async fn lists_the_full_paths_when_the_heading_name_is_used_more_than_once() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = read_section(&client, "Arma Reforger:Server Config", "password").await;

        // -- Check
        assert!(answer.is_error);
        assert_eq!(
            answer.text,
            "The page \"Arma Reforger:Server Config\" has 2 sections named \"password\". Give one of these paths as the section: \"rcon > password\", \"game > password\"."
        );
    }

    #[tokio::test]
    async fn returns_every_section_when_the_page_repeats_the_same_path() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = read_section(&client, "Arma Reforger:World Editor", "Viewport").await;

        // -- Check
        assert!(!answer.is_error, "{}", answer.text);
        assert_eq!(answer.text.matches("## Viewport\n").count(), 2);
    }

    #[tokio::test]
    async fn returns_both_sections_when_two_headings_differ_only_in_capital_letters() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = read_section(&client, "Arma Reforger:Diag Menu", "wheel drag set").await;

        // -- Check
        assert!(!answer.is_error, "{}", answer.text);
        assert!(answer.text.contains("## Wheel drag set\n"));
        assert!(answer.text.contains("## wheel drag set\n"));
    }

    #[tokio::test]
    async fn keeps_a_code_line_that_starts_with_a_hash_sign_in_the_section() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = read_section(
            &client,
            "Language Server Protocol: Neovim configuration",
            "Start LSP client",
        )
        .await;

        // -- Check
        assert!(!answer.is_error, "{}", answer.text);
        assert!(answer.text.contains(
            "# In case you are using Neovim 0.12+ you can run following instead\n:lsp enable enforce-ls\n"
        ));
        assert!(!answer.text.contains("## Enforce tree-sitter"));
    }

    #[tokio::test]
    async fn lists_the_sections_of_the_page_when_no_section_has_the_name() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let answer = read_section(&client, "Arma Reforger:Server Config", "nothing here").await;

        // -- Check
        assert!(answer.is_error);
        assert!(answer.text.starts_with("The page \"Arma Reforger:Server Config\" has no section \"nothing here\". Its sections are: \"Summary\", \"Root\", \"Root > bindAddress\""));
    }

    // -- Test Support

    async fn read_section(client: &TestClient, title: &str, section: &str) -> Answer {
        test_support::call(
            client,
            "wiki_read",
            json!({ "title": title, "section": section }),
        )
        .await
    }

    async fn connect() -> TestClient {
        test_support::connect(crate::wiki::tools().unwrap()).await
    }

    async fn read_page(client: &TestClient, title: &str) -> Answer {
        test_support::call(client, "wiki_read", json!({ "title": title })).await
    }
}

// endregion: --- Tests
