//! Copies the Handler script into a mod project. The bridge writes only inside its own folder,
//! `Scripts/WorkbenchGame/ReforgerBridge/`, and never changes a `.gproj` file.

use std::fs;
use std::path::{Path, PathBuf};

use super::{Error, Result};

/// The Handler script: one file with one version constant. Later Workbench Tools add their
/// actions to it.
pub(super) const HANDLER_SCRIPT: &str = include_str!("ReforgerBridge.c");

/// The NET API function of the Handler script: its class name.
pub(super) const HANDLER_FUNC: &str = "ReforgerBridge";

/// The folder of the bridge in a mod project. Workbench compiles `Scripts/WorkbenchGame` only in
/// Workbench, not in the game.
pub(super) const HANDLER_FOLDER: &str = "Scripts/WorkbenchGame/ReforgerBridge";

const HANDLER_FILE: &str = "ReforgerBridge.c";

/// Writes the Handler script into the project folder `project`, unless the same text is already
/// there, and returns the path of the file. It writes a temp file in the same folder first, then
/// renames it, so a stopped call never leaves half a file.
pub(super) fn install(project: &Path) -> Result<PathBuf> {
    let folder = HANDLER_FOLDER
        .split('/')
        .fold(project.to_owned(), |path, part| path.join(part));
    let file = folder.join(HANDLER_FILE);
    if fs::read_to_string(&file).is_ok_and(|text| text == HANDLER_SCRIPT) {
        return Ok(file);
    }
    let failed = |err| Error::InstallFailed {
        folder: folder.clone(),
        err,
    };
    fs::create_dir_all(&folder).map_err(failed)?;
    let temp = folder.join(format!("{HANDLER_FILE}.tmp"));
    fs::write(&temp, HANDLER_SCRIPT).map_err(failed)?;
    fs::rename(&temp, &file).map_err(failed)?;
    Ok(file)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_the_version_of_the_server_in_the_handler_script() {
        // -- Exec & Check
        let line = format!(
            "static const string VERSION = \"{}\";",
            env!("CARGO_PKG_VERSION")
        );
        assert!(
            HANDLER_SCRIPT.contains(&line),
            "set VERSION in ReforgerBridge.c to the version in Cargo.toml: {line}"
        );
    }
}

// endregion: --- Tests
