#![doc = include_str!("README.md")]

// region:    --- Modules

mod error;
mod pages;
mod read;

pub use error::{Error, Result};
pub use read::tools;

// endregion: --- Modules
