//! Errors of `cargo xtask call-tool`.

use std::fmt;
use std::path::PathBuf;

use rmcp::ServiceError;
use rmcp::service::ClientInitializeError;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// `HOME` is not set, so the settings file of Claude Code cannot be found.
    HomeNotSet,
    ReadSettings {
        path: PathBuf,
        source: std::io::Error,
    },
    BadSettings {
        path: PathBuf,
        source: serde_json::Error,
    },
    /// `target/release/reforger` does not exist.
    ServerNotBuilt {
        path: PathBuf,
    },
    /// An input is not `name=value`.
    BadInput {
        input: String,
    },
    /// The async runtime cannot start.
    Runtime(std::io::Error),
    ServerNotStarted {
        path: PathBuf,
        source: std::io::Error,
    },
    /// The server started, but the MCP start (initialize) failed.
    NoHandshake(Box<ClientInitializeError>),
    /// The list of Tools or the Tool call failed in MCP.
    Mcp {
        tool: String,
        source: Box<ServiceError>,
    },
    NoAnswer {
        tool: String,
        secs: u64,
    },
    /// The Tool answered, and its answer is an error. The text is printed already.
    ToolFailed {
        tool: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HomeNotSet => write!(
                f,
                "HOME is not set, so ~/.claude/settings.json cannot be found. Set HOME."
            ),
            Self::ReadSettings { path, source } => {
                write!(f, "Cannot read {}: {source}", path.display())
            }
            Self::BadSettings { path, source } => write!(
                f,
                "{} is not valid JSON: {source}. Fix the file.",
                path.display()
            ),
            Self::ServerNotBuilt { path } => write!(
                f,
                "{} does not exist. Run `cargo build --release` first.",
                path.display()
            ),
            Self::BadInput { input } => write!(
                f,
                "\"{input}\" is not an input. Write each input as name=value, for example query=S3C."
            ),
            Self::Runtime(err) => write!(f, "Cannot start the async runtime: {err}"),
            Self::ServerNotStarted { path, source } => write!(
                f,
                "Cannot start {}: {source}. Run `cargo build --release` again.",
                path.display()
            ),
            Self::NoHandshake(err) => write!(
                f,
                "The server started, but the MCP start failed: {err}. Run `cargo build --release` again."
            ),
            Self::Mcp { tool, source } => write!(f, "The call of {tool} failed: {source}"),
            Self::NoAnswer { tool, secs } => {
                write!(f, "The Tool {tool} gave no answer in {secs} seconds.")
            }
            Self::ToolFailed { tool } => write!(f, "The Tool {tool} answered with an error."),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReadSettings { source, .. } | Self::ServerNotStarted { source, .. } => {
                Some(source)
            }
            Self::Runtime(err) => Some(err),
            Self::BadSettings { source, .. } => Some(source),
            Self::NoHandshake(err) => Some(err.as_ref()),
            Self::Mcp { source, .. } => Some(source.as_ref()),
            Self::HomeNotSet
            | Self::ServerNotBuilt { .. }
            | Self::BadInput { .. }
            | Self::NoAnswer { .. }
            | Self::ToolFailed { .. } => None,
        }
    }
}
