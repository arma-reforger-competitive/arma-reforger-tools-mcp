//! Errors of the guid Capability.

use std::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Externals
    /// The operating system gave no random bytes.
    NoRandomBytes(getrandom::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRandomBytes(error) => write!(
                f,
                "The operating system gave no random bytes for the GUID ({error}). Call `guid_generate` again."
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::NoRandomBytes(error) => Some(error),
        }
    }
}
