//! The settings of the Workbench bridge, read from environment variables (ADR 0007). A folder
//! path works in Windows form and in WSL form (ADR 0006).

use std::net::Ipv4Addr;
use std::path::PathBuf;
use std::time::Duration;

use super::{Error, Result};
use crate::host_path::host_path;

/// The computer that runs Workbench.
pub(super) const HOST_SETTING: &str = "REFORGER_WORKBENCH_HOST";

/// The NET API port of Workbench.
pub(super) const PORT_SETTING: &str = "REFORGER_WORKBENCH_PORT";

/// The Workbench folder in Documents, the folder that holds `profile` and `logs`.
pub(super) const DOCUMENTS_SETTING: &str = "REFORGER_WORKBENCH_DOCUMENTS_PATH";

/// The NET API port that Workbench uses when its options do not change it.
const DEFAULT_PORT: u16 = 5775;

/// The Workbench folder in Documents, under the user's folder on Windows.
const DOCUMENTS_IN_USER: &str = "Documents/My Games/ArmaReforgerWorkbench";

/// The raw values of the settings, as the user wrote them, and the time limits. An empty value
/// counts as not set.
#[derive(Clone)]
pub(super) struct Settings {
    pub(super) host: Option<String>,
    pub(super) port: Option<String>,
    pub(super) documents: Option<String>,

    /// How long the bridge waits for Workbench to take the connection.
    pub(super) connect_timeout: Duration,

    /// How long the bridge waits for the whole answer after it connected.
    pub(super) answer_timeout: Duration,

    /// How long the bridge waits for Workbench to compile the scripts again.
    pub(super) reload_timeout: Duration,
}

/// Where the NET API of Workbench listens.
#[derive(Clone, Debug, PartialEq, Eq)]
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
            port: read(PORT_SETTING),
            documents: read(DOCUMENTS_SETTING),
            connect_timeout: Duration::from_secs(3),
            answer_timeout: Duration::from_secs(10),
            reload_timeout: Duration::from_secs(120),
        }
    }

    /// The address of the NET API. Under WSL, when the host is not set, it is the address of the
    /// Windows host, because Workbench listens on all addresses of Windows.
    pub(super) fn address(&self) -> Result<Address> {
        let port = match self.port.as_deref() {
            None => DEFAULT_PORT,
            Some(raw) => {
                raw.trim()
                    .parse()
                    .ok()
                    .filter(|p| *p != 0)
                    .ok_or_else(|| Error::BadPort {
                        value: raw.to_owned(),
                    })?
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

    fn settings(host: Option<&str>, port: Option<&str>) -> Settings {
        Settings {
            host: host.map(str::to_owned),
            port: port.map(str::to_owned),
            documents: None,
            connect_timeout: Duration::from_secs(1),
            answer_timeout: Duration::from_secs(1),
            reload_timeout: Duration::from_secs(1),
        }
    }

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

    #[test]
    fn uses_the_host_and_port_settings_when_they_are_set() {
        // -- Exec & Check
        assert_eq!(
            settings(Some(" 10.0.0.5 "), Some("6000"))
                .address()
                .unwrap(),
            Address {
                host: "10.0.0.5".to_owned(),
                port: 6000
            }
        );
        assert_eq!(
            settings(Some("127.0.0.1"), None).address().unwrap().port,
            DEFAULT_PORT
        );
    }

    #[test]
    fn rejects_a_port_that_is_not_a_port() {
        // -- Exec & Check
        for raw in ["abc", "0", "70000", "-1"] {
            assert!(
                matches!(
                    settings(Some("127.0.0.1"), Some(raw)).address(),
                    Err(Error::BadPort { .. })
                ),
                "{raw}"
            );
        }
    }
}

// endregion: --- Tests
