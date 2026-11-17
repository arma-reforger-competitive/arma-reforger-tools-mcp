//! The settings of the Workbench bridge, read from environment variables (ADR 0007). A folder
//! path works in Windows form and in WSL form (ADR 0006).

use std::fmt;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::time::Duration;

use super::{Error, Result};
use crate::host_path::host_path;

/// The computer that runs Workbench.
pub(super) const HOST_SETTING: &str = "REFORGER_WORKBENCH_HOST";

/// The NET API port of Workbench.
pub(super) const PORT_SETTING: &str = "REFORGER_WORKBENCH_PORT";

/// The Workbench folder in Documents, the folder that holds `profile` and `logs`. It is not
/// `DOWNLOADS_SETTING`: that setting names the folder of the downloaded mods, which the user can
/// move out of Documents.
pub(super) const DOCUMENTS_SETTING: &str = "REFORGER_WORKBENCH_DOCUMENTS_PATH";

/// The folder whose `addons` folder holds the Workshop mods that Workbench downloaded. game-files
/// reads the same setting.
pub(super) const DOWNLOADS_SETTING: &str = "REFORGER_WORKBENCH_PROFILE_PATH";

/// The NET API port that Workbench uses when its options do not change it.
const DEFAULT_PORT: u16 = 5775;

/// The Workbench folder in Documents, under the user's folder on Windows.
const DOCUMENTS_IN_USER: &str = "Documents/My Games/ArmaReforgerWorkbench";

/// The settings and the time limits. An empty value counts as not set.
pub(super) struct Settings {
    /// The host as the user wrote it.
    pub(super) host: Option<String>,

    /// The port, checked when the server starts.
    pub(super) port: Port,

    /// The Workbench folder in Documents as the user wrote it.
    pub(super) documents: Option<String>,

    /// The folder of the downloaded mods as the user wrote it.
    pub(super) downloads: Option<String>,

    /// How long the bridge waits for Workbench to take the connection.
    pub(super) connect_timeout: Duration,

    /// How long the bridge waits for the whole answer after it connected.
    pub(super) answer_timeout: Duration,

    /// How long the bridge waits for Workbench to compile the scripts again.
    pub(super) reload_timeout: Duration,

    /// The Steam folder in Windows form. `None`: the folder that the Windows registry names.
    pub(super) steam: Option<String>,

    /// How `workbench_launch` starts the Workbench program.
    pub(super) starter: Starter,

    /// How long `workbench_launch` waits for the scripts of a started Workbench.
    pub(super) start_timeout: Duration,

    /// How long `workbench_launch` waits between two checks of a starting Workbench.
    pub(super) poll_interval: Duration,
}

/// How `workbench_launch` starts the Workbench program.
pub(super) enum Starter {
    /// With `Start-Process` of the PowerShell of Windows, apart from the server.
    PowerShell,

    /// A test gets the start command and fakes the start.
    #[cfg(test)]
    Test(std::sync::Arc<dyn Fn(&super::launch::StartCommand) + Send + Sync>),
}

/// The value of the port setting.
pub(super) enum Port {
    /// The port from the setting, or the default port when the setting is not set.
    Valid(u16),

    /// The setting has a value that is not a port.
    Invalid(String),
}

/// Where the NET API of Workbench listens.
#[derive(Debug)]
pub(super) struct Address {
    pub(super) host: String,
    pub(super) port: u16,
}

impl Settings {
    /// Reads the settings from the environment of the server.
    pub(super) fn from_env() -> Self {
        let read = |name| std::env::var(name).ok().filter(|v| !v.trim().is_empty());
        Self {
            host: read(HOST_SETTING),
            port: Port::parse(read(PORT_SETTING).as_deref()),
            documents: read(DOCUMENTS_SETTING),
            downloads: read(DOWNLOADS_SETTING),
            connect_timeout: Duration::from_secs(3),
            answer_timeout: Duration::from_secs(10),
            reload_timeout: Duration::from_secs(120),
            steam: None,
            starter: Starter::PowerShell,
            start_timeout: Duration::from_secs(180),
            poll_interval: Duration::from_secs(2),
        }
    }

