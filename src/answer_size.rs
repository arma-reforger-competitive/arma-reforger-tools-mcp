//! Cuts long Tool answers before Claude Code gets them.

use rmcp::model::{CallToolResult, ContentBlock};

/// Claude Code cuts a Tool answer at about 25,000 tokens. That is about 100,000 characters.
const MAX_CHARS: usize = 100_000;

/// Cuts `result` at 100,000 characters, counted over all its text blocks. A cut answer ends with
/// "(Cut at 100,000 of N characters.)". The blocks after the cut are left out, also the blocks
/// that are not text.
pub fn cut(mut result: CallToolResult) -> CallToolResult {
    let total: usize = result
        .content
        .iter()
        .filter_map(ContentBlock::as_text)
        .map(|block| block.text.chars().count())
        .sum();
    if total <= MAX_CHARS {
        return result;
    }
    let note = format!(
        "\n\n(Cut at {} of {} characters.)",
        with_commas(MAX_CHARS),
        with_commas(total)
    );
    let mut left = MAX_CHARS;
    let mut kept = Vec::new();
    for mut block in result.content {
        if let ContentBlock::Text(content) = &mut block {
            if let Some((end, _)) = content.text.char_indices().nth(left) {
                content.text.truncate(end);
            }
            let count = content.text.chars().count();
            if count == left {
                content.text.push_str(&note);
                kept.push(block);
                break;
            }
            left -= count;
        }
        kept.push(block);
    }
    result.content = kept;
    result
}

// region:    --- Support

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
    use super::*;

    #[test]
    fn keeps_an_answer_of_100_000_characters_as_it_is() {
        // -- Setup & Fixtures
        let answer = CallToolResult::success(vec![
            ContentBlock::text("a".repeat(60_000)),
            ContentBlock::text("b".repeat(40_000)),
        ]);

        // -- Exec
        let cut_answer = cut(answer.clone());

        // -- Check
        assert_eq!(cut_answer, answer);
    }

    #[test]
    fn cuts_an_answer_with_more_than_one_text_block_at_100_000_characters_in_total() {
        // -- Setup & Fixtures
        let answer = CallToolResult::success(vec![
            ContentBlock::text("a".repeat(60_000)),
            ContentBlock::text("b".repeat(60_000)),
            ContentBlock::text("c".repeat(10)),
        ]);

        // -- Exec
        let cut_answer = cut(answer);

        // -- Check
        let texts: Vec<&str> = cut_answer
            .content
            .iter()
            .filter_map(ContentBlock::as_text)
            .map(|block| block.text.as_str())
            .collect();
        let expected_end = format!(
            "{}\n\n(Cut at 100,000 of 120,010 characters.)",
            "b".repeat(40_000)
        );
        assert_eq!(texts, ["a".repeat(60_000), expected_end]);
    }

    #[test]
    fn puts_the_note_on_a_block_that_ends_at_100_000_characters_and_leaves_out_the_rest() {
        // -- Setup & Fixtures
        let answer = CallToolResult::success(vec![
            ContentBlock::text("a".repeat(100_000)),
            ContentBlock::image("aW1hZ2U=", "image/png"),
            ContentBlock::text("b"),
        ]);

        // -- Exec
        let cut_answer = cut(answer);

        // -- Check
        let expected = format!(
            "{}\n\n(Cut at 100,000 of 100,001 characters.)",
            "a".repeat(100_000)
        );
        assert_eq!(
            cut_answer.content,
            [ContentBlock::text(expected)],
            "the image and the text after the cut must be left out"
        );
    }

    #[test]
    fn counts_and_cuts_characters_not_bytes() {
        // -- Setup & Fixtures
        let answer = CallToolResult::success(vec![ContentBlock::text("é".repeat(100_001))]);

        // -- Exec
        let cut_answer = cut(answer);

        // -- Check
        let text = &cut_answer.content[0].as_text().unwrap().text;
        let expected = format!(
            "{}\n\n(Cut at 100,000 of 100,001 characters.)",
            "é".repeat(100_000)
        );
        assert_eq!(*text, expected);
    }

    #[test]
    fn keeps_the_error_flag_of_a_cut_answer() {
        // -- Setup & Fixtures
        let answer = crate::server::answer(Err::<String, _>("a".repeat(100_001)));

        // -- Exec
        let cut_answer = cut(answer);

        // -- Check
        assert_eq!(cut_answer.is_error, Some(true));
    }
}

// endregion: --- Tests
