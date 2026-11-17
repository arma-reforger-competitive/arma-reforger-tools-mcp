//! Starts Workbench with one mod open, and waits until it has compiled its scripts. The bridge
//! itself never starts Workbench (ADR 0009): only `workbench_launch` does, through this module.
//!
//! It finds Workbench through Steam, copies the Handler script into the mod before the start, so
//! the first compile loads it, and starts the program with `-gproj` and the game folder as the
//! work folder. Translated from `launchWorkbench` in `src/workbench/client.ts` of enfusion-mcp,
//! with the program found through Steam, and with `-addonsDir` in place of the dependency links.
//!
//! A start with `-gproj` looks for dependencies only in the `addons` folders of the game and of
//! the Workbench folder in Documents. It does not get the launch options of Steam, so when the
//! user moved the folder of the downloaded mods, `-addonsDir` names it (Workbench 1.6 test: a
//! Workshop dependency in `D:\DownloadedMods\addons` loaded only with it).

use std::fmt::Write as _;
use std::fs;
use std::path::Path;
use std::time::Instant;

use tokio::process::Command;

use super::bridge::Bridge;
use super::error::{NoProject, NoStart};
use super::install::install;
use super::project::{Project, listed_mods};
use super::settings::Starter;
use super::steam::find_install;
use super::{Error, Result};
use crate::host_path::windows_path;

/// The most errors of the Workbench log that an answer gives.
const MAX_LOG_ERRORS: usize = 5;

/// What `workbench_launch` did.
pub(super) enum Launched {
    /// Workbench ran already, so it started nothing.
    AlreadyRunning,

    /// It started Workbench with `project` open, and the scripts are compiled.
    Started { project: Project },

    /// It started Workbench without a mod, because the project list has none. Workbench shows
    /// its launcher.
    StartedLauncher,
}

/// The start of the Workbench program. Every path is in Windows form.
#[derive(Debug)]
pub(super) struct StartCommand {
    pub(super) workbench: String,

    /// The work folder: the game folder, where Workbench finds the game data.
    pub(super) game: String,

    /// The `.gproj` file of the mod to open, or `None` for the launcher.
    pub(super) gproj: Option<String>,

    /// The `addons` folder of the downloaded mods, or `None` when the user did not move it.
    pub(super) addons: Option<String>,
}

/// Starts Workbench with the mod named `wanted`, or with the one mod of the project list.
pub(super) async fn launch(bridge: &Bridge, wanted: Option<&str>) -> Result<Launched> {
    match bridge.scripts_compiled().await {
        Err(Error::NotRunning { .. }) => {}
        Ok(_) => return Ok(Launched::AlreadyRunning),
        Err(error) => return Err(error),
    }
    let settings = bridge.settings();
    let install_folders = find_install(settings.steam.as_deref()).await?;
    let project = choose(mods(bridge)?, wanted)?;
    if let Some(project) = &project {
        install(&project.folder)?;
    }
    let command = StartCommand {
        workbench: install_folders.workbench,
        game: install_folders.game,
        gproj: project.as_ref().map(|project| project.gproj.clone()),
        addons: settings
            .downloads
            .as_deref()
            .map(|downloads| format!(r"{}\addons", windows_path(downloads))),
    };
    start(&settings.starter, &command)
        .await
        .map_err(|err| NoStart::StartFailed { err })?;
    let Some(project) = project else {
        return Ok(Launched::StartedLauncher);
    };
    wait_for_scripts(bridge).await?;
    // A mod whose dependency Workbench does not find does not load, and Workbench opens only the
    // game.
    if !bridge.loaded_projects().await?.contains(&project.id) {
        return Err(NoStart::NotLoaded {
            id: project.id,
            errors: log_errors(&settings.documents()?),
        }
        .into());
    }
    Ok(Launched::Started { project })
}

// region:    --- Support

/// The mods of the project list. A Workbench that never ran has no Documents folder and no
/// project list, so it has no mods.
fn mods(bridge: &Bridge) -> Result<Vec<Project>> {
    let documents = match bridge.settings().documents() {
        Ok(documents) => documents,
        Err(Error::DocumentsNotFound) => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    match listed_mods(&documents) {
        Err(Error::NoProject {
            reason: NoProject::NoList { .. },
        }) => Ok(Vec::new()),
        listed => listed,
    }
}

/// The mod named `wanted` (its ID or its folder name, in any case), or the one mod when Claude
/// named none. `None`: no mod, so Workbench starts on its launcher.
fn choose(mods: Vec<Project>, wanted: Option<&str>) -> Result<Option<Project>> {
    let names: Vec<String> = mods.iter().map(|project| project.id.clone()).collect();
    let Some(wanted) = wanted.map(str::trim).filter(|wanted| !wanted.is_empty()) else {
        return match mods.len() {
            0 => Ok(None),
            1 => Ok(mods.into_iter().next()),
            _ => Err(NoStart::ChooseMod { names }.into()),
        };
    };
    let is_wanted = |project: &Project| {
        project.id.eq_ignore_ascii_case(wanted)
            || project
                .folder
                .file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.eq_ignore_ascii_case(wanted))
    };
    match mods.into_iter().find(is_wanted) {
        Some(project) => Ok(Some(project)),
        None => Err(NoStart::UnknownMod {
            wanted: wanted.to_owned(),
            names,
        }
        .into()),
    }
}

