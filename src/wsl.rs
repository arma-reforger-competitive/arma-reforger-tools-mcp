//! WSL, the Linux of Windows: how to see that the program runs in it, and where its Windows host
//! is.
//!
//! This file uses only `std`, because `cargo xtask net-api` builds it too.

use std::net::Ipv4Addr;

/// True when the program runs in WSL.
pub fn is_wsl() -> bool {
    cfg!(target_os = "linux")
        && std::fs::read_to_string("/proc/version")
            .is_ok_and(|version| version.to_ascii_lowercase().contains("microsoft"))
}

/// The address of the Windows host as WSL sees it: the gateway of the default route. `None` when
/// `/proc/net/route` cannot be read or has no default route.
pub fn windows_host() -> Option<Ipv4Addr> {
    let table = std::fs::read_to_string("/proc/net/route").ok()?;
    default_gateway(&table)
}

// region:    --- Support

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
        // The route table of a WSL 2 computer. `ip route show default` gives 172.17.240.1 for it.
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
