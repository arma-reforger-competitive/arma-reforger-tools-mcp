//! Finds Arma Reforger Tools and the game through Steam. The Windows registry names the Steam
//! folder (`HKCU\Software\Valve\Steam`, value `SteamPath`). In it, `steamapps/libraryfolders.vdf`
//! names each library folder and the apps in it, and `steamapps/appmanifest_<app>.acf` in a
//! library names the folder of the app under `steamapps/common`.
//!
//! Workbench needs a running Steam with a logged-in user, and it does not start Steam itself. On
//! Workbench 1.6 without Steam, it logs `SteamAPI_Init failed. Is Steam running?` and closes.
//! While a user is logged in, Steam writes a value other than 0 to `ActiveUser` in
//! `HKCU\Software\Valve\Steam\ActiveProcess`. Steam does not always write 0 when it closes, so
//! the check also needs a running `steam.exe` in `tasklist.exe`.

use std::fs;
use std::path::PathBuf;

use tokio::process::Command;

use super::error::NoStart;
use super::{Error, Result};
use crate::host_path::host_path;

/// The registry key of Steam for the user.
const STEAM_KEY: &str = r"HKCU\Software\Valve\Steam";

/// The registry key that Steam writes while it runs.
const ACTIVE_PROCESS_KEY: &str = r"HKCU\Software\Valve\Steam\ActiveProcess";

/// The Steam program in the Steam folder.
const STEAM_EXE: &str = "steam.exe";

/// The Workbench program in the folder of Arma Reforger Tools.
const WORKBENCH_EXE: &str = r"Workbench\ArmaReforgerWorkbenchSteamDiag.exe";

/// A Steam app that Workbench needs.
#[derive(Clone, Copy, Debug)]
pub(crate) enum App {
    /// Arma Reforger Tools: Workbench.
    Tools,

    /// Arma Reforger: the game data that Workbench loads.
    Game,
}

/// The Workbench program, the game folder, and the Steam program, in Windows form, for the
/// start commands.
pub(super) struct Install {
    pub(super) workbench: String,
    pub(super) game: String,
    pub(super) steam: String,
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

/// Finds the Workbench program, the game folder, and the Steam program. `steam` is the Steam folder in Windows form,
/// or `None` to read it from the registry.
pub(super) async fn find_install(steam: Option<&str>) -> Result<Install> {
    let steam = match steam {
        Some(steam) => steam.to_owned(),
        None => steam_from_registry().await?,
    };
    if !host_path(&steam).join(STEAM_EXE).is_file() {
        return Err(NoStart::NoSteam.into());
    }
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
        steam: format!(r"{steam}\{STEAM_EXE}"),
    })
}

/// True when Steam runs and a user is logged in. A missing key or value counts as no user: Steam
/// never ran. Steam does not always set `ActiveUser` back to 0 when it closes, so the process must
/// run too.
pub(super) async fn steam_logged_in() -> bool {
    let Ok(output) = Command::new("reg.exe")
        .args(["query", ACTIVE_PROCESS_KEY, "/v", "ActiveUser"])
        .output()
        .await
    else {
        return false;
    };
    if !active_user(&String::from_utf8_lossy(&output.stdout)) {
        return false;
    }
    let Ok(output) = Command::new("tasklist.exe")
        .args(["/FI", "IMAGENAME eq steam.exe", "/FO", "CSV", "/NH"])
        .output()
        .await
    else {
        return false;
    };
    steam_process(&String::from_utf8_lossy(&output.stdout))
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

/// True when the output of `reg.exe query` names an `ActiveUser` other than 0.
fn active_user(output: &str) -> bool {
    output
        .lines()
        .filter(|line| line.trim_start().starts_with("ActiveUser"))
        .find_map(|line| line.split_once("REG_DWORD")?.1.trim().strip_prefix("0x"))
        .and_then(|hex| u32::from_str_radix(hex, 16).ok())
        .is_some_and(|user| user != 0)
}

/// True when the CSV output of `tasklist.exe` has a `steam.exe` row. When no process matches,
/// `tasklist.exe` prints a message in the language of Windows, so only the row counts.
fn steam_process(output: &str) -> bool {
    output.lines().any(|line| {
        line.get(..11)
            .is_some_and(|start| start.eq_ignore_ascii_case("\"steam.exe\""))
    })
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
    fs::write(steam.join(STEAM_EXE), "").unwrap();
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

    #[test]
    fn reads_the_logged_in_user_from_the_registry() {
        // -- Setup & Fixtures
        // The output of `reg.exe query` on Windows 11 with Steam 2026, closed and logged in.
        let answer = |user: &str| {
            format!(
                "\r\nHKEY_CURRENT_USER\\Software\\Valve\\Steam\\ActiveProcess\r\n    ActiveUser    REG_DWORD    {user}\r\n\r\n"
            )
        };

        // -- Exec & Check
        assert!(active_user(&answer("0x37cfa8c0")));
        assert!(!active_user(&answer("0x0")));
        assert!(!active_user(
            "ERROR: The system was unable to find the specified registry key or value.\r\n"
        ));
    }

    #[test]
    fn finds_the_steam_process_in_the_process_list() {
        // -- Setup & Fixtures
        // The output of `tasklist.exe /FO CSV /NH` on Windows 11: a running Steam, and no match.
        let running = "\"steam.exe\",\"12345\",\"Console\",\"1\",\"80 000 K\"\r\n";
        let none = "INFO: No tasks are running which match the specified criteria.\r\n";

        // -- Exec & Check
        assert!(steam_process(running));
        assert!(!steam_process(none));
        assert!(!steam_process(
            "\"steamwebhelper.exe\",\"222\",\"Console\",\"1\",\"9 K\"\r\n"
        ));
    }
}

// endregion: --- Tests
