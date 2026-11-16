//! Errors of the game-files Capability.

use std::fmt;
use std::path::PathBuf;

use super::settings::GAME_SETTING;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The setting that names the game folder is missing.
    GamePathNotSet,

    /// The game folder has no `addons` folder with .pak files.
    NoGamePaks {
        setting: String,
        folder: PathBuf,
    },

    /// Claude cancelled the call while the file lists were read.
    Cancelled,

    /// The work that reads the file lists stopped with a panic.
    ReaderStopped,

    // -- Damaged .pak files
    NotAPak,
    ChunkOutsidePak {
        at: u64,
    },
    NoFileTable,
    FileTableTooLarge {
        len: u64,
        max: u64,
    },
    FileTableEnds,
    RootNotAFolder,
    BadEntryKind {
        path: String,
        kind: u8,
    },
    TooManyChildren {
        path: String,
        count: u32,
    },
    TooDeep {
        path: String,
        max: usize,
    },
    PathTooLong {
        path: String,
        max: usize,
    },
    FileOutsidePak {
        path: String,
        end: u64,
        len: u64,
    },

    // -- Externals
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GamePathNotSet => write!(
                f,
                "{GAME_SETTING} is not set. Set it to the game folder, the folder that holds \
                 ArmaReforger.exe, for example \"D:\\SteamLibrary\\steamapps\\common\\Arma Reforger\" \
                 or \"/mnt/d/SteamLibrary/steamapps/common/Arma Reforger\". Then restart Claude Code."
            ),
            Self::NoGamePaks { setting, folder } => write!(
                f,
                "{GAME_SETTING} is \"{setting}\", but {} has no addons folder with .pak files. Set \
                 {GAME_SETTING} to the game folder, the folder that holds ArmaReforger.exe and \
                 addons. Then restart Claude Code.",
                folder.display()
            ),
            Self::Cancelled => write!(f, "The call was cancelled before the file lists were read."),
            Self::ReaderStopped => write!(
                f,
                "The reader of the .pak files stopped with an error. Call game_find again."
            ),
            Self::NotAPak => write!(f, "it does not start with a FORM PAC1 header"),
            Self::ChunkOutsidePak { at } => {
                write!(f, "the chunk at byte {at} goes past the end of the file")
            }
            Self::NoFileTable => write!(f, "it has no FILE table"),
            Self::FileTableTooLarge { len, max } => {
                write!(
                    f,
                    "its FILE table has {len} bytes, more than the limit of {max}"
                )
            }
            Self::FileTableEnds => write!(f, "its FILE table ends in the middle of an entry"),
            Self::RootNotAFolder => write!(f, "the first entry of its FILE table is not a folder"),
            Self::BadEntryKind { path, kind } => {
                write!(f, "the entry \"{path}\" has the unknown kind {kind}")
            }
            Self::TooManyChildren { path, count } => write!(
                f,
                "the folder \"{path}\" says it has {count} entries, more than its FILE table can hold"
            ),
            Self::TooDeep { path, max } => {
                write!(f, "the folder \"{path}\" is more than {max} folders deep")
            }
            Self::PathTooLong { path, max } => {
                write!(f, "the path \"{path}\" is longer than {max} bytes")
            }
            Self::FileOutsidePak { path, end, len } => write!(
                f,
                "the file \"{path}\" ends at byte {end}, past the end of the .pak file at byte {len}"
            ),
            Self::Io(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::GamePathNotSet
            | Self::NoGamePaks { .. }
            | Self::Cancelled
            | Self::ReaderStopped
            | Self::NotAPak
            | Self::ChunkOutsidePak { .. }
            | Self::NoFileTable
            | Self::FileTableTooLarge { .. }
            | Self::FileTableEnds
            | Self::RootNotAFolder
            | Self::BadEntryKind { .. }
            | Self::TooManyChildren { .. }
            | Self::TooDeep { .. }
            | Self::PathTooLong { .. }
            | Self::FileOutsidePak { .. } => None,
        }
    }
}

// region:    --- Froms

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

// endregion: --- Froms
