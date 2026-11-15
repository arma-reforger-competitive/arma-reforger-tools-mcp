#![doc = include_str!("README.md")]

// region:    --- Modules

mod error;
mod pages;
mod read;
mod search;
mod sections;
mod tools;

pub use error::{Error, Result};
pub use tools::tools;

// endregion: --- Modules
