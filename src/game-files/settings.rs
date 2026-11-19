//! The settings of the game-files Capability, read from environment variables (ADR 0007). A
//! folder path works in Windows form and in WSL form (ADR 0006): see `crate::host_path`.

use std::path::PathBuf;

use crate::workbench_documents::{DOCUMENTS_SETTING, workbench_documents};

/// The game folder, the folder that holds `ArmaReforger.exe` and `addons`.
pub(super) const GAME_SETTING: &str = "REFORGER_GAME_PATH";

/// A second folder of Workshop mods, for example `D:\DownloadedMods`. game-files reads the mods in
/// its `addons` folder and in the `addons` folder of the Workbench folder in Documents.
pub(super) const PROFILE_SETTING: &str = "REFORGER_WORKBENCH_PROFILE_PATH";

/// The raw values of the settings, as the user wrote them. An empty value counts as not set.
#[derive(Clone, Default)]
pub(super) struct Settings {
    pub(super) game: Option<String>,
    pub(super) profile: Option<String>,

    /// The Workbench folder in Documents, where Workbench downloads mods to its `addons` folder.
    /// `None`: no setting, and no such folder found.
    pub(super) documents: Option<PathBuf>,
}

impl Settings {
    /// Reads the settings from the environment of the server.
    pub(super) fn from_env() -> Self {
        let read = |name| std::env::var(name).ok().filter(|v| !v.trim().is_empty());
        Self {
            game: read(GAME_SETTING),
            profile: read(PROFILE_SETTING),
            documents: workbench_documents(read(DOCUMENTS_SETTING).as_deref()),
        }
    }
}
