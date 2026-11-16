//! The files of the Knowledge base: one Markdown file per page with a short header, and the writer
//! that changes only the files whose content changed.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::{Error, Result};

/// One wiki page or engine page, ready to write.
pub struct PageFile {
    pub title: String,
    pub url: String,

    /// The path under the pages folder, for example `arma_reforger/server_config.md`.
    pub path: String,
    pub markdown: String,
}

/// How many files a write changed.
#[derive(Default)]
pub struct Changes {
    pub written: usize,
    pub deleted: usize,
    pub unchanged: usize,
}

impl PageFile {
    /// `name` is the end of the page URL, for example `Arma_Reforger:Server_Config`.
    pub fn new(title: String, url: String, name: &str, markdown: &str) -> Self {
        Self {
            path: page_path(name),
            title,
            url,
            markdown: markdown.to_owned(),
        }
    }

    /// The file text: a header with the title and the URL, then the Markdown.
    pub fn text(&self) -> String {
        format!(
            "---\ntitle: {}\nurl: {}\n---\n\n{}",
            self.title, self.url, self.markdown
        )
    }
}

/// Writes the pages under `dir`, and deletes the Markdown files of pages that are gone.
/// A file whose text did not change is not written, so a second run changes no file.
pub fn write_pages(dir: &Path, pages: &[PageFile]) -> Result<Changes> {
    let mut wanted: BTreeMap<&str, &PageFile> = BTreeMap::new();
    for page in pages {
        if let Some(first) = wanted.insert(&page.path, page) {
            return Err(Error::SamePath {
                path: page.path.clone(),
                first: first.title.clone(),
                second: page.title.clone(),
            });
        }
    }
    let mut changes = Changes::default();
    for old in markdown_files(dir)? {
        let relative = old
            .strip_prefix(dir)
            .map_or_default(|path| path.to_string_lossy().replace('\\', "/"));
        if !wanted.contains_key(relative.as_str()) {
            fs::remove_file(&old).map_err(|source| Error::Write { path: old, source })?;
            changes.deleted += 1;
        }
    }
    for (path, page) in wanted {
        let path = dir.join(path);
        if write_if_changed(&path, &page.text())? {
            changes.written += 1;
        } else {
            changes.unchanged += 1;
        }
    }
    remove_empty_dirs(dir)?;
    Ok(changes)
}

/// Writes `text` to `path` when the file is missing or holds another text. Gives `true` when it wrote.
pub fn write_if_changed(path: &Path, text: &str) -> Result<bool> {
    if fs::read_to_string(path).is_ok_and(|old| old == text) {
        return Ok(false);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| Error::Write {
            path: parent.to_owned(),
            source,
        })?;
    }
    fs::write(path, text).map_err(|source| Error::Write {
        path: path.to_owned(),
        source,
    })?;
    Ok(true)
}

/// Makes the file path of a page from the end of its URL: lowercase, the first `:` becomes a folder,
/// and every other sign outside `a-z 0-9 _ - /` becomes `_`.
/// `Arma_Reforger:Server_Config` gives `arma_reforger/server_config.md`.
pub fn page_path(name: &str) -> String {
    let lower = name.to_lowercase().replacen(':', "/", 1);
    let mapped: String = lower
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '-' | '/') {
                c
            } else {
                '_'
            }
        })
        .collect();
    let segments: Vec<String> = mapped
        .split('/')
        .map(|segment| {
            segment
                .split('_')
                .filter(|part| !part.is_empty())
                .collect::<Vec<_>>()
                .join("_")
        })
        .filter(|segment| !segment.is_empty())
        .collect();
    format!("{}.md", segments.join("/"))
}

// region:    --- Support

fn markdown_files(dir: &Path) -> Result<BTreeSet<PathBuf>> {
    let mut files = BTreeSet::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(files);
    };
    for entry in entries {
        let entry = entry.map_err(|source| Error::Read {
            path: dir.to_owned(),
            source,
        })?;
        let path = entry.path();
        // `file_type` does not follow a symlink, so the walk never leaves `dir`.
        let is_dir = entry
            .file_type()
            .map_err(|source| Error::Read {
                path: path.clone(),
                source,
            })?
            .is_dir();
        if is_dir {
            files.extend(markdown_files(&path)?);
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.insert(path);
        }
    }
    Ok(files)
}

fn remove_empty_dirs(dir: &Path) -> Result<()> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
            remove_empty_dirs(&path)?;
            if fs::read_dir(&path).is_ok_and(|mut rest| rest.next().is_none()) {
                fs::remove_dir(&path).map_err(|source| Error::Write { path, source })?;
            }
        }
    }
    Ok(())
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn makes_the_file_path_from_the_end_of_the_url() {
        // -- Exec & Check
        for (name, expected) in [
            (
                "Arma_Reforger:Server_Config",
                "arma_reforger/server_config.md",
            ),
            (
                "Arma_Reforger:Audio:_SCR_SoundManagerModule",
                "arma_reforger/audio_scr_soundmanagermodule.md",
            ),
            (
                "Arma_Reforger:Modding/Fire_Geometry_(LOD)",
                "arma_reforger/modding/fire_geometry_lod.md",
            ),
            (
                "EnfusionScriptAPIPublic/Page_WorldSystems",
                "enfusionscriptapipublic/page_worldsystems.md",
            ),
        ] {
            assert_eq!(page_path(name), expected, "{name}");
        }
    }

    #[test]
    fn writes_changed_pages_and_deletes_pages_that_are_gone() {
        // -- Setup & Fixtures
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/.out/writes_changed_pages_and_deletes_pages_that_are_gone");
        assert!(dir.to_string_lossy().contains("tests/.out"));
        let _ = fs::remove_dir_all(&dir);
        let page = |name: &str, markdown: &str| {
            PageFile::new(name.to_owned(), format!("https://x/{name}"), name, markdown)
        };
        write_pages(&dir, &[page("A:One", "1\n"), page("A:Two", "2\n")]).unwrap();

        // -- Exec
        let again = write_pages(&dir, &[page("A:One", "1\n"), page("A:Two", "2\n")]).unwrap();
        let changed = write_pages(&dir, &[page("A:One", "one\n")]).unwrap();

        // -- Check
        assert_eq!((again.written, again.deleted, again.unchanged), (0, 0, 2));
        assert_eq!((changed.written, changed.deleted), (1, 1));
        assert_eq!(
            fs::read_to_string(dir.join("a/one.md")).unwrap(),
            "---\ntitle: A:One\nurl: https://x/A:One\n---\n\none\n"
        );
        assert!(!dir.join("a/two.md").exists());
    }
}

// endregion: --- Tests
