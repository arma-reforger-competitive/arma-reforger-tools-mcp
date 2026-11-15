//! Reads an HTML file and makes CSS selectors.

use std::fs;
use std::path::Path;

use scraper::{Html, Selector};

use crate::{Error, Result};

/// Reads an HTML file. A few Doxygen files are in Latin-1, not UTF-8: each of their bytes is one character.
pub fn read_html(path: &Path) -> Result<Html> {
    let bytes = fs::read(path).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })?;
    let text = String::from_utf8(bytes)
        .unwrap_or_else(|err| err.into_bytes().into_iter().map(char::from).collect());
    Ok(Html::parse_document(&text))
}

pub fn selector(css: &'static str) -> Result<Selector> {
    Selector::parse(css).map_err(|_| Error::BadSelector(css))
}
