//! What the Doxygen readers share: reading an HTML file, CSS selectors, the docs URL, and the tree
//! rows of `hierarchy.html` and `pages.html`.

use std::fs;
use std::path::Path;

use scraper::{Html, Selector};

use crate::{Error, Result};

/// The Workbench docs folders are on the wiki server too, so each file has a public URL.
pub const DOCS_URL: &str = "https://community.bistudio.com/wikidata/external-data/arma-reforger/";

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

/// The parent of a tree row: `row_3_0_` gives `row_3_`. A top row such as `row_3_` has no parent.
pub fn parent_row(row: &str) -> Option<String> {
    let trimmed = row.strip_suffix('_')?;
    let (parent, _) = trimmed.rsplit_once('_')?;
    (parent != "row").then(|| format!("{parent}_"))
}
