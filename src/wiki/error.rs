//! Errors of the wiki Capability.

use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// A page file does not start with the `---` header that holds its title and URL.
    BadPageHeader { path: &'static str },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadPageHeader { path } => write!(
                f,
                "src/wiki/pages/{path} has no valid header with `title:` and `url:`. Run `cargo xtask import` again."
            ),
        }
    }
}

impl std::error::Error for Error {}
