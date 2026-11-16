//! Splits a page into Sections: one heading with all the text and smaller headings under it.

/// One heading of a page and where its text is in the page content.
pub(super) struct Section {
    /// The heading names from the top of the page down to this heading, for example `["game", "mods"]`.
    /// The text before the first heading has an empty path.
    pub(super) path: Vec<String>,

    /// The byte range of the heading line and the text up to the next heading of any level.
    pub(super) own: std::ops::Range<usize>,

    /// The byte range of the heading line and the text up to the next heading of the same or a higher level.
    pub(super) whole: std::ops::Range<usize>,
}

impl Section {
    /// The path as Claude writes it: `game > mods`.
    pub(super) fn path_text(&self) -> String {
        self.path.join(" > ")
    }

    /// The heading line and the text up to the next heading of any level, from the page `content`.
    pub(super) fn own_text<'a>(&self, content: &'a str) -> &'a str {
        content.get(self.own.clone()).unwrap_or_default()
    }

    /// The heading line and the text with the smaller headings under it, from the page `content`.
    pub(super) fn whole_text<'a>(&self, content: &'a str) -> &'a str {
        content.get(self.whole.clone()).unwrap_or_default()
    }
}

/// Finds the Sections of a Markdown page. Lines inside code blocks are never headings.
pub(super) fn sections(content: &str) -> Vec<Section> {
    let headings = headings(content);
    let mut sections = Vec::new();
    let first_heading = headings
        .first()
        .map_or(content.len(), |heading| heading.start);
    if !content
        .get(..first_heading)
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        sections.push(Section {
            path: Vec::new(),
            own: 0..first_heading,
            whole: 0..first_heading,
        });
    }
    let mut stack: Vec<(usize, String)> = Vec::new();
    for (index, heading) in headings.iter().enumerate() {
        while stack
            .last()
            .is_some_and(|(level, _)| *level >= heading.level)
        {
            stack.pop();
        }
        stack.push((heading.level, heading.name.clone()));
        let own_end = headings
            .get(index + 1)
            .map_or(content.len(), |next| next.start);
        let whole_end = headings
            .iter()
            .skip(index + 1)
            .find(|next| next.level <= heading.level)
            .map_or(content.len(), |next| next.start);
        sections.push(Section {
            path: stack.iter().map(|(_, name)| name.clone()).collect(),
            own: heading.start..own_end,
            whole: heading.start..whole_end,
        });
    }
    sections
}

// region:    --- Support

struct Heading {
    level: usize,
    name: String,

    /// The byte where the heading line starts.
    start: usize,
}

fn headings(content: &str) -> Vec<Heading> {
    let mut headings = Vec::new();
    let mut in_code = false;
    let mut start = 0;
    for line in content.split_inclusive('\n') {
        let text = line.trim_end();
        if text.trim_start().starts_with("```") {
            in_code = !in_code;
        } else if !in_code {
            let level = text.chars().take_while(|c| *c == '#').count();
            if (1..=6).contains(&level)
                && let Some(name) = text.get(level..).and_then(|rest| rest.strip_prefix(' '))
                && !name.trim().is_empty()
            {
                headings.push(Heading {
                    level,
                    name: name.trim().to_owned(),
                    start,
                });
            }
        }
        start += line.len();
    }
    headings
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gives_each_heading_its_path_and_its_text_with_the_smaller_headings_under_it() {
        // -- Setup & Fixtures
        let content = "Intro\n## Root\nr\n### password\np1\n## game\n```\n## not a heading\n```\n### password\np2\n";

        // -- Exec
        let sections = sections(content);

        // -- Check
        let paths: Vec<String> = sections.iter().map(Section::path_text).collect();
        assert_eq!(
            paths,
            ["", "Root", "Root > password", "game", "game > password"]
        );
        let root = &sections[1];
        assert_eq!(content.get(root.own.clone()).unwrap(), "## Root\nr\n");
        assert_eq!(
            content.get(root.whole.clone()).unwrap(),
            "## Root\nr\n### password\np1\n"
        );
        let game = &sections[3];
        assert_eq!(
            content.get(game.whole.clone()).unwrap(),
            "## game\n```\n## not a heading\n```\n### password\np2\n"
        );
    }
}

// endregion: --- Tests
