//! Converts the wiki markup inside one line: links, bold and italic, code, HTML tags, and entities.

use super::stash::code_span;
use super::templates::page_url;

/// HTML tags that the wiki allows in a page. Other words in `<...>`, for example `array<string>`, are text.
const HTML_TAGS: [&str; 30] = [
    "span",
    "div",
    "font",
    "center",
    "small",
    "big",
    "u",
    "ins",
    "sup",
    "sub",
    "p",
    "ul",
    "ol",
    "li",
    "abbr",
    "blockquote",
    "poem",
    "spoiler",
    "ref",
    "references",
    "cite",
    "var",
    "samp",
    "b",
    "strong",
    "i",
    "em",
    "s",
    "del",
    "br",
];

#[derive(Clone, Copy, PartialEq)]
enum Style {
    Markdown,
    /// Text only: for headings and code spans.
    Plain,
}

/// Converts the wiki markup in `text` to Markdown.
pub(super) fn markdown(text: &str) -> String {
    convert(text, Style::Markdown)
}

/// Converts the wiki markup in `text` to text with no markup: links keep only their words.
pub(super) fn plain(text: &str) -> String {
    convert(text, Style::Plain)
}

/// Removes the images (`[[File:...]]`). Their captions can run over many lines and hold links,
/// so this runs on the whole page before the lines are converted.
pub(super) fn drop_files(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("[[") {
        let after_open = rest.get(start + 2..).unwrap_or_default();
        let namespace = after_open
            .split_once(':')
            .map(|(namespace, _)| namespace.trim().to_ascii_lowercase());
        let is_file = matches!(namespace.as_deref(), Some("file" | "image" | "media"));
        if let Some(end) = closing_brackets(after_open).filter(|_| is_file) {
            out.push_str(rest.get(..start).unwrap_or_default());
            rest = after_open.get(end + 2..).unwrap_or_default();
        } else {
            out.push_str(rest.get(..start + 2).unwrap_or_default());
            rest = after_open;
        }
    }
    out.push_str(rest);
    out
}

// region:    --- Support

fn convert(text: &str, style: Style) -> String {
    let text = code_tags(text, style);
    let text = internal_links(&text, style);
    let text = external_links(&text, style);
    let text = quotes(&text, style);
    let text = html_tags(&text, style);
    html_escape::decode_html_entities(&text).replace('\u{a0}', " ")
}

/// `<code>`, `<tt>`, and `<kbd>` become code spans. The wiki reads markup inside them, so links keep only their words.
fn code_tags(text: &str, style: Style) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some((start, name, after_open)) = next_tag(rest, &["code", "tt", "kbd"]) {
        let close = format!("</{name}>");
        let Some(end) = after_open.find(&close) else {
            break;
        };
        out.push_str(rest.get(..start).unwrap_or_default());
        let inner = plain(after_open.get(..end).unwrap_or_default());
        if style == Style::Plain {
            out.push_str(&inner);
        } else {
            out.push_str(&code_span(&inner));
        }
        rest = after_open.get(end + close.len()..).unwrap_or_default();
    }
    out.push_str(rest);
    out
}

/// Finds the next opening tag with one of `names`: gives its start, its name, and the text after it.
fn next_tag<'a>(text: &'a str, names: &[&'static str]) -> Option<(usize, &'static str, &'a str)> {
    let mut offset = 0;
    while let Some(found) = text.get(offset..)?.find('<') {
        let start = offset + found;
        let after_lt = text.get(start + 1..)?;
        let name = names.iter().find(|name| {
            after_lt.len() > name.len()
                && after_lt
                    .get(..name.len())
                    .is_some_and(|word| word.eq_ignore_ascii_case(name))
                && after_lt
                    .get(name.len()..)
                    .is_some_and(|rest| rest.starts_with([' ', '>']))
        });
        if let Some(name) = name {
            let open_end = after_lt.find('>')?;
            return Some((start, name, after_lt.get(open_end + 1..)?));
        }
        offset = start + 1;
    }
    None
}

