//! Reads the engine pages (`Page_*.html`) of the Enfusion script API docs as Markdown pages.

use std::collections::HashMap;
use std::path::Path;

use scraper::Html;

use super::markdown;
use super::{read_html, selector};
use crate::knowledge::PageFile;
use crate::{Error, Result};

/// The same files are on the wiki server, so each engine page has a public URL.
const ENGINE_URL: &str =
    "https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/";

/// Reads every page that `pages.html` lists. A page under another page gets the other page's title
/// in front of its own, because titles like "Syntax" or "Features" say little alone.
pub fn read(html_dir: &Path) -> Result<Vec<PageFile>> {
    let index_path = html_dir.join("pages.html");
    let entries = page_list(&read_html(&index_path)?)?;
    if entries.is_empty() {
        return Err(Error::NoEnginePages { path: index_path });
    }
    let titles: HashMap<&str, &str> = entries
        .iter()
        .map(|entry| (entry.row.as_str(), entry.title.as_str()))
        .collect();
    let mut pages = Vec::new();
    for entry in &entries {
        let title = match parent_row(&entry.row).and_then(|parent| titles.get(parent.as_str())) {
            Some(parent) => format!("{parent}: {}", entry.title),
            None => entry.title.clone(),
        };
        let document = read_html(&html_dir.join(&entry.file))?;
        let content = document
            .select(&selector("div.contents div.textblock")?)
            .next()
            .map_or_default(|block| markdown::blocks(block, 1));
        let name = format!(
            "EnfusionScriptAPIPublic/{}",
            entry.file.trim_end_matches(".html")
        );
        let url = format!("{ENGINE_URL}{}", entry.file);
        pages.push(PageFile::new(title, url, &name, &format!("{content}\n")));
    }
    Ok(pages)
}

// region:    --- Support

struct Entry {
    /// The row id in the page tree, for example `row_3_0_` for the first page under `row_3_`.
    row: String,
    file: String,
    title: String,
}

fn page_list(index: &Html) -> Result<Vec<Entry>> {
    let link = selector("td.entry a.el")?;
    let mut entries = Vec::new();
    for row in index.select(&selector("tr[id^=\"row_\"]")?) {
        let Some(anchor) = row.select(&link).next() else {
            continue;
        };
        let file = anchor.value().attr("href").unwrap_or_default();
        if !file.starts_with("Page_") {
            continue;
        }
        entries.push(Entry {
            row: row.value().attr("id").unwrap_or_default().to_owned(),
            file: file.to_owned(),
            title: markdown::text(anchor),
        });
    }
    Ok(entries)
}

/// `row_3_0_` gives `row_3_`. A top row such as `row_3_` has no parent.
fn parent_row(row: &str) -> Option<String> {
    let trimmed = row.strip_suffix('_')?;
    let (parent, _) = trimmed.rsplit_once('_')?;
    (parent != "row").then(|| format!("{parent}_"))
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_parent_row_of_a_page() {
        // -- Exec & Check
        assert_eq!(parent_row("row_3_0_").as_deref(), Some("row_3_"));
        assert_eq!(parent_row("row_10_2_1_").as_deref(), Some("row_10_2_"));
        assert_eq!(parent_row("row_3_"), None);
    }
}

// endregion: --- Tests
