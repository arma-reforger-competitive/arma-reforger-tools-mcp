//! Errors of the reference-mods Capability. Each one stops `reference_sync` before any delete,
//! except `WriteRecord` and `Cancelled`, which can come after a delete.

use std::fmt;
use std::path::PathBuf;

use super::sync::{LIST_FILE, MODS_SETTING};
use crate::workbench_documents::DOCUMENTS_SETTING;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    // -- Settings
    ModsNotSet,
    ModsNotFolder {
        folder: PathBuf,
    },
    DocumentsNotFound,

    // -- Files
    ReadFailed {
        path: PathBuf,
        err: std::io::Error,
    },
    BadListLine {
        file: PathBuf,
        line: usize,
        reason: BadLine,
    },
    BadRecordLine {
        file: PathBuf,
        line: usize,
        reason: BadLine,
    },
    WriteRecord {
        file: PathBuf,
        err: std::io::Error,
    },

    /// Claude cancelled the call.
    Cancelled,

    /// The work on the files stopped with a panic.
    Stopped,
}

/// Why a line of a list or of the record is not valid.
#[derive(Debug)]
pub enum BadLine {
    NotJson { detail: String },
    BadId { id: String },
    EmptyName,
    BadFolder { folder: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ModsNotSet => write!(
                f,
                "{MODS_SETTING} is not set. Set it to the folder that holds your mod repos, for \
                 example \"D:\\Mods\". The Tool reads {LIST_FILE} in each folder in it. Then \
                 restart Claude Code. Nothing was deleted."
            ),
            Self::ModsNotFolder { folder } => write!(
                f,
                "{MODS_SETTING} names {}, which is not a folder. Set it to the folder that holds \
                 your mod repos, then restart Claude Code. Nothing was deleted.",
                folder.display()
            ),
            Self::DocumentsNotFound => write!(
                f,
                "The Tool did not find the Workbench folder in Documents, the folder whose \
                 \"addons\" folder holds the mods that Workbench downloads. Set \
                 {DOCUMENTS_SETTING} to it, for example \
                 \"C:\\Users\\<you>\\Documents\\My Games\\ArmaReforgerWorkbench\". Then restart \
                 Claude Code. Nothing was deleted."
            ),
            Self::ReadFailed { path, err } => write!(
                f,
                "The Tool could not read {}: {err}. Nothing was deleted.",
                path.display()
            ),
            Self::BadListLine { file, line, reason } => write!(
                f,
                "Line {line} of {} is not valid: {reason}. A line is \
                 {{\"id\":\"<16 hex digits>\",\"name\":\"<mod name>\"}}, with no other fields, \
                 for example {{\"id\":\"6AA81EE549F50CB7\",\"name\":\"tvtLobby\"}}. Fix the line \
                 and call again. Nothing was deleted.",
                file.display()
            ),
            Self::BadRecordLine { file, line, reason } => write!(
                f,
                "Line {line} of the record {} is not valid: {reason}. The Tool writes this file \
                 itself. Delete the line, then call again. Nothing was deleted.",
                file.display()
            ),
            Self::WriteRecord { file, err } => write!(
                f,
                "The Tool deleted the mods, but could not write the record {}: {err}. Check that \
                 the folder is not read-only, then call again.",
                file.display()
            ),
            Self::Cancelled => write!(
                f,
                "The call was cancelled. The record names only the mod folders that are still on \
                 disk. Call again to finish the sync."
            ),
            Self::Stopped => write!(
                f,
                "The work on the files stopped with an error inside the Tool. Call again."
            ),
        }
    }
}

impl fmt::Display for BadLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotJson { detail } => write!(
                f,
                "it is not a JSON object with exactly the fields of a line ({detail})"
            ),
            Self::BadId { id } => write!(f, "the id \"{id}\" is not 16 hex digits"),
            Self::EmptyName => write!(f, "the name is empty"),
            Self::BadFolder { folder } => write!(
                f,
                "the folder \"{folder}\" is not a folder name of the form <Name>_<ID>"
            ),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadFailed { err, .. } | Self::WriteRecord { err, .. } => Some(err),
            Self::ModsNotSet
            | Self::ModsNotFolder { .. }
            | Self::DocumentsNotFound
            | Self::BadListLine { .. }
            | Self::BadRecordLine { .. }
            | Self::Cancelled
            | Self::Stopped => None,
        }
    }
}
