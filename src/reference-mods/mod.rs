#![doc = include_str!("README.md")]

// region:    --- Modules

mod error;
mod plan;
mod sync;
mod tools;

use error::{Error, Result};
pub use tools::tools;

// endregion: --- Modules
