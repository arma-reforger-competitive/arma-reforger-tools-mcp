#![doc = include_str!("README.md")]

// region:    --- Modules

mod error;
mod find;
mod index;
mod pak;
mod read;
mod settings;
mod tools;

use error::{Error, Result};
pub use tools::tools;

#[cfg(test)]
mod test_pak;

// endregion: --- Modules
