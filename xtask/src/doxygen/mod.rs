//! Reads the Doxygen HTML docs that ship with Arma Reforger Tools: engine pages and API classes.

// region:    --- Modules

mod classes;
mod engine_pages;
mod html;
mod markdown;

pub use classes::{ApiFolder, read as read_classes};
pub use engine_pages::read as read_engine_pages;

use html::{read_html, selector};

// endregion: --- Modules