/// `[[Page|words]]` becomes a Markdown link to the wiki page. Files, images, and categories are dropped.
fn internal_links(text: &str, style: Style) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("[[") {
        out.push_str(rest.get(..start).unwrap_or_default());
        let after_open = rest.get(start + 2..).unwrap_or_default();
        let Some(end) = closing_brackets(after_open) else {
            out.push_str("[[");
            rest = after_open;
            continue;
        };
        let inner = after_open.get(..end).unwrap_or_default();
        let after_link = after_open.get(end + 2..).unwrap_or_default();
        // Letters right after the link belong to its words: `[[Mod]]s` shows "Mods".
        let trail_len = after_link
            .find(|c: char| !c.is_ascii_lowercase())
            .unwrap_or(after_link.len());
        let trail = after_link.get(..trail_len).unwrap_or_default();
        if let Some(link) = wiki_link(inner, trail, style) {
            out.push_str(&link);
            rest = after_link.get(trail_len..).unwrap_or_default();
        } else {
            rest = after_link;
        }
    }
    out.push_str(rest);
    out
}

fn closing_brackets(text: &str) -> Option<usize> {
    let mut depth = 1;
    let mut index = 0;
    while let Some(next) = text.get(index..).and_then(|rest| rest.find(['[', ']'])) {
        let at = index + next;
        let rest = text.get(at..)?;
        if rest.starts_with("[[") {
            depth += 1;
            index = at + 2;
        } else if rest.starts_with("]]") {
            depth -= 1;
            if depth == 0 {
                return Some(at);
            }
            index = at + 2;
        } else {
            index = at + 1;
        }
    }
    None
}

/// Writes one `[[...]]` link, or gives `None` for a file, an image, or a category.
fn wiki_link(inner: &str, trail: &str, style: Style) -> Option<String> {
    let (target, words) = inner.split_once('|').unwrap_or((inner, ""));
    let target = target.trim();
    let namespace = target
        .split_once(':')
        .map(|(namespace, _)| namespace.trim().to_ascii_lowercase());
    if matches!(
        namespace.as_deref(),
        Some("file" | "image" | "media" | "category")
    ) {
        return None;
    }
    let target = target.trim_start_matches(':');
    let words = match words.trim() {
        "" => target.trim_start_matches('#').to_owned(),
        words => words.to_owned(),
    };
    let words = format!("{words}{trail}");
    if style == Style::Plain || target.starts_with('#') {
        return Some(words);
    }
    let (page, anchor) = target.split_once('#').unwrap_or((target, ""));
    let mut url = page_url(page);
    if !anchor.is_empty() {
        url.push('#');
        url.push_str(&anchor.trim().replace(' ', "_"));
    }
    Some(format!("[{words}]({url})"))
}

/// `[https://example.com words]` becomes a Markdown link.
fn external_links(text: &str, style: Style) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('[') {
        out.push_str(rest.get(..start).unwrap_or_default());
        let after_open = rest.get(start + 1..).unwrap_or_default();
        let is_url = ["http://", "https://"]
            .iter()
            .any(|scheme| after_open.starts_with(scheme));
        let Some(end) = after_open.find(']').filter(|_| is_url) else {
            out.push('[');
            rest = after_open;
            continue;
        };
        let inner = after_open.get(..end).unwrap_or_default();
        let (url, words) = inner.split_once(' ').unwrap_or((inner, ""));
        let words = words.trim();
        let link = match (style, words.is_empty()) {
            (Style::Plain, true) => url.to_owned(),
            (Style::Plain, false) => words.to_owned(),
            (Style::Markdown, true) => format!("<{url}>"),
            (Style::Markdown, false) => format!("[{words}]({url})"),
        };
        out.push_str(&link);
        rest = after_open.get(end + 1..).unwrap_or_default();
    }
    out.push_str(rest);
    out
}

/// `'''bold'''` and `''italic''` become `**bold**` and `*italic*`.
fn quotes(text: &str, style: Style) -> String {
    let (bold_italic, bold, italic) = match style {
        Style::Markdown => ("***", "**", "*"),
        Style::Plain => ("", "", ""),
    };
    text.replace("'''''", bold_italic)
        .replace("'''", bold)
        .replace("''", italic)
}

