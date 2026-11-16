//! The steps from wikitext to Markdown.

use super::Result;
use super::blocks;
use super::inline;
use super::stash::{Place, Stash};
use super::templates;

/// Converts the wikitext of one page to Markdown. It fails when the page uses an unknown template.
pub fn to_markdown(wikitext: &str) -> Result<String> {
    let (text, mut stash) = Stash::take(wikitext);
    stash.convert_spoilers(to_markdown)?;
    let text = inline::drop_files(&templates::expand(&text)?);
    let markdown = blocks::markdown(&text, &stash, Place::Page);
    Ok(tidy(&stash.restore(&markdown, Place::Page)))
}

// region:    --- Support

/// Removes spaces at line ends and keeps at most one empty line in a row. Code blocks stay as they are.
fn tidy(markdown: &str) -> String {
    let mut out: Vec<&str> = Vec::new();
    let mut in_code = false;
    for line in markdown.lines() {
        if line.trim_start().starts_with("```") {
            in_code = !in_code;
        }
        let line = if in_code { line } else { line.trim_end() };
        let after_empty = out.last().is_none_or(|last| last.is_empty());
        if line.is_empty() && !in_code && after_empty {
            continue;
        }
        out.push(line);
    }
    while out.last().is_some_and(|line| line.is_empty()) {
        out.pop();
    }
    let mut text = out.join("\n");
    text.push('\n');
    text
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_a_page_with_templates_code_and_a_table() {
        // -- Setup & Fixtures
        let wikitext = "{{TOC|side}}\n== gameProperties ==\n\n\n{{Feature|important|Keep {{hl|fastValidation}} to {{hl|true}}.}}\n<enforce>\nint x;\n</enforce>\n{|\n! Key\n|-\n| value\n|}\n{{GameCategory|armaR|Modding}}";

        // -- Exec
        let markdown = to_markdown(wikitext).unwrap();

        // -- Check
        assert_eq!(
            markdown,
            "## gameProperties\n\n**Important:** Keep `fastValidation` to `true`.\n\n```enforce\nint x;\n```\n\n| Key |\n| --- |\n| value |\n"
        );
    }
}

// endregion: --- Tests
