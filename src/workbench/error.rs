//! Errors of the Workbench bridge. Every Workbench Tool gives the same text for them, and
//! `workbench_status` answers "not running" with the text of `NotRunning`.

use std::fmt;
use std::path::PathBuf;

use super::settings::{Address, DOWNLOADS_SETTING, HOST_SETTING, PORT_SETTING};
use super::steam::App;
use crate::workbench_documents::DOCUMENTS_SETTING;

/// Where users report a bug of the plugin.
const ISSUES_URL: &str = "https://github.com/lanka-community/arma-reforger-tools-mcp/issues";

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Nothing answers on the host and port.
    NotRunning {
        address: Address,
    },

    /// The bridge copied the Handler script and reloaded the scripts, but Workbench still does
    /// not have the current version.
    HandlerNotLoaded {
        file: PathBuf,
    },

    /// The bridge copied the Handler script, but Workbench could not compile the scripts.
    CompileFailed {
        file: PathBuf,
    },

    /// Workbench took the connection but sent no whole answer in time.
    NoAnswer {
        address: Address,
        secs: f32,
    },

    /// The connection broke after the connect, before the whole answer came.
    ConnectionLost {
        address: Address,
        err: std::io::Error,
    },

    /// The answer is not a NET API answer.
    BadAnswer {
        garbled: Garbled,
    },

    /// Workbench answered with an error status.
    WorkbenchFailed {
        func: &'static str,
        status: String,
    },

    /// The Handler script answered with an error.
    HandlerFailed {
        message: String,
    },

    // -- Settings
    BadPort {
        value: String,
    },
    NoWindowsHost,
    DocumentsNotFound,

    // -- The open project
    /// The bridge did not find the folder of the open mod project.
    NoProject {
        reason: NoProject,
    },
    InstallFailed {
        folder: PathBuf,
        err: std::io::Error,
    },

    // -- The start
    /// `workbench_launch` did not start Workbench, or Workbench did not get ready.
    NoStart {
        reason: NoStart,
    },
}

/// Why the bridge did not find the folder of the open mod project.
#[derive(Debug)]
pub enum NoProject {
    /// The Workbench folder in Documents has no project list.
    NoList { folder: PathBuf },

    /// No loaded project is in the project list.
    NotListed { loaded: Vec<String> },

    /// More than one loaded project in the list, and none of them needs the others.
    Many { folders: Vec<PathBuf> },
}

/// Why `workbench_launch` did not start Workbench, or why Workbench did not get ready.
#[derive(Debug)]
pub enum NoStart {
    /// The registry names no Steam folder, or the folder has no Steam program or no library file.
    NoSteam,

    /// No user was logged in to Steam, and PowerShell could not start Steam.
    SteamStartFailed {
        err: std::io::Error,
    },

    /// The bridge started Steam, but no user logged in in time.
    NoSteamLogin {
        secs: u64,
    },

    NotInstalled {
        app: App,
    },

    /// The project list has no mod with the name that Claude gave.
    UnknownMod {
        wanted: String,
        names: Vec<String>,
    },

    /// Claude gave no mod, and the project list has more than one.
    ChooseMod {
        names: Vec<String>,
    },

    /// PowerShell could not start Workbench.
    StartFailed {
        err: std::io::Error,
    },

    /// The NET API did not answer in time after the start.
    NoNetApi {
        secs: u64,
    },

    /// The NET API answered, but the scripts were not compiled in time.
    NotCompiled {
        secs: u64,
    },

    /// Workbench started, but it did not load the mod. `errors`: the errors in its log.
    NotLoaded {
        id: String,
        errors: Vec<String>,
    },
}

