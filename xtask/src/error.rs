//! Errors of the developer commands.

use std::fmt;
use std::path::PathBuf;
use std::process::ExitStatus;

use crate::wikitext;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The setting that names the Workbench docs folder is missing.
    DocsPathNotSet,

    /// The wiki export has no page in the Arma Reforger namespace.
    NoWikiPages {
        path: PathBuf,
    },

    /// The Workbench docs list no engine page.
    NoEnginePages {
        path: PathBuf,
    },

    /// Two pages get the same file path.
    SamePath {
        path: String,
        first: String,
        second: String,
    },

    /// A CSS selector in the Doxygen readers is not valid.
    BadSelector(&'static str),

    /// A check of `cargo xtask ci` failed.
    CiStepFailed {
        step: &'static str,
        status: ExitStatus,
    },
    /// A program of `cargo xtask ci` cannot start, for example because it is not installed.
    CiStepNotStarted {
        step: &'static str,
        source: std::io::Error,
    },

    /// A line of tests/tool-choice/cases.jsonl is not a valid Tool-choice test.
    BadCase {
        line: usize,
        source: serde_json::Error,
    },
    /// The `claude` program cannot start.
    ClaudeNotStarted {
        source: std::io::Error,
    },
    /// A thread that ran Claude Code stopped with a panic.
    ClaudeRunStopped,
    /// No Tool-choice test accepts the Tool of `--tool`.
    NoToolChoiceCase {
        tool: String,
    },
    /// Too few Tool-choice tests passed.
    ToolChoiceBelowFloor {
        passed: usize,
        total: usize,
    },

    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    Write {
        path: PathBuf,
        source: std::io::Error,
    },

    // -- Modules
    Wikitext {
        title: String,
        source: wikitext::Error,
    },

    // -- Externals
    Xml {
        path: PathBuf,
        source: quick_xml::Error,
    },
    XmlEntity {
        path: PathBuf,
        source: quick_xml::escape::EscapeError,
    },
    Json(serde_json::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DocsPathNotSet => write!(
                f,
                "Set REFORGER_WORKBENCH_DOCS to the docs folder of Arma Reforger Tools, for example \
                 \"/mnt/d/Games/steamapps/common/Arma Reforger Tools/Workbench/docs\"."
            ),
            Self::NoWikiPages { path } => write!(
                f,
                "{} has no Arma Reforger page. Save it again from the wiki's Special:Export page \
                 (run `cargo xtask import --export-list` for the steps).",
                path.display()
            ),
            Self::NoEnginePages { path } => write!(
                f,
                "{} lists no engine page. Check that REFORGER_WORKBENCH_DOCS names the Workbench docs folder.",
                path.display()
            ),
            Self::SamePath {
                path,
                first,
                second,
            } => write!(
                f,
                "\"{first}\" and \"{second}\" both go to {path}. Change `page_path` in xtask/src/knowledge.rs."
            ),
            Self::BadSelector(css) => write!(
                f,
                "The CSS selector `{css}` in xtask/src/doxygen is not valid."
            ),
            Self::CiStepFailed { step, status } => write!(
                f,
                "ci failed at `{step}`: {status}. Fix it, then run `cargo xtask ci` again."
            ),
            Self::CiStepNotStarted { step, source } => write!(
                f,
                "ci cannot start `{step}`: {source}. Install the program (see the \"Development\" \
                 section of README.md)."
            ),
            Self::BadCase { line, source } => write!(
                f,
                "tests/tool-choice/cases.jsonl line {line}: {source}. A line needs \"question\" and \
                 \"tools\", and can have \"args\"."
            ),
            Self::ClaudeNotStarted { source } => write!(
                f,
                "Cannot start `claude`: {source}. Install Claude Code and log in."
            ),
            Self::ClaudeRunStopped => write!(f, "A Claude Code run stopped with a panic."),
            Self::NoToolChoiceCase { tool } => write!(
                f,
                "No test in tests/tool-choice/cases.jsonl accepts the Tool {tool}. Check the Tool \
                 name, or add at least 2 tests for it (docs/agents/mcp.md)."
            ),
            Self::ToolChoiceBelowFloor { passed, total } => write!(
                f,
                "Only {passed} of {total} Tool-choice tests passed; at least 90 % must pass. Read \
                 the FAIL lines, then fix the Tool description, the input text, or the Server guide."
            ),
            Self::Read { path, source } => write!(f, "Cannot read {}: {source}", path.display()),
            Self::Write { path, source } => write!(f, "Cannot write {}: {source}", path.display()),
            Self::Wikitext { title, source } => write!(f, "Page \"{title}\": {source}"),
            Self::Xml { path, source } => {
                write!(f, "{} is not valid XML: {source}", path.display())
            }
            Self::XmlEntity { path, source } => {
                write!(f, "{} has a bad XML entity: {source}", path.display())
            }
            Self::Json(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read { source, .. }
            | Self::Write { source, .. }
            | Self::CiStepNotStarted { source, .. }
            | Self::ClaudeNotStarted { source } => Some(source),
            Self::BadCase { source, .. } => Some(source),
            Self::Wikitext { source, .. } => Some(source),
            Self::Xml { source, .. } => Some(source),
            Self::XmlEntity { source, .. } => Some(source),
            Self::Json(err) => Some(err),
            Self::DocsPathNotSet
            | Self::NoWikiPages { .. }
            | Self::NoEnginePages { .. }
            | Self::SamePath { .. }
            | Self::BadSelector(_)
            | Self::CiStepFailed { .. }
            | Self::ClaudeRunStopped
            | Self::NoToolChoiceCase { .. }
            | Self::ToolChoiceBelowFloor { .. } => None,
        }
    }
}

// region:    --- Froms

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

// endregion: --- Froms