    /// The address of the NET API. Under WSL, when the host is not set, it is the address of the
    /// Windows host, because Workbench listens on all addresses of Windows.
    pub(super) fn address(&self) -> Result<Address> {
        let port = match &self.port {
            Port::Valid(port) => *port,
            Port::Invalid(value) => {
                return Err(Error::BadPort {
                    value: value.clone(),
                });
            }
        };
        let host = match self.host.as_deref() {
            Some(host) => host.trim().to_owned(),
            None if is_wsl() => windows_host()?.to_string(),
            None => Ipv4Addr::LOCALHOST.to_string(),
        };
        Ok(Address { host, port })
    }

    /// The Workbench folder in Documents. Without the setting, it is the standard folder of the
    /// Windows user. Under WSL, that is the one folder in `/mnt/c/Users` that has it.
    pub(super) fn documents(&self) -> Result<PathBuf> {
        if let Some(raw) = self.documents.as_deref() {
            return Ok(host_path(raw));
        }
        if cfg!(windows) {
            let user = std::env::var_os("USERPROFILE").ok_or(Error::DocumentsNotFound)?;
            return Ok(PathBuf::from(user).join(DOCUMENTS_IN_USER));
        }
        if !is_wsl() {
            return Err(Error::DocumentsNotFound);
        }
        let mut found = std::fs::read_dir("/mnt/c/Users")
            .map_err(|_| Error::DocumentsNotFound)?
            .filter_map(|user| Some(user.ok()?.path().join(DOCUMENTS_IN_USER)))
            .filter(|folder| folder.join("profile").is_dir());
        match (found.next(), found.next()) {
            (Some(folder), None) => Ok(folder),
            _ => Err(Error::DocumentsNotFound),
        }
    }
}

impl Port {
    /// Checks the raw value of the port setting. Without a value, the port is 5775.
    pub(super) fn parse(raw: Option<&str>) -> Self {
        let Some(raw) = raw else {
            return Self::Valid(DEFAULT_PORT);
        };
        match raw.trim().parse() {
            Ok(port) if port != 0 => Self::Valid(port),
            _ => Self::Invalid(raw.to_owned()),
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

// region:    --- Support

/// True when the server runs in WSL.
fn is_wsl() -> bool {
    cfg!(target_os = "linux")
        && std::fs::read_to_string("/proc/version")
            .is_ok_and(|version| version.to_ascii_lowercase().contains("microsoft"))
}

/// The address of the Windows host as WSL sees it: the gateway of the default route.
fn windows_host() -> Result<Ipv4Addr> {
    let table = std::fs::read_to_string("/proc/net/route").map_err(|_| Error::NoWindowsHost)?;
    default_gateway(&table).ok_or(Error::NoWindowsHost)
}

/// The gateway of the default route in the text of `/proc/net/route`. Each address there is 8 hex
/// digits in little-endian order.
fn default_gateway(table: &str) -> Option<Ipv4Addr> {
    table.lines().skip(1).find_map(|line| {
        let mut fields = line.split_whitespace().skip(1);
        let (destination, gateway) = (fields.next()?, fields.next()?);
        if destination != "00000000" {
            return None;
        }
        let gateway = u32::from_str_radix(gateway, 16).ok()?;
        Some(Ipv4Addr::from(gateway.to_le_bytes()))
    })
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_the_windows_host_in_the_route_table_of_wsl() {
        // -- Setup & Fixtures
        // The route table of a WSL 2 computer whose Windows host is 172.17.240.1.
        let table = "Iface\tDestination\tGateway \tFlags\tRefCnt\tUse\tMetric\tMask\t\tMTU\tWindow\tIRTT\n\
                     eth0\t00F011AC\t00000000\t0001\t0\t0\t0\t00F0FFFF\t0\t0\t0\n\
                     eth0\t00000000\t01F011AC\t0003\t0\t0\t0\t00000000\t0\t0\t0\n";

        // -- Exec & Check
        assert_eq!(default_gateway(table), Some(Ipv4Addr::new(172, 17, 240, 1)));
    }

    #[test]
    fn finds_no_windows_host_without_a_default_route() {
        // -- Exec & Check
        assert_eq!(default_gateway("Iface\tDestination\tGateway\n"), None);
    }
}

// endregion: --- Tests