/// Starts the Workbench program. `Start-Process` of the PowerShell of Windows starts it apart
/// from the server, so Workbench keeps running when the server stops. Under WSL,
/// `powershell.exe` is the PowerShell of Windows.
async fn start(starter: &Starter, command: &StartCommand) -> std::io::Result<()> {
    match starter {
        Starter::PowerShell => {
            let status = Command::new("powershell.exe")
                .args(["-NoProfile", "-NonInteractive", "-Command"])
                .arg(start_script(command))
                .status()
                .await?;
            if status.success() {
                Ok(())
            } else {
                Err(std::io::Error::other(format!(
                    "PowerShell ended with {status}"
                )))
            }
        }
        #[cfg(test)]
        Starter::Test(start) => {
            start(command);
            Ok(())
        }
    }
}

/// The PowerShell command that starts `command`. Single quotes keep every character of a path,
/// and a single quote in a path is written twice.
fn start_script(command: &StartCommand) -> String {
    let quote = |text: &str| format!("'{}'", text.replace('\'', "''"));
    let mut script = format!(
        "Start-Process -FilePath {} -WorkingDirectory {}",
        quote(&command.workbench),
        quote(&command.game)
    );
    // Workbench gets one argument for each path, also when the path has spaces.
    let mut arguments = Vec::new();
    for (option, path) in [("-gproj", &command.gproj), ("-addonsDir", &command.addons)] {
        if let Some(path) = path {
            arguments.push(quote(option));
            arguments.push(quote(&format!("\"{path}\"")));
        }
    }
    if !arguments.is_empty() {
        let _ = write!(script, " -ArgumentList {}", arguments.join(","));
    }
    script
}

/// The errors in the log of the last Workbench start: the lines with `(E)` in `error.log` of the
/// newest folder in `logs`. Workbench names each folder by its start time, so the newest name
/// sorts last.
fn log_errors(documents: &Path) -> Vec<String> {
    let newest = fs::read_dir(documents.join("logs"))
        .into_iter()
        .flatten()
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.is_dir())
        .max();
    let Some(text) = newest.and_then(|folder| fs::read_to_string(folder.join("error.log")).ok())
    else {
        return Vec::new();
    };
    text.lines()
        .filter_map(|line| Some(line.split_once("(E): ")?.1.trim().to_owned()))
        .take(MAX_LOG_ERRORS)
        .collect()
}

/// Asks `IsWorkbenchRunning` until the scripts are compiled or the time is over.
async fn wait_for_scripts(bridge: &Bridge) -> Result<()> {
    let settings = bridge.settings();
    let deadline = Instant::now() + settings.start_timeout;
    let mut answered = false;
    loop {
        match bridge.scripts_compiled().await {
            Ok(true) => return Ok(()),
            Ok(false) => answered = true,
            Err(
                Error::NotRunning { .. } | Error::ConnectionLost { .. } | Error::NoAnswer { .. },
            ) => {}
            Err(error) => return Err(error),
        }
        if Instant::now() >= deadline {
            let secs = settings.start_timeout.as_secs();
            let reason = if answered {
                NoStart::NotCompiled { secs }
            } else {
                NoStart::NoNetApi { secs }
            };
            return Err(reason.into());
        }
        tokio::time::sleep(settings.poll_interval).await;
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_a_start_command_that_powershell_reads_as_one_path() {
        // -- Setup & Fixtures
        let command = StartCommand {
            workbench: r"D:\Games\steamapps\common\Arma Reforger Tools\Workbench\ArmaReforgerWorkbenchSteamDiag.exe".to_owned(),
            game: r"D:\Games\steamapps\common\Arma Reforger".to_owned(),
            gproj: Some("D:/Mods/Bob's Mod/addon.gproj".to_owned()),
            addons: Some(r"D:\Downloaded Mods\addons".to_owned()),
        };

        // -- Exec & Check
        assert_eq!(
            start_script(&command),
            r#"Start-Process -FilePath 'D:\Games\steamapps\common\Arma Reforger Tools\Workbench\ArmaReforgerWorkbenchSteamDiag.exe' -WorkingDirectory 'D:\Games\steamapps\common\Arma Reforger' -ArgumentList '-gproj','"D:/Mods/Bob''s Mod/addon.gproj"','-addonsDir','"D:\Downloaded Mods\addons"'"#
        );
    }
}

// endregion: --- Tests
