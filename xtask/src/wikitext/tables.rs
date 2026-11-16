//! Converts a wiki table (`{| ... |}`) to a Markdown table.

use super::blocks;
use super::inline;
use super::stash::{Place, Stash};

struct Cell {
    header: bool,
    text: String,

    /// How many rows and columns the cell covers: `rowspan="2"` and `colspan="2"`.
    rows: usize,
    columns: usize,
}

/// Converts the lines of one table, from `{|` to `|}`. A cell that holds more than one line
/// gets `<br>` between its lines, because a Markdown table cell is one line.
pub(super) fn markdown(lines: &[&str], stash: &Stash) -> String {
    let (caption, rows) = parse(lines);
    let grid = grid(&rows, stash);
    let columns = grid.iter().map(Vec::len).max().unwrap_or(0);
    let mut out = Vec::new();
    if let Some(caption) = caption {
        out.push(format!("**{}**", inline::plain(&caption).trim()));
        out.push(String::new());
    }
    for (index, row) in grid.iter().enumerate() {
        let cells: Vec<&str> = (0..columns)
            .map(|column| row.get(column).map_or("", String::as_str))
            .collect();
        out.push(format!("| {} |", cells.join(" | ")));
        if index == 0 {
            out.push(format!("|{}", " --- |".repeat(columns)));
        }
    }
    out.join("\n")
}

/// A table starts with `{|` at the start of a line. `:` in front of it only indents it.
pub(super) fn opens_table(line: &str) -> bool {
    line.trim_start()
        .trim_start_matches(':')
        .trim_start()
        .starts_with("{|")
}

// region:    --- Support

/// Places the cells on a grid. A cell that covers more rows repeats its text in each of them,
/// so every row keeps its columns in place. A cell that covers more columns leaves the others empty.
fn grid(rows: &[Vec<Cell>], stash: &Stash) -> Vec<Vec<String>> {
    // For each column: the text that comes down from a cell above, and for how many more rows.
    let mut from_above: Vec<Option<(String, usize)>> = Vec::new();
    let mut grid = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        let mut texts: Vec<String> = Vec::new();
        let mut cells = row.iter();
        loop {
            let column = texts.len();
            if let Some(slot) = from_above.get_mut(column)
                && let Some((text, left)) = slot
            {
                texts.push(text.clone());
                *left -= 1;
                if *left == 0 {
                    *slot = None;
                }
                continue;
            }
            let Some(cell) = cells.next() else {
                if from_above.iter().skip(column).any(Option::is_some) {
                    texts.push(String::new());
                    continue;
                }
                break;
            };
            let text = cell_text(cell, index, stash);
            for offset in 0..cell.columns {
                let at = column + offset;
                let shown = if offset == 0 {
                    text.clone()
                } else {
                    String::new()
                };
                if from_above.len() <= at {
                    from_above.resize(at + 1, None);
                }
                if cell.rows > 1
                    && let Some(slot) = from_above.get_mut(at)
                {
                    *slot = Some((shown.clone(), cell.rows - 1));
                }
                texts.push(shown);
            }
        }
        grid.push(texts);
    }
    grid
}

/// Reads the caption and the rows. Text on a line with no `|` or `!` at its start belongs to the cell above.
fn parse(lines: &[&str]) -> (Option<String>, Vec<Vec<Cell>>) {
    let mut caption = None;
    let mut rows: Vec<Vec<Cell>> = Vec::new();
    let mut row: Vec<Cell> = Vec::new();
    let mut index = 1;
    while let Some(line) = lines.get(index) {
        index += 1;
        let trimmed = line.trim_start();
        if opens_table(line) {
            // A table inside a cell: keep its lines in the cell, and convert it with the cell.
            let start = index - 1;
            let mut depth = 1usize;
            while depth > 0
                && let Some(next) = lines.get(index)
            {
                if opens_table(next) {
                    depth += 1;
                } else if next.trim_start().starts_with("|}") {
                    depth -= 1;
                }
                index += 1;
            }
            let nested = lines.get(start..index).unwrap_or_default().join("\n");
            append(&mut row, &nested);
        } else if trimmed.starts_with("|}") {
            break;
        } else if let Some(text) = trimmed.strip_prefix("|+") {
            caption = Some(text.to_owned());
        } else if trimmed.starts_with("|-") {
            if !row.is_empty() {
                rows.push(std::mem::take(&mut row));
            }
        } else if let Some(text) = trimmed.strip_prefix('!') {
            let cells = text.split("!!").flat_map(|part| part.split("||"));
            row.extend(cells.map(|text| cell(text, true)));
        } else if let Some(text) = trimmed.strip_prefix('|') {
            row.extend(text.split("||").map(|text| cell(text, false)));
        } else {
            append(&mut row, line);
        }
    }
    if !row.is_empty() {
        rows.push(row);
    }
    (caption, rows)
}

