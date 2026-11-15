//! Lists the Knowledge base page files, so that `include_str!` puts each of them into the server program.

use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    let pages_dir = Path::new("src/wiki/pages");
    println!("cargo::rerun-if-changed={}", pages_dir.display());
    let mut files = Vec::new();
    collect_markdown(pages_dir, &mut files)?;
    files.sort();
    let mut list = String::from("&[\n");
    for file in files {
        let relative = file
            .strip_prefix(pages_dir)
            .map_err(io::Error::other)?
            .to_string_lossy()
            .replace('\\', "/");
        let absolute = fs::canonicalize(&file)?;
        let _ = writeln!(
            list,
            "    ({relative:?}, include_str!({:?})),",
            absolute.to_string_lossy()
        );
    }
    list.push(']');
    let out_dir = PathBuf::from(
        std::env::var_os("OUT_DIR").ok_or_else(|| io::Error::other("OUT_DIR is not set"))?,
    );
    fs::write(out_dir.join("wiki_pages.rs"), list)
}

fn collect_markdown(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            collect_markdown(&path, files)?;
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    Ok(())
}
