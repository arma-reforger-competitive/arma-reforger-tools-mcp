//! Converts Doxygen HTML to Markdown: headings, paragraphs, lists, tables, notes, and code.

use std::fmt::Write as _;

use scraper::{ElementRef, Node};

/// Converts the content of `element` to Markdown blocks. A heading `<hN>` becomes `N + shift` `#` signs.
pub fn blocks(element: ElementRef, shift: usize) -> String {
    let mut writer = Writer {
        blocks: Vec::new(),
        paragraph: String::new(),
        shift,
    };
    writer.children(element);
    writer.flush();
    writer.blocks.join("\n\n")
}

/// Converts the content of `element` to one line of Markdown text.
pub fn inline(element: ElementRef) -> String {
    let mut text = String::new();
    inline_into(element, &mut text);
    collapse_spaces(&text)
}

/// Gives the text of `element` with each run of spaces as one space.
pub fn text(element: ElementRef) -> String {
    collapse_spaces(&element.text().collect::<String>())
}

// region:    --- Support

struct Writer {
    blocks: Vec<String>,
    paragraph: String,
    shift: usize,
}

impl Writer {
    fn children(&mut self, element: ElementRef) {
        for child in element.children() {
            match child.value() {
                Node::Text(text) => self.paragraph.push_str(text),
                Node::Element(_) => {
                    if let Some(child) = ElementRef::wrap(child) {
                        self.element(child);
                    }
                }
                _ => {}
            }
        }
    }

    fn element(&mut self, element: ElementRef) {
        let name = element.value().name();
        let has_class = |class: &str| element.value().has_class(class, CASE_SENSITIVE);
        match name {
            "p" => self.block(inline(element)),
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level: usize = name.get(1..).and_then(|n| n.parse().ok()).unwrap_or(1);
                let hashes = "#".repeat((level + self.shift).min(6));
                self.block(format!("{hashes} {}", inline(element)));
            }
            "ul" | "ol" => self.block(list(element, name == "ol", "")),
            "dl" => self.block(definitions(element)),
            "table" => self.block(table(element)),
            "pre" => self.block(code_block(&element.text().collect::<String>())),
            "div" if has_class("fragment") => self.block(fragment(element)),
            "div" if has_class("toc") => {}
            "blockquote" => {
                let quoted = blocks(element, self.shift);
                self.block(
                    quoted
                        .lines()
                        .map(|line| format!("> {line}"))
                        .collect::<Vec<_>>()
                        .join("\n"),
                );
            }
            "hr" => self.block("---".to_owned()),
            "br" => self.paragraph.push('\n'),
            "img" | "iframe" | "script" | "style" | "map" => {}
            "a" | "code" | "tt" | "em" | "i" | "strong" | "b" | "span" | "sup" | "sub" | "kbd"
            | "small" => inline_element(element, &mut self.paragraph),
            _ => {
                self.flush();
                self.children(element);
                self.flush();
            }
        }
    }

    fn block(&mut self, text: String) {
        self.flush();
        if !text.trim().is_empty() {
            self.blocks.push(text);
        }
    }

    fn flush(&mut self) {
        let paragraph = std::mem::take(&mut self.paragraph);
        let lines: Vec<String> = paragraph
            .lines()
            .map(collapse_spaces)
            .filter(|line| !line.is_empty())
            .collect();
        if !lines.is_empty() {
            self.blocks.push(lines.join("\n"));
        }
    }
}

const CASE_SENSITIVE: scraper::CaseSensitivity = scraper::CaseSensitivity::CaseSensitive;

/// Writes the content of `element` as inline Markdown.
fn inline_into(element: ElementRef, out: &mut String) {
    for child in element.children() {
        match child.value() {
            Node::Text(text) => out.push_str(text),
            Node::Element(_) => {
                if let Some(child) = ElementRef::wrap(child) {
                    inline_element(child, out);
                }
            }
            _ => {}
        }
    }
}

/// Writes `element` itself as inline Markdown: code, emphasis, and links keep their meaning.
fn inline_element(element: ElementRef, out: &mut String) {
    match element.value().name() {
        "code" | "tt" | "kbd" => {
            let code = text(element);
            if !code.is_empty() {
                let _ = write!(out, "`{code}`");
            }
        }
        "em" | "i" => wrap(element, "*", out),
        "strong" | "b" => wrap(element, "**", out),
        "a" => link(element, out),
        "br" => out.push(' '),
        "img" | "iframe" | "script" | "style" | "map" => {}
        _ => inline_into(element, out),
    }
}

fn wrap(element: ElementRef, marker: &str, out: &mut String) {
    let inner = inline(element);
    if !inner.is_empty() {
        let _ = write!(out, "{marker}{inner}{marker}");
    }
}

/// A link to a website keeps its URL. A link inside the docs keeps only its words.
fn link(element: ElementRef, out: &mut String) {
    let words = inline(element);
    match element.value().attr("href") {
        Some(href) if href.starts_with("http") && !words.is_empty() => {
            let _ = write!(out, "[{words}]({href})");
        }
        _ => out.push_str(&words),
    }
}

