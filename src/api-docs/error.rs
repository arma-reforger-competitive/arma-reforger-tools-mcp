//! Errors of the api-docs Capability.

use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// A line of `classes.jsonl` is not a valid class.
    BadClassLine {
        line: usize,
        error: serde_json::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadClassLine { line, error } => write!(
                f,
                "Line {line} of src/api-docs/classes.jsonl is not a valid class ({error}). Run `cargo xtask import` again."
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadClassLine { error, .. } => Some(error),
        }
    }
}
