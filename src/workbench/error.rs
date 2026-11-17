//! Errors of the Workbench bridge. Every Workbench Tool gives the same text for them, and
//! `workbench_status` answers "not running" with the text of `NotRunning`.

use std::fmt;
use std::path::PathBuf;

use super::settings::{Address, DOCUMENTS_SETTING, HOST_SETTING, PORT_SETTING};

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
                "Workbench is not running: nothing answers on {address}. Start Workbench and \
                 open your mod. If Workbench runs, check that File > Options > General > Net API \
                 is on, and that {HOST_SETTING} and {PORT_SETTING} match it. Without \
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
            | Self::NoProject { .. } => None,
        }
    }
}

// region:    --- Froms

impl From<Garbled> for Error {
    fn from(garbled: Garbled) -> Self {
        Self::BadAnswer { garbled }
    }
}

impl From<NoProject> for Error {
    fn from(reason: NoProject) -> Self {
        Self::NoProject { reason }
    }
}

// endregion: --- Froms
