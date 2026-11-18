//! The settings of the game-files Capability, read from environment variables (ADR 0007). A
//! folder path works in Windows form and in WSL form (ADR 0006): see `crate::host_path`.

/// The game folder, the folder that holds `ArmaReforger.exe` and `addons`.
pub(super) const GAME_SETTING: &str = "REFORGER_GAME_PATH";

/// The Workbench profile folder. Its `addons` folder holds the Workshop mods that Workbench
/// downloaded.
pub(super) const PROFILE_SETTING: &str = "REFORGER_WORKBENCH_PROFILE_PATH";

/// The raw values of the settings, as the user wrote them. An empty value counts as not set.
#[derive(Clone, Default)]
pub(super) struct Settings {
    pub(super) game: Option<String>,
    pub(super) profile: Option<String>,
}

impl Settings {
    /// Reads the settings from the environment of the server.
    pub(super) fn from_env() -> Self {
        let read = |name| std::env::var(name).ok().filter(|v| !v.trim().is_empty());
        Self {
            game: read(GAME_SETTING),
            profile: read(PROFILE_SETTING),
        }
    }
}
