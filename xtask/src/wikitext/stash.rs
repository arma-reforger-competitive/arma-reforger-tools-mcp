//! Takes comments, code, and `<nowiki>` text out of the wikitext before the templates are expanded,
//! and puts the code back at the end. The wiki does the same: it does not read wiki markup inside these tags.

use std::fmt::Write as _;

/// Marks the start and the end of a stashed item in the text. These are private-use characters,
/// so they are never in a wiki page.
const OPEN: char = '\u{E000}';
const CLOSE: char = '\u{E001}';

/// Tags whose content the wiki does not read as page markup. `<gallery>` holds only image names,
/// so it is dropped. `<spoiler>` is a box that opens on a click: the wiki reads its content as a page
/// of its own, so a table can start in the middle of a line there.
const RAW_TAGS: [&str; 9] = [
    "nowiki",
    "pre",
    "enforce",
    "syntaxhighlight",
    "source",
    "sqf",
    "math",
    "gallery",
    "spoiler",
];

pub(super) struct Stash {
    items: Vec<Item>,
}

enum Item {
    Code {
        lang: String,
        code: String,
        inline: bool,
    },
    Text(String),

    /// The wikitext of a spoiler, until `convert_spoilers` turns it into Markdown.
    Spoiler(String),
    Markdown(String),
}

/// Where the restored text goes. A Markdown table cell cannot hold a code block.
#[derive(Clone, Copy)]
pub(super) enum Place {
    Page,
    TableCell,
}

impl Stash {
    /// Gives the wikitext without comments, with a placeholder for each code block and `<nowiki>` text.
    pub(super) fn take(wikitext: &str) -> (String, Self) {
        let mut stash = Self { items: Vec::new() };
        let mut text = String::with_capacity(wikitext.len());
        let mut rest = wikitext;
        while let Some(start) = rest.find('<') {
            let (before, from_tag) = rest.split_at(start);
            text.push_str(before);
            if let Some(after_comment) = from_tag.strip_prefix("<!--") {
                rest = after_comment
                    .find("-->")
                    .map_or("", |end| after_comment.get(end + 3..).unwrap_or_default());
                continue;
            }
            if let Some(tag) = raw_tag(from_tag) {
                if let Some(item) = tag.item {
                    stash.push(&mut text, item);
                }
                rest = tag.rest;
            } else {
                text.push('<');
                rest = from_tag.get(1..).unwrap_or_default();
            }
        }
        text.push_str(rest);
        (text, stash)
    }

    /// Converts the content of each spoiler with `convert`, which is the converter of a whole page.
    pub(super) fn convert_spoilers<E>(
        &mut self,
        convert: impl Fn(&str) -> Result<String, E>,
    ) -> Result<(), E> {
        for item in &mut self.items {
            if let Item::Spoiler(wikitext) = item {
                *item = Item::Markdown(convert(wikitext)?);
            }
        }
        Ok(())
    }

    /// Puts the stashed items back in place of their placeholders.
    pub(super) fn restore(&self, text: &str, place: Place) -> String {
        let mut out = String::with_capacity(text.len());
        let mut rest = text;
        while let Some(start) = rest.find(OPEN) {
            out.push_str(rest.get(..start).unwrap_or_default());
            let after = rest.get(start + OPEN.len_utf8()..).unwrap_or_default();
            let Some(end) = after.find(CLOSE) else {
                rest = after;
                continue;
            };
            let item = after
                .get(..end)
                .and_then(|index| index.parse::<usize>().ok())
                .and_then(|index| self.items.get(index));
            if let Some(item) = item {
                out.push_str(&render(item, place));
            }
            rest = after.get(end + CLOSE.len_utf8()..).unwrap_or_default();
        }
        out.push_str(rest);
        out
    }
}

/// Writes `code` as one Markdown code span, with longer backtick fences when the code holds backticks.
pub(super) fn code_span(code: &str) -> String {
    let code = code.trim();
    if code.is_empty() {
        return String::new();
    }
    if code.contains('`') {
        format!("`` {code} ``")
    } else {
        format!("`{code}`")
    }
}

// region:    --- Support

impl Stash {
    fn push(&mut self, text: &mut String, item: Item) {
        let _ = write!(text, "{OPEN}{}{CLOSE}", self.items.len());
        self.items.push(item);
    }
}

struct RawTag<'a> {
    /// `None` when the tag gives no text, for example `<nowiki/>` or a gallery.
    item: Option<Item>,
    rest: &'a str,
}

/// Reads one raw tag at the start of `text`, or gives `None` when `text` does not start with one.
fn raw_tag(text: &str) -> Option<RawTag<'_>> {
    let after_lt = text.get(1..)?;
    let name_len = after_lt
        .find(|c: char| !c.is_ascii_alphabetic())
        .unwrap_or(after_lt.len());
    let name = after_lt.get(..name_len)?.to_ascii_lowercase();
    if !RAW_TAGS.contains(&name.as_str()) {
        return None;
    }
    let open_end = text.find('>')?;
    let attributes = text.get(1 + name_len..open_end)?;
    if !attributes.is_empty() && !attributes.starts_with([' ', '\t', '\n', '/']) {
        return None;
    }
    let after_open = text.get(open_end + 1..)?;
    if attributes.trim_end().ends_with('/') {
        return Some(RawTag {
            item: None,
            rest: after_open,
        });
    }
    let content_len = closing_tag(after_open, &name)?;
    let content = after_open.get(..content_len)?;
    let rest = after_open.get(content_len + name.len() + 3..)?;
    let inline = attributes.split_whitespace().any(|word| {
        word.trim_end_matches('/')
            .split('=')
            .next()
            .is_some_and(|key| key == "inline")
    });
    let item = match name.as_str() {
        "gallery" => None,
        "spoiler" => Some(Item::Spoiler(content.to_owned())),
        "sqf" => Some(code("sqf".to_owned(), content.to_owned(), inline)),
        "nowiki" => Some(Item::Text(decode_entities(content))),
        "math" => Some(code(String::new(), content.to_owned(), true)),
        "pre" => Some(code(String::new(), decode_entities(content), false)),
        "enforce" => Some(code("enforce".to_owned(), content.to_owned(), inline)),
        _ => Some(code(
            attribute(attributes, "lang"),
            content.to_owned(),
            inline,
        )),
    };
    Some(RawTag { item, rest })
}

