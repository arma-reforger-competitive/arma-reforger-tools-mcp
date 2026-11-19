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

    #[test]
    fn handler_script_knows_the_actions_and_fields_of_reference_sync() {
        // -- Setup & Fixtures
        // The actions and the fields that `reference_sync` uses (see src/reference-mods/README.md).
        let actions = ["\"reference_download\"", "\"reference_progress\""];
        let fields = [
            "id",
            "state",
            "percent",
            "name",
            "mod_version",
            "license",
            "reason",
        ];

        // -- Exec & Check
        for action in actions {
            assert!(
                HANDLER_SCRIPT.contains(&format!("req.action == {action}")),
                "the Handler script has no branch for the action {action}"
            );
        }
        for field in fields {
            assert!(
                HANDLER_SCRIPT.contains(&format!("RegV(\"{field}\");")),
                "the Handler script does not register the field \"{field}\""
            );
        }
        for state in ["searching", "downloading", "complete", "failed", "unknown"] {
            assert!(
                HANDLER_SCRIPT.contains(&format!("\"{state}\"")),
                "the Handler script never answers the state \"{state}\""
            );
        }
    }

    #[test]
    fn handler_script_starts_a_download_again_unless_it_searches_or_downloads() {
        // -- Setup & Fixtures
        let start = HANDLER_SCRIPT
            .split_once("static void Start(string id)")
            .and_then(|(_, rest)| rest.split_once("static void SearchEnded()"))
            .map(|(start, _)| start)
            .unwrap();

        // -- Exec & Check
        assert!(
            start.contains("if (download && download.IsRunning())\n\t\t\treturn;"),
            "Start must skip only a download that runs: {start}"
        );
        assert!(
            HANDLER_SCRIPT.contains(
                "bool IsRunning()\n\t{\n\t\treturn m_State == \"searching\" || m_State == \"downloading\";"
            ),
            "IsRunning must be true only while the download searches or downloads"
        );
    }
}

// endregion: --- Tests