fn append(row: &mut [Cell], text: &str) {
    if let Some(last) = row.last_mut() {
        last.text.push('\n');
        last.text.push_str(text);
    }
}

/// Drops the attributes in front of the cell text: `style="color: red" | text` keeps `text`.
fn cell(text: &str, header: bool) -> Cell {
    let (attributes, text) = match single_pipe(text) {
        Some(at) if is_attributes(text.get(..at).unwrap_or_default()) => (
            text.get(..at).unwrap_or_default(),
            text.get(at + 1..).unwrap_or_default(),
        ),
        _ => ("", text),
    };
    Cell {
        header,
        text: text.to_owned(),
        rows: span(attributes, "rowspan"),
        columns: span(attributes, "colspan"),
    }
}

/// Reads `rowspan="2"` or `colspan=2` from the cell attributes. It is 1 when it is missing.
fn span(attributes: &str, name: &str) -> usize {
    attributes
        .find(name)
        .and_then(|start| attributes.get(start + name.len()..))
        .and_then(|rest| rest.trim_start().strip_prefix('='))
        .map(|value| {
            value
                .trim_start()
                .trim_start_matches(['"', '\''])
                .chars()
                .take_while(char::is_ascii_digit)
                .collect::<String>()
        })
        .and_then(|digits| digits.parse().ok())
        .unwrap_or(1)
        .clamp(1, 100)
}

/// Finds a `|` that is not inside a `[[ ]]` link.
fn single_pipe(text: &str) -> Option<usize> {
    let mut depth = 0usize;
    let bytes = text.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        match byte {
            b'[' => depth += 1,
            b']' => depth = depth.saturating_sub(1),
            b'|' if depth == 0 => return Some(index),
            _ => {}
        }
    }
    None
}

/// `style="..." colspan=2` looks like attributes. Text with links, code, or no `=` does not.
fn is_attributes(text: &str) -> bool {
    let text = text.trim();
    !text.is_empty()
        && text.contains('=')
        && !text.contains(['[', '<', '{', '\u{E000}'])
        && text
            .split_whitespace()
            .next()
            .is_some_and(|word| word.chars().next().is_some_and(char::is_alphabetic))
}

fn cell_text(cell: &Cell, row: usize, stash: &Stash) -> String {
    let markdown = blocks::markdown(cell.text.trim(), stash, Place::TableCell);
    let text = stash
        .restore(&markdown, Place::TableCell)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("<br>")
        .replace('|', "\\|");
    if cell.header && row > 0 && !text.is_empty() {
        format!("**{text}**")
    } else {
        text
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    fn convert(wikitext: &str) -> String {
        let (text, stash) = Stash::take(wikitext);
        let lines: Vec<&str> = text.lines().collect();
        markdown(&lines, &stash)
    }

    #[test]
    fn writes_header_and_data_rows() {
        // -- Exec
        let text = convert(
            "{| class=\"wikitable\"\n! Name !! Type\n|-\n| bindPort || int\n|-\n| style=\"color: red\" | game || object\n|}",
        );

        // -- Check
        assert_eq!(
            text,
            "| Name | Type |\n| --- | --- |\n| bindPort | int |\n| game | object |"
        );
    }

    #[test]
    fn repeats_a_cell_that_covers_two_rows_so_the_columns_stay_in_place() {
        // -- Exec
        let text = convert(
            "{|\n! Port !! Protocol !! Reason\n|-\n| 2001 || rowspan=\"2\" | UDP || Public\n|-\n| 17777 || A2S\n|-\n| colspan=\"2\" | all || end\n|}",
        );

        // -- Check
        assert_eq!(
            text,
            "| Port | Protocol | Reason |\n| --- | --- | --- |\n| 2001 | UDP | Public |\n| 17777 | UDP | A2S |\n| all |  | end |"
        );
    }

    #[test]
    fn joins_the_lines_of_a_cell_with_line_breaks() {
        // -- Exec
        let text = convert("{|\n! Key\n! Meaning\n|-\n| a\n|\n* one\n* two\n|}");

        // -- Check
        assert_eq!(
            text,
            "| Key | Meaning |\n| --- | --- |\n| a | - one<br>- two |"
        );
    }

    #[test]
    fn writes_a_row_header_in_bold_and_escapes_pipes() {
        // -- Exec
        let text = convert("{|\n! A !! B\n|-\n! Row\n| <enforce inline>a || b</enforce>\n|}");

        // -- Check
        assert_eq!(text, "| A | B |\n| --- | --- |\n| **Row** | `a \\|\\| b` |");
    }
}

// endregion: --- Tests
