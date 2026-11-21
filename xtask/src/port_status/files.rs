//! Small helpers for files and text, for the ledger and the old project.

use std::fs;
use std::path::{Path, PathBuf};

use super::{Error, Result};

/// Every file under `dir`, at any depth, whose path ends with `extension`, sorted by the path
/// text. The walk does not go into a folder link, but a link to a file counts. No folder: no
/// files.
pub fn files_under(dir: &Path, extension: &str) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    if dir.exists() {
        walk(dir, &mut found)?;
    }
    let mut files: Vec<(String, PathBuf)> = found
        .into_iter()
        .map(|path| (path.to_string_lossy().into_owned(), path))
        .filter(|(text, path)| text.ends_with(extension) && path.is_file())
        .collect();
    // Sort the text, not the `PathBuf`: a `PathBuf` compares part by part, so "a-c" comes after
    // "a/b", but the text order puts it first.
    files.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(files.into_iter().map(|(_, path)| path).collect())
}

/// The paths of the files under `dir` with `extension`, relative to `dir` and without the
/// extension, for example `ui/timer_hud`.
pub fn names_under(dir: &Path, extension: &str) -> Result<Vec<String>> {
    Ok(files_under(dir, extension)?
        .iter()
        .map(|file| without_extension(&relative(dir, file)))
        .collect())
}

/// The text of `file` relative to `dir`.
pub fn relative(dir: &Path, file: &Path) -> String {
    file.strip_prefix(dir)
        .unwrap_or(file)
        .to_string_lossy()
        .into_owned()
}

/// `path` without its last extension: "a/b.json" gives "a/b". A dot in a folder name is not an
/// extension.
pub fn without_extension(path: &str) -> String {
    match path.rsplit_once('.') {
        Some((stem, extension)) if !extension.is_empty() && !extension.contains('/') => {
            stem.to_owned()
        }
        _ => path.to_owned(),
    }
}

/// The text of `file`. Bytes that are not UTF-8 become U+FFFD.
pub fn read_text(file: &Path) -> Result<String> {
    let bytes = fs::read(file).map_err(|source| Error::Read {
        path: file.to_owned(),
        source,
    })?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

// region:    --- Support

/// Adds every entry under `dir` to `found`, and goes into each folder that is not a link.
fn walk(dir: &Path, found: &mut Vec<PathBuf>) -> Result<()> {
    let read_error = |source| Error::Read {
        path: dir.to_owned(),
        source,
    };
    for entry in fs::read_dir(dir).map_err(read_error)? {
        let entry = entry.map_err(read_error)?;
        let path = entry.path();
        let is_folder = entry.file_type().map_err(read_error)?.is_dir();
        found.push(path.clone());
        if is_folder {
            walk(&path, found)?;
        }
    }
    Ok(())
}

// endregion: --- Support
