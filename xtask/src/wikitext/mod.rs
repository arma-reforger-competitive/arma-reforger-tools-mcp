//! Converts `MediaWiki` wikitext from the wiki export to Markdown.

// region:    --- Modules

mod blocks;
mod convert;
mod error;
mod inline;
mod stash;
mod tables;
mod templates;

pub use convert::to_markdown;
pub use error::{Error, Result};
pub use templates::page_url;

// endregion: --- Modules
