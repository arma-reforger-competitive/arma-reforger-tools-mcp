//! The Workbench folder in Documents, the folder that holds `profile`, `logs`, and `addons`. Every
//! Capability that needs it finds it here, the same way. The path works in Windows form and in
//! WSL form (ADR 0006).

use std::path::PathBuf;

use crate::host_path::host_path;
use crate::wsl::is_wsl;

/// The setting of the Workbench folder in Documents.
pub const DOCUMENTS_SETTING: &str = "REFORGER_WORKBENCH_DOCUMENTS_PATH";

/// The Workbench folder in Documents, under the user's folder on Windows.
const DOCUMENTS_IN_USER: &str = "Documents/My Games/ArmaReforgerWorkbench";

/// The Workbench folder in Documents from the raw value of `DOCUMENTS_SETTING`. Without the
/// setting, it is the standard folder of the Windows user. Under WSL, that is the one folder in
/// `/mnt/c/Users` that has it. `None`: no setting, and no such folder found.
pub fn workbench_documents(setting: Option<&str>) -> Option<PathBuf> {
    if let Some(raw) = setting {
        return Some(host_path(raw));
    }
    if cfg!(windows) {
        let user = std::env::var_os("USERPROFILE")?;
        return Some(PathBuf::from(user).join(DOCUMENTS_IN_USER));
    }
    if !is_wsl() {
        return None;
    }
    let mut found = std::fs::read_dir("/mnt/c/Users")
        .ok()?
        .filter_map(|user| Some(user.ok()?.path().join(DOCUMENTS_IN_USER)))
        .filter(|folder| folder.join("profile").is_dir());
    match (found.next(), found.next()) {
        (Some(folder), None) => Some(folder),
        _ => None,
    }
}

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(windows))]
    fn uses_the_setting_in_windows_form() {
        // -- Exec & Check
        assert_eq!(
            workbench_documents(Some(
                r"C:\Users\me\Documents\My Games\ArmaReforgerWorkbench"
            )),
            Some(PathBuf::from(
                "/mnt/c/Users/me/Documents/My Games/ArmaReforgerWorkbench"
            ))
        );
    }
}

// endregion: --- Tests
