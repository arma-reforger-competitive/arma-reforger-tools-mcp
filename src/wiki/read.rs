//! The `wiki_read` Tool.

use std::{borrow::Cow, sync::Arc};

use rmcp::{
    handler::server::{common::schema_for_type, router::tool::ToolRouter, wrapper::Parameters},
    model::{CallToolResult, ContentBlock, Tool},
};
use schemars::JsonSchema;
use serde::Deserialize;

use super::pages::{Page, Pages, Source};

/// Claude Code cuts Tool answers longer than about 25,000 tokens, which is about 100,000 characters.
const MAX_CONTENT_CHARS: usize = 100_000;

#[derive(Deserialize, JsonSchema)]
struct ReadInput {
    /// Title of the wiki page, for example "Arma Reforger:Mod Project Setup"
    #[schemars(length(min = 1))]
    title: String,
}

/// Gives the wiki Tools to the server. It fails when the built-in page data is not valid.
pub fn tools<S: Send + Sync + 'static>() -> Result<ToolRouter<S>, serde_json::Error> {
    let pages = Arc::new(Pages::parse(include_str!("pages.json"))?);
    let wiki_read = Tool::new(
        "wiki_read",
        "Read the full text of one Arma Reforger wiki page by its title. \
         If no page has that title, the answer lists the closest real titles.",
        schema_for_type::<ReadInput>(),
    );
    Ok(ToolRouter::new().with_route((
        wiki_read,
        move |Parameters(input): Parameters<ReadInput>| read(&pages, &input.title),
    )))
}

// region:    --- Support

fn read(pages: &Pages, title: &str) -> CallToolResult {
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
    CallToolResult::success(vec![ContentBlock::text(page_text(page))])
}

fn page_text(page: &Page) -> String {
    let source = match &page.source {
        Source::BistudioWiki { url } => format!("BI Community Wiki, {url}"),
        Source::Enfusion => "Enfusion engine docs".to_owned(),
    };
    format!(
        "## {}\nSource: {source}\n\n{}",
        page.title,
        page_content(&page.content)
    )
}

fn page_content(content: &str) -> Cow<'_, str> {
    let total = content.chars().count();
    if total <= MAX_CONTENT_CHARS {
        return Cow::Borrowed(content);
    }
    let kept: String = content.chars().take(MAX_CONTENT_CHARS).collect();
    Cow::Owned(format!(
        "{kept}\n\n(Cut at {} of {} characters.)",
        with_commas(MAX_CONTENT_CHARS),
        with_commas(total),
    ))
}

/// Writes 164192 as "164,192".
fn with_commas(number: usize) -> String {
    let digits = number.to_string();
    let mut text = String::new();
    for (index, digit) in digits.chars().enumerate() {
        if index > 0 && (digits.len() - index).is_multiple_of(3) {
            text.push(',');
        }
        text.push(digit);
    }
    text
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
    async fn names_the_source_of_a_page_that_has_no_url() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let page = read_page(&client, "Event System").await;

        // -- Check
        assert!(!page.is_error);
        assert!(page.text.contains("Source: Enfusion engine docs\n"));
    }

    #[tokio::test]
    async fn cuts_a_very_long_page_at_100_000_characters_and_says_so() {
        // -- Setup & Fixtures
        let client = connect().await;

        // -- Exec
        let page = read_page(&client, "Arma Reforger:Diag Menu").await;

        // -- Check
        assert!(!page.is_error);
        assert!(page.text.chars().count() < 100_500);
        assert!(page.text.contains("Cut at 100,000 of 164,192 characters."));
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

    // -- Test Support

    async fn connect() -> TestClient {
        test_support::connect(super::tools().unwrap()).await
    }

    async fn read_page(client: &TestClient, title: &str) -> Answer {
        test_support::call(client, "wiki_read", json!({ "title": title })).await
    }
}

// endregion: --- Tests