/// Why an answer is not a NET API answer.
#[derive(Debug)]
pub enum Garbled {
    /// The answer ends in the middle of a string, or is empty.
    Ends,
    ExtraBytes {
        count: usize,
    },
    TooLong {
        max: usize,
    },
    NotUtf8,
    NotJson {
        payload: String,
    },
    /// The JSON payload has no `field`.
    NoField {
        field: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRunning { address } => write!(
                f,
                "Workbench is not running: nothing answers on {address}. Call workbench_launch to \
                 start it with your mod. If Workbench runs, check that File > Options > General > \
                 Net API is on, and that {HOST_SETTING} and {PORT_SETTING} match it. Without \
                 {HOST_SETTING}, the bridge uses the Windows host under WSL and 127.0.0.1 on \
                 Windows. Without {PORT_SETTING}, it uses 5775."
            ),
            Self::HandlerNotLoaded { file } => write!(
                f,
                "The bridge copied the Handler script of this plugin to {} and reloaded the \
                 scripts, but Workbench still does not have version {}. Check that this folder \
                 is the mod that is open in Workbench. If it is not, set {DOCUMENTS_SETTING} to \
                 the Workbench folder in Documents, the folder whose project list names the open \
                 mod. Then call again.",
                file.display(),
                env!("CARGO_PKG_VERSION")
            ),
            Self::CompileFailed { file } => write!(
                f,
                "The bridge copied the Handler script of this plugin to {}, but Workbench could \
                 not compile the scripts of the open mod. Open the Script Editor in Workbench, \
                 press F7 to see the errors, fix them, and call again.",
                file.display()
            ),
            Self::NoAnswer { address, secs } => write!(
                f,
                "Workbench on {address} took the connection but sent no answer in {secs} \
                 seconds. Workbench may be busy, for example loading a World. Wait and call \
                 workbench_status again."
            ),
            Self::ConnectionLost { address, err } => write!(
                f,
                "The program on {address} took the connection, then closed it before it \
                 answered: {err}. Workbench may have stopped, or a port-forward rule may answer \
                 for a Workbench that does not run. Check that Workbench runs, then call \
                 workbench_status again."
            ),
            Self::BadAnswer { garbled } => write!(
                f,
                "The program on the Workbench port sent an answer that is not a Workbench NET \
                 API answer: {garbled}. Check that {PORT_SETTING} is the Net API port in \
                 Workbench (File > Options > General)."
            ),
            Self::WorkbenchFailed { func, status } => write!(
                f,
                "Workbench answered \"{status}\" to {func}. The bridge was tested with Workbench \
                 1.6: update Workbench if it is older. Otherwise, read the newest log in the \
                 \"logs\" folder of the Workbench folder in Documents, fix what it names, and \
                 call again."
            ),
            Self::HandlerFailed { message } => write!(
                f,
                "The Handler script in Workbench answered: {message}. If this text does not say \
                 what to do, report it as a bug at {ISSUES_URL}."
            ),
            Self::BadPort { value } => write!(
                f,
                "{PORT_SETTING} is \"{value}\", which is not a port. Set it to the Net API port in \
                 Workbench (File > Options > General), for example 5775, or remove it. Then \
                 restart Claude Code."
            ),
            Self::NoWindowsHost => write!(
                f,
                "The bridge runs in WSL but did not find the Windows host in /proc/net/route. Set \
                 {HOST_SETTING} to the address of Windows as WSL sees it, for example \
                 172.17.240.1. Then restart Claude Code."
            ),
            Self::DocumentsNotFound => write!(
                f,
                "The bridge did not find the Workbench folder in Documents, the folder that holds \
                 \"profile\" and \"logs\". Set {DOCUMENTS_SETTING} to it, for example \
                 \"C:\\Users\\<you>\\Documents\\My Games\\ArmaReforgerWorkbench\". Then restart \
                 Claude Code."
            ),
            Self::NoProject { reason } => write!(f, "{reason}"),
            Self::NoStart { reason } => write!(f, "{reason}"),
            Self::InstallFailed { folder, err } => write!(
                f,
                "The bridge could not copy the Handler script into {}: {err}. Check that the \
                 folder is not read-only, then call again.",
                folder.display()
            ),
        }
    }
}

impl fmt::Display for NoProject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoList { folder } => write!(
                f,
                "{} has no project list of Workbench (profile/.projectList_*.conf). Set \
                 {DOCUMENTS_SETTING} to the Workbench folder in Documents, the folder that holds \
                 \"profile\" and \"logs\". Then restart Claude Code.",
                folder.display()
            ),
            Self::NotListed { loaded } => write!(
                f,
                "Workbench has no mod of yours open: it has loaded {}, and none of them is in \
                 the project list of Workbench. Open your mod in Workbench. If your mod is open, \
                 add it to the project list: in the Workbench launcher, click Add Existing and \
                 pick its .gproj file. Then call again.",
                loaded.join(", ")
            ),
            Self::Many { folders } => {
                write!(
                    f,
                    "Workbench has more than one mod of yours loaded, and none of them is a \
                     dependency of the others:"
                )?;
                for folder in folders {
                    write!(f, " {};", folder.display())?;
                }
                write!(f, " Open only one mod in Workbench, then call again.")
            }
        }
    }
}

