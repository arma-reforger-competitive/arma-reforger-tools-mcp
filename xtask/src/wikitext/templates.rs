//! Expands the wiki templates (`{{Link|...}}`, `{{hl|...}}`, `{{Feature|...}}` and the others that the
//! Arma Reforger pages use) into plain wikitext, inner templates first, as the wiki does.

use super::{Error, Result};

const WIKI_URL: &str = "https://community.bistudio.com/wiki/";

/// Expands every template in `text`. An unknown template is an error, so a new template on the wiki
/// shows up at the next import and does not leave `{{` or `}}` in a page.
pub(super) fn expand(text: &str) -> Result<String> {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(start) = rest.find("{{") {
        out.push_str(rest.get(..start).unwrap_or_default());
        let from_braces = rest.get(start..).unwrap_or_default();
        // A run of three braces, as in `{{{!}}`, is one brace of text and then a template.
        let run = from_braces.chars().take_while(|c| *c == '{').count();
        for _ in 2..run {
            out.push('{');
        }
        let inner_start = start + run.max(2);
        let after_open = rest.get(inner_start..).unwrap_or_default();
        // A template that the page never closes ends at its paragraph: the author forgot the `}}`.
        let (inner, after) = if let Some(close) = closing_braces(after_open) {
            (after_open.get(..close), after_open.get(close + 2..))
        } else {
            let end = after_open.find("\n\n").unwrap_or(after_open.len());
            (after_open.get(..end), after_open.get(end..))
        };
        out.push_str(&template(inner.unwrap_or_default())?);
        rest = after.unwrap_or_default();
    }
    out.push_str(rest);
    Ok(out)
}

/// Gives the URL of a wiki page, as the wiki writes it: spaces become underscores.
pub fn page_url(title: &str) -> String {
    let title = title.trim().replace(' ', "_");
    let mut chars = title.chars();
    let first = chars.next().map(|c| c.to_uppercase().to_string());
    let encoded: String = first
        .into_iter()
        .chain(chars.map(String::from))
        .collect::<String>()
        .replace('(', "%28")
        .replace(')', "%29")
        .replace('"', "%22");
    format!("{WIKI_URL}{encoded}")
}

// region:    --- Support

/// The arguments of one template call. The wiki counts only the unnamed arguments, and `1=` names the first one.
struct Args {
    positional: Vec<String>,
    named: Vec<(String, String)>,
}

impl Args {
    /// Gives the argument at `position` (from 1), or an empty text when it is missing.
    fn get(&self, position: usize) -> &str {
        self.named(&position.to_string())
            .or_else(|| {
                position
                    .checked_sub(1)
                    .and_then(|index| self.positional.get(index))
                    .map(String::as_str)
            })
            .unwrap_or_default()
    }

    fn named(&self, name: &str) -> Option<&str> {
        self.named
            .iter()
            .find(|(key, _)| key == name)
            .map(|(_, value)| value.as_str())
    }
}