/// Nested lists go under the text of their item: 2 spaces under `-`, 3 spaces under `1.`.
fn list(element: ElementRef, ordered: bool, indent: &str) -> String {
    let marker = if ordered { "1." } else { "-" };
    let mut lines = Vec::new();
    for item in element
        .child_elements()
        .filter(|e| e.value().name() == "li")
    {
        let mut text = String::new();
        let mut nested = Vec::new();
        for child in item.children() {
            match child.value() {
                Node::Text(part) => text.push_str(part),
                Node::Element(_) => {
                    let Some(child) = ElementRef::wrap(child) else {
                        continue;
                    };
                    match child.value().name() {
                        name @ ("ul" | "ol") => {
                            let inner = format!("{indent}{}", " ".repeat(marker.len() + 1));
                            nested.push(list(child, name == "ol", &inner));
                        }
                        "div" if child.value().has_class("fragment", CASE_SENSITIVE) => {
                            nested.push(fragment(child));
                        }
                        "p" => {
                            text.push(' ');
                            text.push_str(&inline(child));
                        }
                        _ => inline_element(child, &mut text),
                    }
                }
                _ => {}
            }
        }
        lines.push(format!("{indent}{marker} {}", collapse_spaces(&text)));
        lines.extend(nested);
    }
    lines.join("\n")
}

/// Doxygen writes notes, warnings, and return values as `<dl class="section note">`.
fn definitions(element: ElementRef) -> String {
    let mut out = Vec::new();
    let mut term = String::new();
    for child in element.child_elements() {
        match child.value().name() {
            "dt" => term = text(child),
            "dd" => {
                let body = blocks(child, 0).replace("\n\n", "\n");
                if term.is_empty() {
                    out.push(body);
                } else if body.starts_with('|') {
                    // A table must start on its own line, or Markdown reads it as text.
                    out.push(format!("**{term}:**\n\n{body}"));
                } else {
                    out.push(format!("**{term}:** {body}"));
                }
            }
            _ => {}
        }
    }
    out.join("\n\n")
}

/// Markdown reads the first row of a table as its header. A table with no `<th>` in its first row,
/// for example the parameters of a method, gets an empty header, so that its first row stays data.
fn table(element: ElementRef) -> String {
    let rows: Vec<ElementRef> = element
        .descendants()
        .filter_map(ElementRef::wrap)
        .filter(|e| e.value().name() == "tr")
        .collect();
    let has_header = rows
        .first()
        .is_some_and(|row| row.child_elements().any(|cell| cell.value().name() == "th"));
    let rows: Vec<Vec<String>> = rows
        .into_iter()
        .map(|row| {
            row.child_elements()
                .filter(|cell| matches!(cell.value().name(), "td" | "th"))
                .map(|cell| inline(cell).replace('|', "\\|"))
                .collect()
        })
        .filter(|cells: &Vec<String>| !cells.is_empty())
        .collect();
    let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
    let mut lines = Vec::new();
    if !has_header {
        lines.push(format!("|{}", "  |".repeat(columns)));
        lines.push(format!("|{}", " --- |".repeat(columns)));
    }
    for (index, row) in rows.iter().enumerate() {
        let cells: Vec<&str> = (0..columns)
            .map(|column| row.get(column).map_or("", String::as_str))
            .collect();
        lines.push(format!("| {} |", cells.join(" | ")));
        if index == 0 && has_header {
            lines.push(format!("|{}", " --- |".repeat(columns)));
        }
    }
    lines.join("\n")
}

/// A Doxygen code example is a `div.fragment` with one `div.line` per line.
fn fragment(element: ElementRef) -> String {
    let lines: Vec<String> = element
        .child_elements()
        .filter(|line| line.value().has_class("line", CASE_SENSITIVE))
        .map(|line| line.text().collect::<String>().replace('\u{a0}', " "))
        .collect();
    code_block(&lines.join("\n"))
}

fn code_block(code: &str) -> String {
    format!("```\n{}\n```", code.trim_matches('\n').trim_end())
}

fn collapse_spaces(text: &str) -> String {
    text.replace('\u{a0}', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use scraper::Html;

    use super::*;

    fn convert(html: &str) -> String {
        let document = Html::parse_fragment(html);
        blocks(document.root_element(), 1)
    }

    #[test]
    fn writes_headings_paragraphs_lists_and_code() {
        // -- Exec
        let markdown = convert(
            "<h1 class=\"doxsection\">Intro</h1><p>Use <code>Rpc</code> <b>only</b> on the <a href=\"interfaceRplNode.html\">RplNode</a>.</p>\
             <ul><li>one<ul><li>two</li></ul></li></ul>\
             <div class=\"fragment\"><div class=\"line\">int&#160;x;</div><div class=\"line\">x++;</div></div>",
        );

        // -- Check
        assert_eq!(
            markdown,
            "## Intro\n\nUse `Rpc` **only** on the RplNode.\n\n- one\n  - two\n\n```\nint x;\nx++;\n```"
        );
    }

    #[test]
    fn writes_notes_and_tables() {
        // -- Exec
        let markdown = convert(
            "<dl class=\"section warning\"><dt>Warning</dt><dd>Only before <a href=\"https://x.y\">insert</a>.</dd></dl>\
             <table><tr><th>Key</th><th>Value</th></tr><tr><td>a|b</td><td>1</td></tr></table>",
        );

        // -- Check
        assert_eq!(
            markdown,
            "**Warning:** Only before [insert](https://x.y).\n\n| Key | Value |\n| --- | --- |\n| a\\|b | 1 |"
        );
    }

    #[test]
    fn writes_a_parameter_table_below_its_term_with_every_parameter_as_data() {
        // -- Exec
        let markdown = convert(
            "<dl class=\"params\"><dt>Parameters</dt><dd><table class=\"params\">\
             <tr><td class=\"paramname\">iNum</td><td>Number of image</td></tr>\
             <tr><td class=\"paramname\">pWidget</td><td>Widget to set the texture in</td></tr>\
             </table></dd></dl>",
        );

        // -- Check
        assert_eq!(
            markdown,
            "**Parameters:**\n\n|  |  |\n| --- | --- |\n| iNum | Number of image |\n| pWidget | Widget to set the texture in |"
        );
    }
}

// endregion: --- Tests