/// Drops the HTML tags and keeps their text. Bold, italic, strike, and line breaks keep their meaning.
fn html_tags(text: &str, style: Style) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find('<') {
        out.push_str(rest.get(..start).unwrap_or_default());
        let from_lt = rest.get(start..).unwrap_or_default();
        let Some((tag, len)) = html_tag(from_lt) else {
            out.push('<');
            rest = from_lt.get(1..).unwrap_or_default();
            continue;
        };
        let replacement = match (style, tag.as_str()) {
            (Style::Markdown, "b" | "strong") => "**",
            (Style::Markdown, "i" | "em") => "*",
            (Style::Markdown, "s" | "del") => "~~",
            (Style::Markdown, "br") => "<br>",
            (Style::Plain, "br") => " ",
            _ => "",
        };
        out.push_str(replacement);
        rest = from_lt.get(len..).unwrap_or_default();
    }
    out.push_str(rest);
    out
}

/// Reads an HTML tag at the start of `text`: gives its lowercase name and its length.
/// A `<ref name="x" />` link to a footnote is a tag too.
fn html_tag(text: &str) -> Option<(String, usize)> {
    let end = text.find('>')?;
    let inside = text.get(1..end)?;
    let name_part = inside.strip_prefix('/').unwrap_or(inside);
    let name_len = name_part
        .find(|c: char| !c.is_ascii_alphanumeric())
        .unwrap_or(name_part.len());
    let name = name_part.get(..name_len)?.to_ascii_lowercase();
    let after_name = name_part.get(name_len..)?;
    let is_tag = HTML_TAGS.contains(&name.as_str())
        && (after_name.is_empty() || after_name.starts_with([' ', '/', '\t']));
    is_tag.then_some((name, end + 1))
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_wiki_links_as_markdown_links() {
        // -- Exec & Check
        for (wikitext, expected) in [
            (
                "See [[Arma Reforger:Server Config|the config]].",
                "See [the config](https://community.bistudio.com/wiki/Arma_Reforger:Server_Config).",
            ),
            (
                "[[Arma Reforger:Startup Parameters#checkInstance|-checkInstance]]",
                "[-checkInstance](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#checkInstance)",
            ),
            ("[[#Usage|below]]", "below"),
            (
                "[[Mod]]s",
                "[Mods](https://community.bistudio.com/wiki/Mod)",
            ),
        ] {
            assert_eq!(markdown(wikitext), expected, "{wikitext}");
        }
    }

    #[test]
    fn drops_images_with_links_in_their_caption() {
        // -- Exec
        let text = markdown("A[[File:a.png|thumb|See [[Arma Reforger:Mod]]]]B");

        // -- Check
        assert_eq!(text, "AB");
    }

    #[test]
    fn drops_an_image_whose_caption_runs_over_lines() {
        // -- Exec
        let text = drop_files(
            "Before\n[[File:map.png|thumb|\nStep one<br>\n[[Arma Reforger:Map|Map]]\n]]\nAfter [[Arma Reforger:Mod]]",
        );

        // -- Check
        assert_eq!(text, "Before\n\nAfter [[Arma Reforger:Mod]]");
    }

    #[test]
    fn writes_bold_italic_and_external_links() {
        // -- Exec
        let text = markdown("'''Bold''' and ''italic'' [https://example.com site] [https://a.b]");

        // -- Check
        assert_eq!(
            text,
            "**Bold** and *italic* [site](https://example.com) <https://a.b>"
        );
    }

    #[test]
    fn writes_code_tags_as_code_spans_with_only_the_words_of_links() {
        // -- Exec
        let text =
            markdown("Open <code>[[Arma Reforger:Diag Menu|Diag Menu]] &gt; Statistics</code>.");

        // -- Check
        assert_eq!(text, "Open `Diag Menu > Statistics`.");
    }

    #[test]
    fn drops_html_tags_but_keeps_tag_like_words_and_entities_as_text() {
        // -- Exec
        let text = markdown(
            "<span style=\"color: red\">array<string></span> 2&nbsp;km&sup2;<br>next<ref name=\"n1\" />",
        );

        // -- Check
        assert_eq!(text, "array<string> 2 km²<br>next");
    }
}

// endregion: --- Tests
