#![doc = include_str!("README.md")]

// region:    --- Modules

mod classes;
mod error;
mod search;
mod tools;

pub use error::{Error, Result};
pub use tools::tools;

// endregion: --- Modules
