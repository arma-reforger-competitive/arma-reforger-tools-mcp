//! Reads the engine pages (`Page_*.html`) of the Enfusion script API docs as Markdown pages.

use std::collections::HashMap;

use scraper::Html;

use super::markdown;
use super::{ApiFolder, parent_row, read_html, selector};
use crate::knowledge::PageFile;
use crate::{Error, Result};

/// Reads every page that `pages.html` of `folder` lists. A page under another page gets the other
/// page's title in front of its own, because titles like "Syntax" or "Features" say little alone.
pub fn read(folder: &ApiFolder) -> Result<Vec<PageFile>> {
    let html_dir = &folder.html_dir;
    let index_path = html_dir.join("pages.html");
    let entries = page_list(&read_html(&index_path)?)?;
    if entries.is_empty() {
        return Err(Error::NoEnginePages { path: index_path });
    }
    let titles: HashMap<&str, &str> = entries
        .iter()
        .map(|entry| (entry.row.as_str(), entry.title.as_str()))
        .collect();
    let content_selector = selector("div.contents div.textblock")?;
    let mut pages = Vec::new();
    for entry in &entries {
        let title = match parent_row(&entry.row).and_then(|parent| titles.get(parent.as_str())) {
            Some(parent) => format!("{parent}: {}", entry.title),
            None => entry.title.clone(),
        };
        let document = read_html(&html_dir.join(&entry.file))?;
        let content = document
            .select(&content_selector)
            .next()
            .map_or_default(|block| markdown::blocks(block, 1));
        let name = format!(
            "{}/{}",
            folder.public_name,
            entry.file.trim_end_matches(".html")
        );
        let url = folder.url(&entry.file);
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

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use super::*;

    #[test]
    fn puts_the_title_of_the_page_above_in_front_of_a_page_title() {
        // -- Setup & Fixtures
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/.out/puts_the_title_of_the_page_above_in_front_of_a_page_title");
        assert!(dir.to_string_lossy().contains("tests/.out"));
        let _ = fs::remove_dir_all(&dir);
        let html_dir = dir.join("EnfusionScriptAPI/html");
        fs::create_dir_all(&html_dir).unwrap();
        let row = |id: &str, file: &str, title: &str| {
            format!(
                "<tr id=\"{id}\"><td class=\"entry\"><a class=\"el\" href=\"{file}\">{title}</a></td></tr>"
            )
        };
        fs::write(
            html_dir.join("pages.html"),
            format!(
                "<table>{}{}</table>",
                row("row_0_", "Page_Replication.html", "Replication"),
                row("row_0_0_", "Page_RplNode.html", "RplNode")
            ),
        )
        .unwrap();
        for (file, text) in [
            ("Page_Replication.html", "Sends state."),
            ("Page_RplNode.html", "A node."),
        ] {
            fs::write(
                html_dir.join(file),
                format!(
                    "<div class=\"contents\"><div class=\"textblock\"><p>{text}</p></div></div>"
                ),
            )
            .unwrap();
        }

        // -- Exec
        let pages = read(&ApiFolder::enfusion(&dir)).unwrap();

        // -- Check
        let titles: Vec<(&str, &str)> = pages
            .iter()
            .map(|page| (page.title.as_str(), page.path.as_str()))
            .collect();
        assert_eq!(
            titles,
            [
                ("Replication", "enfusionscriptapipublic/page_replication.md"),
                (
                    "Replication: RplNode",
                    "enfusionscriptapipublic/page_rplnode.md"
                ),
            ]
        );
        assert_eq!(
            pages[1].url,
            "https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_RplNode.html"
        );
    }
}

// endregion: --- Tests