/// Finds the `</name>` that closes a tag, in the text after the tag. The same tag can be nested.
fn closing_tag(text: &str, name: &str) -> Option<usize> {
    let lower = text.to_ascii_lowercase();
    let (open, close) = (format!("<{name}"), format!("</{name}>"));
    let mut depth = 1;
    let mut index = 0;
    loop {
        let next_close = index + lower.get(index..)?.find(&close)?;
        let next_open = lower
            .get(index..)
            .and_then(|rest| rest.find(&open))
            .map(|found| index + found)
            .filter(|at| *at < next_close);
        if let Some(at) = next_open {
            depth += 1;
            index = at + open.len();
        } else {
            depth -= 1;
            if depth == 0 {
                return Some(next_close);
            }
            index = next_close + close.len();
        }
    }
}

fn code(lang: String, code: String, inline: bool) -> Item {
    Item::Code { lang, code, inline }
}

/// Reads `name="value"` from the attributes of a tag.
fn attribute(attributes: &str, name: &str) -> String {
    let pattern = format!("{name}=\"");
    attributes
        .find(&pattern)
        .and_then(|start| attributes.get(start + pattern.len()..))
        .and_then(|value| value.split('"').next())
        .unwrap_or_default()
        .to_owned()
}

fn decode_entities(text: &str) -> String {
    html_escape::decode_html_entities(text).replace('\u{a0}', " ")
}

fn render(item: &Item, place: Place) -> String {
    match item {
        Item::Text(text) | Item::Spoiler(text) => text.clone(),
        Item::Markdown(markdown) => match place {
            Place::Page => format!("\n{markdown}\n"),
            Place::TableCell => markdown.clone(),
        },
        Item::Code { code, inline, .. } if *inline => code_span(&code.replace('\n', " ")),
        Item::Code { lang, code, .. } => match place {
            Place::Page => format!("\n```{lang}\n{}\n```\n", trim_blank_lines(code)),
            Place::TableCell => trim_blank_lines(code)
                .lines()
                .map(code_span)
                .filter(|line| !line.is_empty())
                .collect::<Vec<_>>()
                .join("<br>"),
        },
    }
}

/// Removes the empty lines at the start and the end of a code block, and keeps the indent of the first line.
fn trim_blank_lines(code: &str) -> &str {
    let start = code
        .find(|c: char| !c.is_whitespace())
        .map_or(code.len(), |first| {
            code.get(..first)
                .and_then(|blank| blank.rfind('\n'))
                .map_or(0, |newline| newline + 1)
        });
    code.get(start..).unwrap_or_default().trim_end()
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drops_comments_and_keeps_the_text_around_them() {
        // -- Exec
        let (text, _) = Stash::take("Keep <!-- an editor note --> this.<!-- open to the end");

        // -- Check
        assert_eq!(text, "Keep  this.");
    }

    #[test]
    fn keeps_wiki_markup_inside_code_as_it_is() {
        // -- Setup & Fixtures
        let wikitext = "Use:\n<enforce>\n\tPrint({{hl|x}}); // <!-- not a comment -->\n</enforce>";

        // -- Exec
        let (text, stash) = Stash::take(wikitext);
        let page = stash.restore(&text, Place::Page);

        // -- Check
        assert!(!text.contains("hl"));
        assert_eq!(
            page,
            "Use:\n\n```enforce\n\tPrint({{hl|x}}); // <!-- not a comment -->\n```\n"
        );
    }

    #[test]
    fn writes_inline_code_as_a_code_span() {
        // -- Exec
        let (text, stash) = Stash::take("Call <enforce inline>GetGame()</enforce> first.");

        // -- Check
        assert_eq!(stash.restore(&text, Place::Page), "Call `GetGame()` first.");
    }

    #[test]
    fn writes_each_code_line_as_a_code_span_in_a_table_cell() {
        // -- Exec
        let (text, stash) =
            Stash::take("<syntaxhighlight lang=\"json\">\n{\n  \"a\": 1\n}\n</syntaxhighlight>");

        // -- Check
        assert_eq!(
            stash.restore(&text, Place::TableCell),
            "`{`<br>`\"a\": 1`<br>`}`"
        );
    }

    #[test]
    fn keeps_a_tag_like_word_that_is_not_a_raw_tag() {
        // -- Exec
        let (text, _) = Stash::take("array<ref PotentialTarget> and <preview>");

        // -- Check
        assert_eq!(text, "array<ref PotentialTarget> and <preview>");
    }
}

// endregion: --- Tests
