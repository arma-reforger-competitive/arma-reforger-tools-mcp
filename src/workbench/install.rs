//! Copies the handler script into a mod project. The bridge writes only inside its own folder,
//! `Scripts/WorkbenchGame/ReforgerBridge/`, and never changes a `.gproj` file.

use std::fs;
use std::path::{Path, PathBuf};

use super::{Error, Result};

/// The handler script: one file with one version constant. Later Workbench Tools add their
/// actions to it.
pub(super) const HANDLER: &str = include_str!("ReforgerBridge.c");

/// The NET API function of the handler script: its class name.
pub(super) const HANDLER_FUNC: &str = "ReforgerBridge";

/// The folder of the bridge in a mod project. Workbench compiles `Scripts/WorkbenchGame` only in
/// Workbench, not in the game.
const HANDLER_FOLDER: [&str; 3] = ["Scripts", "WorkbenchGame", "ReforgerBridge"];

const HANDLER_FILE: &str = "ReforgerBridge.c";

/// Writes the handler script into the project folder `project`, unless the same text is already
/// there, and returns the path of the file. It writes a temp file in the same folder first, then
/// renames it, so a stopped call never leaves half a file.
pub(super) fn install(project: &Path) -> Result<PathBuf> {
    let folder = HANDLER_FOLDER
        .iter()
        .fold(project.to_owned(), |path, part| path.join(part));
    let file = folder.join(HANDLER_FILE);
    if fs::read_to_string(&file).is_ok_and(|text| text == HANDLER) {
        return Ok(file);
    }
    let failed = |err| Error::InstallFailed {
        folder: folder.clone(),
        err,
    };
    fs::create_dir_all(&folder).map_err(failed)?;
    let temp = folder.join(format!("{HANDLER_FILE}.tmp"));
    fs::write(&temp, HANDLER).map_err(failed)?;
    fs::rename(&temp, &file).map_err(failed)?;
    Ok(file)
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    /// Every file under `root` with its bytes.
    fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
        let mut files = BTreeMap::new();
        let mut folders = vec![root.to_owned()];
        while let Some(folder) = folders.pop() {
            for entry in fs::read_dir(&folder).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    folders.push(path);
                } else {
                    files.insert(
                        path.strip_prefix(root).unwrap().to_owned(),
                        fs::read(&path).unwrap(),
                    );
                }
            }
        }
        files
    }

    #[test]
    fn writes_only_the_handler_file_in_its_own_folder() {
        // -- Setup & Fixtures
        let dir = tempfile::tempdir().unwrap();
        let project = dir.path();
        fs::write(
            project.join("addon.gproj"),
            "GameProject {\n ID \"BridgeTest\"\n}",
        )
        .unwrap();
        fs::create_dir_all(project.join("Scripts/WorkbenchGame/MyPlugin")).unwrap();
        fs::write(
            project.join("Scripts/WorkbenchGame/MyPlugin/Tool.c"),
            "class Tool {}",
        )
        .unwrap();
        fs::create_dir_all(project.join("Scripts/Game")).unwrap();
        fs::write(project.join("Scripts/Game/Mode.c"), "class Mode {}").unwrap();
        let before = snapshot(project);

        // -- Exec
        let file = install(project).unwrap();

        // -- Check
        let mut after = snapshot(project);
        let handler = PathBuf::from("Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c");
        assert_eq!(file, project.join(&handler));
        assert_eq!(after.remove(&handler).as_deref(), Some(HANDLER.as_bytes()));
        assert_eq!(
            after, before,
            "the install changed a file outside its folder"
        );
    }

    #[test]
    fn replaces_an_old_handler_file() {
        // -- Setup & Fixtures
        let dir = tempfile::tempdir().unwrap();
        let folder = dir.path().join("Scripts/WorkbenchGame/ReforgerBridge");
        fs::create_dir_all(&folder).unwrap();
        fs::write(folder.join("ReforgerBridge.c"), "old").unwrap();

        // -- Exec
        install(dir.path()).unwrap();

        // -- Check
        assert_eq!(
            fs::read_to_string(folder.join("ReforgerBridge.c")).unwrap(),
            HANDLER
        );
        assert_eq!(
            fs::read_dir(&folder).unwrap().count(),
            1,
            "a temp file is left"
        );
    }

    #[test]
    fn has_the_version_of_the_server_in_the_handler_script() {
        // -- Exec & Check
        let line = format!(
            "static const string VERSION = \"{}\";",
            env!("CARGO_PKG_VERSION")
        );
        assert!(
            HANDLER.contains(&line),
            "set VERSION in ReforgerBridge.c to the version in Cargo.toml: {line}"
        );
    }
}

// endregion: --- Tests
