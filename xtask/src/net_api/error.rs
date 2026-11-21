//! Errors of `cargo xtask net-api`.

use std::fmt;

use super::call::Address;
use crate::wire::BadBytes;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// The parameters are not a JSON object.
    BadParams {
        params: String,
    },
    BadPort {
        value: String,
    },
    NoWindowsHost,
    BadHost {
        host: String,
        source: std::io::Error,
    },
    NotRunning {
        address: Address,
    },
    NoAnswer {
        address: Address,
        secs: u64,
    },
    ConnectionLost {
        address: Address,
        source: std::io::Error,
    },
    AnswerTooLong {
        max: usize,
    },
    BadAnswer(BadBytes),

    /// Workbench answered, with a status that is not `Ok`.
    StatusNotOk {
        status: String,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadParams { params } => write!(
                f,
                r#"The parameters must be a JSON object, for example '{{"action":"ping"}}'. Got: {params}"#
            ),
            Self::BadPort { value } => write!(
                f,
                "REFORGER_WORKBENCH_PORT is \"{value}\", which is not a port."
            ),
            Self::NoWindowsHost => write!(
                f,
                "/proc/net/route has no default route. Set REFORGER_WORKBENCH_HOST."
            ),
            Self::BadHost { host, source } => write!(
                f,
                "Cannot find the computer \"{host}\": {source}. Check REFORGER_WORKBENCH_HOST."
            ),
            Self::NotRunning { address } => {
                write!(f, "Nothing answers on {address}. Workbench is not running.")
            }
            Self::NoAnswer { address, secs } => {
                write!(f, "No answer from {address} in {secs} seconds.")
            }
            Self::ConnectionLost { address, source } => {
                write!(f, "The connection to {address} broke: {source}")
            }
            Self::AnswerTooLong { max } => write!(
                f,
                "The answer of Workbench is longer than {max} bytes, so it is not a NET API answer."
            ),
            Self::BadAnswer(bad) => write!(f, "The answer of Workbench is broken: {bad}."),
            Self::StatusNotOk { status } => {
                write!(f, "Workbench answered with the status \"{status}\".")
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::BadHost { source, .. } | Self::ConnectionLost { source, .. } => Some(source),
            Self::BadParams { .. }
            | Self::BadPort { .. }
            | Self::NoWindowsHost
            | Self::NotRunning { .. }
            | Self::NoAnswer { .. }
            | Self::AnswerTooLong { .. }
            | Self::BadAnswer(_)
            | Self::StatusNotOk { .. } => None,
        }
    }
}
