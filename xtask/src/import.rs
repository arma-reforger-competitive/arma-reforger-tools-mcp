//! `cargo xtask import`: rebuilds the Knowledge base from the Workbench docs and the wiki export.

use std::env;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::doxygen::{self, ApiFolder};
use crate::knowledge::{self, PageFile};
use crate::wiki_export;
use crate::wikitext;
use crate::{Error, Result};

/// The environment variable with the docs folder of Arma Reforger Tools (ADR 0007).
const DOCS_VAR: &str = "REFORGER_WORKBENCH_DOCS";

/// Runs the import, or with `export_list` only prints how to renew `export.xml`.
pub fn exec(export_list: bool) -> Result<()> {
    let repo = repo_root();
    let export = repo.join("src/wiki/source/export.xml");
    if export_list {
        return print_export_steps(&export);
    }
    let docs = env::var_os(DOCS_VAR)
        .map(PathBuf::from)
        .ok_or(Error::DocsPathNotSet)?;
    let enfusion = ApiFolder::enfusion(&docs);
    let arma = ApiFolder::arma(&docs);

    // -- Wiki and engine pages
    let mut pages = wiki_pages(&export)?;
    let wiki_count = pages.len();
    pages.extend(doxygen::read_engine_pages(&enfusion)?);
    let engine_count = pages.len() - wiki_count;
    let changes = knowledge::write_pages(&repo.join("src/wiki/pages"), &pages)?;
    say(&format!(
        "Pages: {wiki_count} wiki pages and {engine_count} engine pages. \
         {} written, {} deleted, {} unchanged.",
        changes.written, changes.deleted, changes.unchanged
    ));

    // -- API classes
    let mut lines = String::new();
    for folder in [enfusion, arma] {
        let classes = doxygen::read_classes(&folder)?;
        say(&format!(
            "API classes from {}: {}",
            folder.public_name,
            classes.len()
        ));
        for class in classes {
            lines.push_str(&serde_json::to_string(&class)?);
            lines.push('\n');
        }
    }
    let classes_path = repo.join("src/api-docs/classes.jsonl");
    let written = knowledge::write_if_changed(&classes_path, &lines)?;
    say(&format!(
        "{}: {}.",
        classes_path.display(),
        if written { "written" } else { "unchanged" }
    ));
    Ok(())
}

// region:    --- Support

/// The xtask crate is one folder below the repo root.
fn repo_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest.parent().unwrap_or(manifest).to_owned()
}

/// Converts the Arma Reforger pages of the export. A page with no text left, for example a
/// placeholder that holds only an editor note, is left out.
fn wiki_pages(export: &Path) -> Result<Vec<PageFile>> {
    let mut pages = Vec::new();
    for page in wiki_export::reforger_pages(export)? {
        let markdown = wikitext::to_markdown(&page.wikitext).map_err(|source| Error::Wikitext {
            title: page.title.clone(),
            source,
        })?;
        if markdown.trim().is_empty() {
            continue;
        }
        let url = wikitext::page_url(&page.title);
        let name = page.title.replace(' ', "_");
        pages.push(PageFile::new(page.title, url, &name, &markdown));
    }
    Ok(pages)
}

/// The wiki answers 403 to scripts, so a person saves `export.xml` in a browser.
fn print_export_steps(export: &Path) -> Result<()> {
    let titles = wiki_export::reforger_titles(export)?;
    say(&format!(
        "To renew {}:\n\
         1. Open https://community.bistudio.com/wiki/Special:Export in a browser.\n\
         2. Paste the {} titles below into the page list. To get new pages too, also add the \
            category \"Arma Reforger\" with \"Add pages from category\".\n\
         3. Turn on \"Include only the current revision\" and \"Save as file\", then click Export.\n\
         4. Save the file as {}, then run `cargo xtask import`.\n",
        export.display(),
        titles.len(),
        export.display()
    ));
    say(&titles.join("\n"));
    Ok(())
}

fn say(text: &str) {
    let _ = writeln!(std::io::stdout(), "{text}");
}

// endregion: --- Support
