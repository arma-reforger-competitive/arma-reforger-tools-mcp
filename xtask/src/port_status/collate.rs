//! The order of feature names in the ledger: the root order of the Unicode Collation Algorithm,
//! which the JavaScript `localeCompare` uses, for ASCII text.

use std::cmp::Ordering;

/// The ASCII characters that are not letters or digits, in the root order. All of them come
/// before the digits, and the digits come before the letters.
const SYMBOLS: &str = " _-,;:!?.'\"()[]{}@*/\\&#%`^+<=>|~$";

/// Compares two names in the root order. First the letters without case, the symbols, and the
/// digits; when they are all the same, a lowercase letter comes before its uppercase letter.
pub fn compare(a: &str, b: &str) -> Ordering {
    let primary = |text: &str| text.chars().map(weight).collect::<Vec<_>>();
    let case = |text: &str| {
        text.chars()
            .map(|c| c.is_ascii_uppercase())
            .collect::<Vec<_>>()
    };
    primary(a)
        .cmp(&primary(b))
        .then_with(|| case(a).cmp(&case(b)))
        .then_with(|| a.cmp(b))
}

// region:    --- Support

/// The first-level weight of one character. A character outside ASCII comes after the letters,
/// in code point order.
fn weight(c: char) -> u32 {
    const DIGITS: u32 = 100;
    const LETTERS: u32 = 200;
    const OTHERS: u32 = 300;
    let lower = c.to_ascii_lowercase();
    if let Some(index) = SYMBOLS.find(c) {
        u32::try_from(index).unwrap_or(DIGITS)
    } else if c.is_ascii_digit() {
        DIGITS + u32::from(c) - u32::from('0')
    } else if lower.is_ascii_lowercase() {
        LETTERS + u32::from(lower) - u32::from('a')
    } else {
        OTHERS + u32::from(c)
    }
}

// endregion: --- Support
