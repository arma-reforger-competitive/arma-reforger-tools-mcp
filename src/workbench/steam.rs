//! Finds Arma Reforger Tools and the game through Steam. The Windows registry names the Steam
//! folder (`HKCU\Software\Valve\Steam`, value `SteamPath`). In it, `steamapps/libraryfolders.vdf`
//! names each library folder and the apps in it, and `steamapps/appmanifest_<app>.acf` in a
//! library names the folder of the app under `steamapps/common`.

use std::fs;
use std::path::PathBuf;

use tokio::process::Command;

use super::error::NoStart;
use super::{Error, Result};
use crate::host_path::host_path;

/// The registry key of Steam for the user.
const STEAM_KEY: &str = r"HKCU\Software\Valve\Steam";

/// The Workbench program in the folder of Arma Reforger Tools.
const WORKBENCH_EXE: &str = r"Workbench\ArmaReforgerWorkbenchSteamDiag.exe";

/// A Steam app that Workbench needs.
#[derive(Clone, Copy, Debug)]
pub(super) enum App {
    /// Arma Reforger Tools: Workbench.
    Tools,

    /// Arma Reforger: the game data that Workbench loads.
    Game,
}

/// The Workbench program and the game folder, in Windows form, for the start command.
pub(super) struct Install {
    pub(super) workbench: String,
    pub(super) game: String,
}

impl App {
    /// The Steam app ID.
    pub(super) fn id(self) -> u32 {
        match self {
            Self::Tools => 1_874_910,
            Self::Game => 1_874_880,
        }
    }

    /// The name of the app in the Steam library.
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::Tools => "Arma Reforger Tools",
            Self::Game => "Arma Reforger",
        }
    }
}

/// Finds the Workbench program and the game folder. `steam` is the Steam folder in Windows form,
/// or `None` to read it from the registry.
pub(super) async fn find_install(steam: Option<&str>) -> Result<Install> {
    let steam = match steam {
        Some(steam) => steam.to_owned(),
        None => steam_from_registry().await?,
    };
    let library = fs::read_to_string(host_path(&steam).join("steamapps/libraryfolders.vdf"))
        .map_err(|_| NoStart::NoSteam)?;
    let (tools, tools_here) = app_folder(&library, App::Tools)?;
    if !tools_here.join(WORKBENCH_EXE.replace('\\', "/")).is_file() {
        return Err(NoStart::NotInstalled { app: App::Tools }.into());
    }
    let (game, _) = app_folder(&library, App::Game)?;
    Ok(Install {
        workbench: format!(r"{tools}\{WORKBENCH_EXE}"),
        game,
    })
}

// region:    --- Support

/// The Steam folder from the registry, in Windows form.
async fn steam_from_registry() -> Result<String> {
    let output = Command::new("reg.exe")
        .args(["query", STEAM_KEY, "/v", "SteamPath"])
        .output()
        .await
        .map_err(|_| NoStart::NoSteam)?;
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .find_map(|line| Some(line.split_once("REG_SZ")?.1.trim().to_owned()))
        .filter(|path| !path.is_empty())
        .ok_or_else(|| NoStart::NoSteam.into())
}

/// The folder of `app` in Windows form and on this computer, from the text of
/// `libraryfolders.vdf`.
fn app_folder(library: &str, app: App) -> Result<(String, PathBuf)> {
    let not_installed = || Error::from(NoStart::NotInstalled { app });
    let id = app.id().to_string();
    let mut path = None;
    let mut found = None;
    for (key, value) in library.lines().filter_map(pair) {
        if key == "path" {
            path = Some(value);
        } else if key == id {
            found = path.take();
            break;
        }
    }
    let root = found.ok_or_else(not_installed)?;
    let manifest = fs::read_to_string(
        host_path(&root).join(format!("steamapps/appmanifest_{}.acf", app.id())),
    )
    .map_err(|_| not_installed())?;
    let folder = manifest
        .lines()
        .filter_map(pair)
        .find_map(|(key, value)| (key == "installdir").then_some(value))
        .ok_or_else(not_installed)?;
    let here = host_path(&root).join("steamapps/common").join(&folder);
    if !here.is_dir() {
        return Err(not_installed());
    }
    Ok((format!(r"{root}\steamapps\common\{folder}"), here))
}

/// The key and the value of a line `"key"  "value"` in a Steam file. A `\\` in the value is one
/// `\`.
fn pair(line: &str) -> Option<(String, String)> {
    let mut parts = line.split('"').skip(1).step_by(2);
    let key = parts.next()?;
    let value = parts.next()?;
    Some((key.to_owned(), value.replace(r"\\", r"\")))
}

/// The installed apps of a Steam folder that tests write, in the formats that Steam writes.
#[cfg(test)]
pub(super) fn write_steam(steam: &std::path::Path, apps: &[App]) -> PathBuf {
    use std::fmt::Write as _;

    let mut library = format!(
        "\"libraryfolders\"\n{{\n\t\"0\"\n\t{{\n\t\t\"path\"\t\t\"{}\"\n\t\t\"apps\"\n\t\t{{\n",
        steam.display().to_string().replace('\\', r"\\")
    );
    for app in apps {
        let _ = writeln!(library, "\t\t\t\"{}\"\t\t\"1000\"", app.id());
        let folder = steam.join("steamapps/common").join(app.name());
        fs::create_dir_all(folder.join("Workbench")).unwrap();
        fs::write(
            steam.join(format!("steamapps/appmanifest_{}.acf", app.id())),
            format!(
                "\"AppState\"\n{{\n\t\"appid\"\t\t\"{}\"\n\t\"name\"\t\t\"{}\"\n\t\"installdir\"\t\t\"{}\"\n}}\n",
                app.id(),
                app.name(),
                app.name()
            ),
        )
        .unwrap();
        if let App::Tools = app {
            fs::write(
                folder.join("Workbench/ArmaReforgerWorkbenchSteamDiag.exe"),
                "",
            )
            .unwrap();
        }
    }
    library.push_str("\t\t}\n\t}\n}\n");
    fs::write(steam.join("steamapps/libraryfolders.vdf"), library).unwrap();
    steam.to_owned()
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_library_file_that_steam_wrote() {
        // -- Setup & Fixtures
        // The start of libraryfolders.vdf of Steam 2026 with the library D:\Games.
        let library = "\"libraryfolders\"\n{\n\t\"0\"\n\t{\n\t\t\"path\"\t\t\"D:\\\\Games\"\n\t\t\"apps\"\n\t\t{\n\t\t\t\"1874910\"\t\t\"725716903\"\n";

        // -- Exec & Check
        assert_eq!(
            library.lines().filter_map(pair).collect::<Vec<_>>(),
            [
                ("path".to_owned(), r"D:\Games".to_owned()),
                ("1874910".to_owned(), "725716903".to_owned())
            ]
        );
    }
}

// endregion: --- Tests
