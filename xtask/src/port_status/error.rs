//! Errors of `cargo xtask port-status`.

use std::fmt;
use std::path::PathBuf;

use super::ledger::KINDS;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The old project is not on disk.
    NoOldProject {
        path: PathBuf,
    },
    /// The ledger does not exist, so `set` has nothing to change.
    NoLedger,
    /// The home folder is not known, so the default settings have no folder.
    NoHome,

    /// A ledger row is not under a heading of a kind.
    NoKindHeading {
        line: usize,
    },
    /// A ledger row does not have 4 cells.
    NotFourCells {
        line: usize,
    },
    /// The status cell of a ledger row is not a status.
    BadRowStatus {
        line: usize,
        status: String,
    },

    /// `set not-needed` got no reason.
    NoReason,
    /// `set ported` got no link.
    NoLink,
    /// The kind in front of a selector is not a kind.
    BadKind {
        kind: String,
    },
    NoMatch {
        selector: String,
    },
    /// A selector without a kind matches rows of more than one kind.
    ManyKinds {
        selector: String,
        choices: Vec<String>,
    },
    NoSkill {
        name: String,
        file: String,
    },
    NoTool {
        name: String,
    },

    /// A regular expression in the code is not valid.
    BadRegex {
        pattern: &'static str,
        source: regex::Error,
    },
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoOldProject { path } => write!(
                f,
                "The old project is not at {}. Clone git@github.com:lanka-community/arma-enfusion-mcp.git there.",
                path.display()
            ),
            Self::NoLedger => write!(
                f,
                "docs/port-ledger.md does not exist. Run the report command first."
            ),
            Self::NoHome => write!(
                f,
                "The home folder is not known. Set PORT_STATUS_OLD_REPO and PORT_STATUS_HISTORY."
            ),
            Self::NoKindHeading { line } => {
                write!(f, "Line {line}: a row is not under a known kind heading.")
            }
            Self::NotFourCells { line } => write!(f, "Line {line}: a row needs 4 cells."),
            Self::BadRowStatus { line, status } => write!(
                f,
                "Line {line}: \"{status}\" is not a status. Use one of: ported, not needed, not yet."
            ),
            Self::NoReason => write!(f, "\"not needed\" needs a reason."),
            Self::NoLink => write!(f, "\"ported\" needs a link to the new Tool or skill."),
            Self::BadKind { kind } => {
                let kinds: Vec<&str> = KINDS.iter().map(|(_, id, _)| *id).collect();
                write!(
                    f,
                    "\"{kind}\" is not a kind. Use one of: {}.",
                    kinds.join(", ")
                )
            }
            Self::NoMatch { selector } => write!(f, "No feature matches \"{selector}\"."),
            Self::ManyKinds { selector, choices } => write!(
                f,
                "\"{selector}\" matches features of more than one kind. Write {}.",
                choices.join(" or ")
            ),
            Self::NoSkill { name, file } => {
                write!(f, "No skill named {name}: {file} does not exist.")
            }
            Self::NoTool { name } => write!(f, "No Tool named {name} in src/."),
            Self::BadRegex { pattern, source } => write!(
                f,
                "The regular expression `{pattern}` in xtask/src/port_status is not valid: {source}"
            ),
            Self::Read { path, source } => write!(f, "Cannot read {}: {source}", path.display()),
            Self::Write { path, source } => write!(f, "Cannot write {}: {source}", path.display()),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadRegex { source, .. } => Some(source),
            Self::Read { source, .. } | Self::Write { source, .. } => Some(source),
            Self::NoOldProject { .. }
            | Self::NoLedger
            | Self::NoHome
            | Self::NoKindHeading { .. }
            | Self::NotFourCells { .. }
            | Self::BadRowStatus { .. }
            | Self::NoReason
            | Self::NoLink
            | Self::BadKind { .. }
            | Self::NoMatch { .. }
            | Self::ManyKinds { .. }
            | Self::NoSkill { .. }
            | Self::NoTool { .. } => None,
        }
    }
}
