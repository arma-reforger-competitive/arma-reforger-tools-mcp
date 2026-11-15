//! Converts the lines of a page after the templates are expanded: headings, lists, tables,
//! preformatted lines, and paragraphs.

use super::inline;
use super::stash::{Place, Stash};
use super::tables;

/// Converts expanded wikitext to Markdown. In a table cell, headings become bold text.
pub(super) fn markdown(text: &str, stash: &Stash, place: Place) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let mut out: Vec<String> = Vec::new();
    let mut index = 0;
    while let Some(line) = lines.get(index) {
        index += 1;
        let line = remove_magic_words(line);
        let trimmed = line.trim_start();
        if tables::opens_table(&line) {
            let end = table_end(&lines, index - 1);
            let table_lines = lines.get(index - 1..end).unwrap_or_default();
            out.push(String::new());
            out.push(tables::markdown(table_lines, stash));
            out.push(String::new());
            index = end;
        } else if line.starts_with(' ') && !trimmed.is_empty() {
            // The wiki shows lines that start with a space as preformatted text.
            let start = index - 1;
            while lines
                .get(index)
                .is_some_and(|next| next.starts_with(' ') && !next.trim().is_empty())
            {
                index += 1;
            }
            out.push(String::new());
            out.push("```".to_owned());
            for pre in lines.get(start..index).unwrap_or_default() {
                out.push(inline::plain(pre.get(1..).unwrap_or_default()));
            }
            out.push("```".to_owned());
            out.push(String::new());
        } else if let Some(heading) = heading(&line, place) {
            out.push(String::new());
            out.push(heading);
            out.push(String::new());
        } else if trimmed == "----" || trimmed.starts_with("-----") {
            out.extend([String::new(), "---".to_owned(), String::new()]);
        } else if line.starts_with(['*', '#', ':', ';']) {
            out.push(list_item(&line));
        } else {
            out.push(inline::markdown(line.trim_end()));
        }
    }
    out.join("\n")
}

// region:    --- Support

fn remove_magic_words(line: &str) -> String {
    let mut line = line.to_owned();
    for word in ["__NOTOC__", "__TOC__", "__FORCETOC__", "__NOEDITSECTION__"] {
        line = line.replace(word, "");
    }
    line
}

/// Gives the index after the `|}` that closes the table that starts at `start`. Tables can be nested.
fn table_end(lines: &[&str], start: usize) -> usize {
    let mut depth = 0usize;
    for (index, line) in lines.iter().enumerate().skip(start) {
        if tables::opens_table(line) {
            depth += 1;
        } else if line.trim_start().starts_with("|}") {
            depth = depth.saturating_sub(1);
            if depth == 0 {
                return index + 1;
            }
        }
    }
    lines.len()
}

/// `== Text ==` becomes `## Text`.
fn heading(line: &str, place: Place) -> Option<String> {
    let line = line.trim_end();
    let open = line.chars().take_while(|c| *c == '=').count();
    let close = line.chars().rev().take_while(|c| *c == '=').count();
    if open == 0 || close == 0 || line.len() <= open + close {
        return None;
    }
    let level = open.min(close).min(6);
    let inner = line.get(level..line.len() - level)?;
    let text = inline::plain(inner).trim().to_owned();
    if text.is_empty() {
        return None;
    }
    Some(match place {
        Place::Page => format!("{} {text}", "#".repeat(level)),
        Place::TableCell => format!("**{text}**"),
    })
}

/// `*`, `#`, `:`, and `;` at the start of a line make lists, indented text, and terms.
fn list_item(line: &str) -> String {
    let prefix_len = line
        .find(|c: char| !matches!(c, '*' | '#' | ':' | ';'))
        .unwrap_or(line.len());
    let prefix = line.get(..prefix_len).unwrap_or_default();
    let body = inline::markdown(line.get(prefix_len..).unwrap_or_default().trim());
    let Some(last) = prefix.chars().last() else {
        return body;
    };
    let indent: String = prefix
        .chars()
        .take(prefix.chars().count() - 1)
        .map(|c| if c == '#' { "   " } else { "  " })
        .collect();
    match last {
        '*' => format!("{indent}- {body}"),
        '#' => format!("{indent}1. {body}"),
        ';' => format!("{indent}**{body}**"),
        _ => format!("{indent}{body}"),
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn convert(text: &str) -> String {
        let (text, stash) = Stash::take(text);
        markdown(&text, &stash, Place::Page)
    }

    #[test]
    fn writes_wiki_headings_as_markdown_headings() {
        // -- Exec
        let text = convert("== Summary ==\nText\n=== fastValidation ===");

        // -- Check
        assert_eq!(text, "\n## Summary\n\nText\n\n### fastValidation\n");
    }

    #[test]
    fn writes_nested_lists_and_terms() {
        // -- Exec
        let text = convert("* one\n** two\n# first\n#* point\n; Term\n: text");

        // -- Check
        assert_eq!(text, "- one\n  - two\n1. first\n   - point\n**Term**\ntext");
    }

    #[test]
    fn writes_lines_that_start_with_a_space_as_a_code_block() {
        // -- Exec
        let text = convert("Output:\n count,class\n 144,array<string>\nEnd");

        // -- Check
        assert_eq!(
            text,
            "Output:\n\n```\ncount,class\n144,array<string>\n```\n\nEnd"
        );
    }
}

// endregion: --- Tests