/// Finds the `}}` that closes a template, in the text after its `{{`.
fn closing_braces(text: &str) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth = 1;
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        let next = bytes.get(index + 1).copied();
        if byte == b'{' && next == Some(b'{') {
            depth += 1;
            index += 2;
        } else if byte == b'}' && next == Some(b'}') {
            depth -= 1;
            if depth == 0 {
                return Some(index);
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    None
}

/// Splits `text` at each `separator` that is not inside `{{ }}` or `[[ ]]`.
fn split_top_level(text: &str, separator: u8, limit: usize) -> Vec<&str> {
    let bytes = text.as_bytes();
    let mut parts = Vec::new();
    let (mut braces, mut brackets) = (0usize, 0usize);
    let mut part_start = 0;
    let mut index = 0;
    while let Some(&byte) = bytes.get(index) {
        let pair = bytes.get(index + 1) == Some(&byte);
        match byte {
            b'{' if pair => braces += 1,
            b'}' if pair => braces = braces.saturating_sub(1),
            b'[' if pair => brackets += 1,
            b']' if pair => brackets = brackets.saturating_sub(1),
            _ => {}
        }
        if matches!(byte, b'{' | b'}' | b'[' | b']') && pair {
            index += 2;
            continue;
        }
        if byte == separator && braces == 0 && brackets == 0 && parts.len() + 1 < limit {
            parts.push(text.get(part_start..index).unwrap_or_default());
            part_start = index + 1;
        }
        index += 1;
    }
    parts.push(text.get(part_start..).unwrap_or_default());
    parts
}

/// Expands one template call: `inner` is the text between `{{` and `}}`.
fn template(inner: &str) -> Result<String> {
    let mut parts = split_top_level(inner, b'|', usize::MAX).into_iter();
    let name = parts.next().unwrap_or_default().trim().replace('_', " ");
    if name.contains(':') {
        // A parser function, for example `{{DISPLAYTITLE:...}}`, only changes how the wiki shows the page.
        return Ok(String::new());
    }
    let mut args = Args {
        positional: Vec::new(),
        named: Vec::new(),
    };
    for part in parts {
        match split_top_level(part, b'=', 2).as_slice() {
            [key, value] => args
                .named
                .push((key.trim().to_owned(), expand(value)?.trim().to_owned())),
            _ => args.positional.push(expand(part)?),
        }
    }
    render(&name, &args)
}

/// Writes one template as wikitext. The wiki matches the first letter of a template name in any case.
fn render(name: &str, args: &Args) -> Result<String> {
    let mut chars = name.chars();
    let key: String = chars
        .next()
        .map(|c| c.to_ascii_uppercase())
        .into_iter()
        .chain(chars)
        .collect();
    let text = match key.as_str() {
        "!" => "|".to_owned(),
        "=" => "=".to_owned(),
        "ArmaR" | "Arma" | "Arma3" | "Arma4" | "Dayz" | "Tkoh" | "Enfusion" => {
            game_name(name).to_owned()
        }
        "Name" if args.get(1) == "bi" && args.get(2) == "short" => "BI".to_owned(),
        "Name" => game_name(args.get(1)).to_owned(),
        "TOC" | "Clear" | "Wiki" => String::new(),
        "GameCategory" => game_category(args),
        "Link" => link(args.get(1), args.get(2)),
        "Link/Enfusion" => enfusion_link(args),
        "Hl" => format!("<code>{}</code>", args.get(1)),
        "Feature" => feature(args.get(1), args.get(2)),
        "Informative" => feature("informative", args.get(1)),
        "Messagebox" | "Sticky" | "Sic" | "Colorball" => args.get(1).to_owned(),
        "Hatnote" => format!("''{}''", args.get(1).trim()),
        "Columns" | "Color" => args.get(2).to_owned(),
        "GVI" => format!("{} {}", game_name(args.get(1)), args.get(2)),
        "ArgTitle" => arg_title(args),
        "Controls" => format!("[{}]", args.positional.join("+")),
        "Icon" => icon(args.get(1)).to_owned(),
        "N/a" => "n/a".to_owned(),
        "Youtube" => youtube(args.get(1), args.get(2)),
        "Infobox/WorkbenchPlugin" | "Infobox/Terrain" => infobox(args),
        _ => return Err(Error::UnknownTemplate(name.to_owned())),
    };
    Ok(text)
}

fn game_name(code: &str) -> &str {
    match code.trim() {
        "armaR" => "Arma Reforger",
        "arma" => "Arma",
        "arma3" => "Arma 3",
        "arma4" => "Arma 4",
        "dayz" => "DayZ",
        "tkoh" => "Take On Helicopters",
        "enfusion" => "Enfusion",
        "bi" => "Bohemia Interactive",
        "ofp" => "Operation Flashpoint",
        other => other,
    }
}

/// `{{GameCategory|armaR|Modding}}` only puts the page in a category. With `text=` or `link=y` it also shows a word.
fn game_category(args: &Args) -> String {
    if let Some(text) = args.named("text") {
        return text.to_owned();
    }
    if args.named("link") == Some("y") {
        return game_name(args.get(1)).to_owned();
    }
    String::new()
}

fn link(target: &str, text: &str) -> String {
    let (target, text) = (target.trim(), text.trim());
    if let Some(anchor) = target.strip_prefix('#') {
        return if text.is_empty() { anchor } else { text }.to_owned();
    }
    if target.starts_with("http://") || target.starts_with("https://") {
        return if text.is_empty() {
            target.to_owned()
        } else {
            format!("[{target} {text}]")
        };
    }
    if target.contains("://") {
        // `enfusion://` and `steam://` links open the Workbench or Steam, so they are shown as paths.
        let path = format!("<code>{target}</code>");
        return if text.is_empty() {
            path
        } else {
            format!("{text} ({path})")
        };
    }
    if text.is_empty() {
        format!("[[{target}]]")
    } else {
        format!("[[{target}|{text}]]")
    }
}

/// `{{Link/Enfusion|armaR|SCR_Class|Method}}` links to the script API docs of a class or one of its methods.
fn enfusion_link(args: &Args) -> String {
    let class = args.get(2).trim();
    let member = args.get(3).trim();
    if member.is_empty() {
        format!("<code>{class}</code>")
    } else {
        format!("<code>{class}.{member}</code>")
    }
}

/// A box with a kind (note, warning, ...) becomes a paragraph that starts with the kind in bold.
/// The empty lines around it keep it apart from a list above or below.
fn feature(kind: &str, text: &str) -> String {
    let label = match kind.trim() {
        "informative" => "Note",
        "important" => "Important",
        "warning" => "Warning",
        "quote" => "Quote",
        "obsolete" => "Obsolete",
        "gamepad" => "Gamepad",
        "consoles" => "Consoles",
        "PC" => "PC",
        "gvi" => "Version",
        other => game_name(other),
    };
    let text = text.trim();
    if text.starts_with(['*', '#', ':', ';', '{']) {
        format!("\n'''{label}:'''\n{text}\n")
    } else {
        format!("\n'''{label}:''' {text}\n")
    }
}

/// `{{ArgTitle|3|Title|{{GVI|armaR|1.6.0}}}}` is a heading with the game version on the next line.
fn arg_title(args: &Args) -> String {
    let level = args.get(1).trim().parse::<usize>().unwrap_or(2).clamp(1, 6);
    let equals = "=".repeat(level);
    let heading = format!("{equals} {} {equals}", args.get(2).trim());
    let extra = args.get(3).trim();
    if extra.is_empty() {
        heading
    } else {
        format!("{heading}\n{extra}")
    }
}

fn icon(name: &str) -> &str {
    match name.trim() {
        "checked" => "Yes",
        "unchecked" => "No",
        "warning" => "Warning",
        "unknown" => "Unknown",
        "xbox" => "Xbox",
        "playstation" => "PlayStation",
        other => other,
    }
}

fn youtube(id: &str, title: &str) -> String {
    let title = title.trim();
    let title = if title.is_empty() { "video" } else { title };
    format!(
        "[https://www.youtube.com/watch?v={} Video: {title}]",
        id.trim()
    )
}

/// An infobox becomes a list of its filled-in fields.
fn infobox(args: &Args) -> String {
    args.named
        .iter()
        .filter(|(key, value)| !value.is_empty() && !key.starts_with("image"))
        .map(|(key, value)| {
            // A table must start on its own line.
            let gap = if value.starts_with("{|") { "\n" } else { " " };
            format!("* '''{key}''':{gap}{value}")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expands_inner_templates_first() {
        // -- Exec
        let text =
            expand("{{Feature|important|Keep {{hl|fastValidation}} on in {{armaR}}.}}").unwrap();

        // -- Check
        assert_eq!(
            text,
            "\n'''Important:''' Keep <code>fastValidation</code> on in Arma Reforger.\n"
        );
    }

    #[test]
    fn keeps_the_pipe_of_a_wiki_link_inside_a_template_argument() {
        // -- Exec
        let text = expand("{{hl|[[Arma Reforger:Diag Menu|Diag Menu]] > Statistics}}").unwrap();

        // -- Check
        assert_eq!(
            text,
            "<code>[[Arma Reforger:Diag Menu|Diag Menu]] > Statistics</code>"
        );
    }

    #[test]
    fn reads_a_numbered_argument_as_a_positional_one() {
        // -- Exec
        let text = expand("{{Informative|1=Fix it: a=b}}").unwrap();

        // -- Check
        assert_eq!(text, "\n'''Note:''' Fix it: a=b\n");
    }

    #[test]
    fn writes_a_pipe_and_a_brace_for_table_syntax_inside_templates() {
        // -- Exec
        let text = expand("{{{!}}\n{{!}} cell\n{{!}}}").unwrap();

        // -- Check
        assert_eq!(text, "{|\n| cell\n|}");
    }

    #[test]
    fn writes_links_to_wiki_pages_websites_and_workbench_paths() {
        // -- Exec & Check
        for (wikitext, expected) in [
            (
                "{{Link|Arma Reforger:Server Config|config}}",
                "[[Arma Reforger:Server Config|config]]",
            ),
            (
                "{{Link|https://example.com|Example}}",
                "[https://example.com Example]",
            ),
            (
                "{{Link|enfusion://ResourceManager/~ArmaReforger:Configs/US.conf}}",
                "<code>enfusion://ResourceManager/~ArmaReforger:Configs/US.conf</code>",
            ),
            ("{{Link|#.wav}}", ".wav"),
            (
                "{{Link/Enfusion|armaR|AudioSystem}}",
                "<code>AudioSystem</code>",
            ),
        ] {
            assert_eq!(expand(wikitext).unwrap(), expected, "{wikitext}");
        }
    }

    #[test]
    fn turns_a_titled_argument_into_a_heading_with_the_version_below() {
        // -- Exec
        let text = expand("{{ArgTitle|3|Opus Ogg Import|{{GVI|armaR|1.6.0}}}}").unwrap();

        // -- Check
        assert_eq!(text, "=== Opus Ogg Import ===\nArma Reforger 1.6.0");
    }

    #[test]
    fn ends_a_template_that_is_never_closed_at_its_paragraph() {
        // -- Exec
        let text = expand("{{Feature|important|Define it.\n\n=== Next ===").unwrap();

        // -- Check
        assert_eq!(text, "\n'''Important:''' Define it.\n\n\n=== Next ===");
    }

    #[test]
    fn fails_on_an_unknown_template() {
        // -- Exec
        let error = expand("Text {{NewTemplate|x}}").unwrap_err();

        // -- Check
        assert!(matches!(error, Error::UnknownTemplate(name) if name == "NewTemplate"));
    }

    #[test]
    fn writes_the_url_of_a_page_with_underscores() {
        // -- Exec & Check
        assert_eq!(
            page_url("Arma Reforger:Server Config"),
            "https://community.bistudio.com/wiki/Arma_Reforger:Server_Config"
        );
    }
}

// endregion: --- Tests
