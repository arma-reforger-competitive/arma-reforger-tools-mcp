//! Errors of the wikitext converter.

use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The page uses a template that the converter does not know.
    UnknownTemplate(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownTemplate(name) => write!(
                f,
                "The template {{{{{name}}}}} is unknown. Add it to `render` in xtask/src/wikitext/templates.rs."
            ),
        }
    }
}

impl std::error::Error for Error {}