impl fmt::Display for NoStart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoSteam => write!(
                f,
                "The bridge did not find Steam: the Windows registry names no Steam folder \
                 (HKCU\\Software\\Valve\\Steam, SteamPath), or the folder has no steam.exe or \
                 no steamapps\\libraryfolders.vdf. Install Steam, sign in, and install Arma Reforger \
                 and Arma Reforger Tools. Then call workbench_launch again."
            ),
            Self::SteamStartFailed { err } => write!(
                f,
                "Workbench needs Steam with a logged-in user, and no user was logged in. The \
                 bridge could not start Steam through PowerShell: {err}. Start Steam yourself, \
                 log in, and call workbench_launch again."
            ),
            Self::NoSteamLogin { secs } => write!(
                f,
                "Workbench needs Steam with a logged-in user. The bridge started Steam, but no \
                 user logged in within {secs} seconds. Ask the user to log in to Steam, or to \
                 wait until the Steam update ends. Then call workbench_launch again."
            ),
            Self::NotInstalled { app } => write!(
                f,
                "{} is not installed in Steam. Install it: open steam://install/{} in a browser, \
                 or find {} in the Steam library and click Install. Then call workbench_launch \
                 again.",
                app.name(),
                app.id(),
                app.name()
            ),
            Self::UnknownMod { wanted, names } if names.is_empty() => write!(
                f,
                "The project list of Workbench has no mod \"{wanted}\": it has no mods. Call \
                 workbench_launch without a mod to open the Workbench launcher, and create the \
                 mod there."
            ),
            Self::UnknownMod { wanted, names } => write!(
                f,
                "The project list of Workbench has no mod \"{wanted}\". Its mods: {}. Call \
                 workbench_launch again with one of them.",
                names.join(", ")
            ),
            Self::ChooseMod { names } => write!(
                f,
                "The project list of Workbench has more than one mod: {}. Call workbench_launch \
                 again with `mod` set to one of them.",
                names.join(", ")
            ),
            Self::StartFailed { err } => write!(
                f,
                "The bridge could not start Workbench through PowerShell: {err}. Start Arma \
                 Reforger Tools from Steam yourself, open your mod, and call workbench_status."
            ),
            Self::NoNetApi { secs } => write!(
                f,
                "Workbench was started, but its NET API did not answer in {secs} seconds. In \
                 Workbench, turn on File > Options > General > Net API, and check that \
                 {HOST_SETTING} and {PORT_SETTING} match it. Then call workbench_status."
            ),
            Self::NotCompiled { secs } => write!(
                f,
                "Workbench was started, but its scripts were not compiled after {secs} seconds. \
                 Open the Script Editor in Workbench, press F7 to see the errors, fix them, and \
                 call workbench_status."
            ),
            Self::NotLoaded { id, errors } => {
                write!(
                    f,
                    "Workbench started, but it did not load the mod {id}: it has only the game \
                     open. "
                )?;
                if errors.is_empty() {
                    write!(f, "The Workbench log has no errors. ")?;
                } else {
                    write!(f, "Errors in the Workbench log: {}. ", errors.join("; "))?;
                }
                write!(
                    f,
                    "When a dependency \"can't be added\", Workbench did not find that mod. If the \
                     user moved the folder of the downloaded mods, set {DOWNLOADS_SETTING} to the \
                     folder that holds its addons folder. Else download the dependency in the \
                     Workbench launcher. Then close Workbench and call workbench_launch again."
                )
            }
        }
    }
}

impl fmt::Display for Garbled {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ends => write!(f, "it ends in the middle of a string"),
            Self::ExtraBytes { count } => write!(f, "it has {count} bytes after the payload"),
            Self::TooLong { max } => write!(f, "it is longer than {max} bytes"),
            Self::NotUtf8 => write!(f, "a string in it is not UTF-8"),
            Self::NotJson { payload } => {
                let start: String = payload.chars().take(80).collect();
                write!(f, "its payload is not JSON: \"{start}\"")
            }
            Self::NoField { field } => write!(f, "its payload has no \"{field}\""),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InstallFailed { err, .. } | Self::ConnectionLost { err, .. } => Some(err),
            Self::NotRunning { .. }
            | Self::HandlerNotLoaded { .. }
            | Self::CompileFailed { .. }
            | Self::NoAnswer { .. }
            | Self::BadAnswer { .. }
            | Self::WorkbenchFailed { .. }
            | Self::HandlerFailed { .. }
            | Self::BadPort { .. }
            | Self::NoWindowsHost
            | Self::DocumentsNotFound
            | Self::NoProject { .. }
            | Self::NoStart { .. } => None,
        }
    }
}

// region:    --- Froms

impl From<Garbled> for Error {
    fn from(garbled: Garbled) -> Self {
        Self::BadAnswer { garbled }
    }
}

impl From<NoStart> for Error {
    fn from(reason: NoStart) -> Self {
        Self::NoStart { reason }
    }
}

impl From<NoProject> for Error {
    fn from(reason: NoProject) -> Self {
        Self::NoProject { reason }
    }
}

// endregion: --- Froms
