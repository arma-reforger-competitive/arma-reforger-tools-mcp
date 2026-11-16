//! Errors of the Workbench bridge. Every Workbench Tool gives the same text for them, and
//! `workbench_status` answers "not running" with the text of `NotRunning`.

use std::fmt;
use std::path::PathBuf;

use super::settings::{DOCUMENTS_SETTING, HOST_SETTING, PORT_SETTING};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Nothing answers on the host and port.
    NotRunning {
        host: String,
        port: u16,
    },

    /// The bridge copied the handler script and reloaded the scripts, but Workbench still does
    /// not have the current version.
    HandlerNotLoaded {
        file: PathBuf,
    },

    /// The bridge copied the handler script, but Workbench could not compile the scripts.
    CompileFailed {
        file: PathBuf,
    },

    /// Workbench took the connection but sent no whole answer in time.
    NoAnswer {
        host: String,
        port: u16,
        secs: f32,
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

    /// The handler script answered with an error.
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
    NoProjectList {
        folder: PathBuf,
    },
    NoOpenProject {
        loaded: Vec<String>,
    },
    ManyOpenProjects {
        folders: Vec<PathBuf>,
    },
    InstallFailed {
        folder: PathBuf,
        err: std::io::Error,
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
            Self::NotRunning { host, port } => write!(
                f,
                "Workbench is not running: nothing answers on {host}:{port}. Start Workbench and \
                 open your mod. If Workbench runs, check that File > Options > General > Net API \
                 is on, and that {HOST_SETTING} and {PORT_SETTING} match it. Without \
                 {HOST_SETTING}, the bridge uses the Windows host under WSL and 127.0.0.1 on \
                 Windows. Without {PORT_SETTING}, it uses 5775."
            ),
            Self::HandlerNotLoaded { file } => write!(
                f,
                "The bridge copied the handler script of this plugin to {} and reloaded the \
                 scripts, but Workbench still does not have version {}. Check that the copy is \
                 in the mod that is open in Workbench. Restart Workbench, open the same mod, and \
                 call again.",
                file.display(),
                env!("CARGO_PKG_VERSION")
            ),
            Self::CompileFailed { file } => write!(
                f,
                "The bridge copied the handler script of this plugin to {}, but Workbench could \
                 not compile the scripts of the open mod. Open the Script Editor in Workbench, \
                 press F7 to see the errors, fix them, and call again.",
                file.display()
            ),
            Self::NoAnswer { host, port, secs } => write!(
                f,
                "Workbench on {host}:{port} took the connection but sent no answer in {secs} \
                 seconds. Workbench may be busy, for example loading a World. Wait and call \
                 workbench_status again."
            ),
            Self::BadAnswer { garbled } => write!(
                f,
                "The program on the Workbench port sent an answer that is not a Workbench NET \
                 API answer: {garbled}. Check that {PORT_SETTING} is the Net API port in \
                 Workbench (File > Options > General)."
            ),
            Self::WorkbenchFailed { func, status } => {
                write!(f, "Workbench answered \"{status}\" to {func}.")
            }
            Self::HandlerFailed { message } => {
                write!(f, "The handler script in Workbench answered: {message}")
            }
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
            Self::NoProjectList { folder } => write!(
                f,
                "{} has no project list of Workbench (profile/.projectList_*.conf). Set \
                 {DOCUMENTS_SETTING} to the Workbench folder in Documents, the folder that holds \
                 \"profile\" and \"logs\". Then restart Claude Code.",
                folder.display()
            ),
            Self::NoOpenProject { loaded } => write!(
                f,
                "Workbench has no mod of yours open: it has loaded {}, and none of them is in \
                 the project list of Workbench. Open your mod in Workbench, then call again.",
                loaded.join(", ")
            ),
            Self::ManyOpenProjects { folders } => {
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
            Self::InstallFailed { folder, err } => write!(
                f,
                "The bridge could not copy the handler script into {}: {err}. Check that the \
                 folder is not read-only, then call again.",
                folder.display()
            ),
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
            Self::InstallFailed { err, .. } => Some(err),
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
            | Self::NoProjectList { .. }
            | Self::NoOpenProject { .. }
            | Self::ManyOpenProjects { .. } => None,
        }
    }
}

// region:    --- Froms

impl From<Garbled> for Error {
    fn from(garbled: Garbled) -> Self {
        Self::BadAnswer { garbled }
    }
}

// endregion: --- Froms
