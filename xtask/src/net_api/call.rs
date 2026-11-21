//! One raw call to the Workbench NET API, with the bytes of
//! `src/workbench/wire.rs` and the host search of `src/wsl.rs`.

use std::fmt;
use std::io::{ErrorKind, Read as _, Write as _};
use std::net::{Shutdown, TcpStream, ToSocketAddrs as _};
use std::time::{Duration, Instant};

use serde_json::{Map, Value};

use super::{Error, Result};
use crate::cmd::NetApiArgs;
use crate::wire::{self, MAX_ANSWER_BYTES};
use crate::wsl::{is_wsl, windows_host};

/// The computer that runs Workbench. The Workbench bridge reads the same setting.
const HOST_SETTING: &str = "REFORGER_WORKBENCH_HOST";

/// The NET API port. The Workbench bridge reads the same setting.
const PORT_SETTING: &str = "REFORGER_WORKBENCH_PORT";

/// The NET API port that Workbench uses when its options do not change it.
const DEFAULT_PORT: u16 = 5775;

/// How long a call waits for Workbench to take the connection.
const CONNECT_WAIT: Duration = Duration::from_secs(3);

/// How long a call waits for the whole answer. `ReloadScripts` answers when the compile ends, and
/// the bridge waits 120 seconds for it.
const ANSWER_WAIT: Duration = Duration::from_secs(130);

/// Where the NET API of Workbench listens.
#[derive(Debug, Clone)]
pub struct Address {
    pub host: String,
    pub port: u16,
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.host, self.port)
    }
}

pub fn run(args: &NetApiArgs) -> Result<()> {
    let params = args.params.as_deref().unwrap_or("{}");
    let Ok(Value::Object(mut payload)) = serde_json::from_str(params) else {
        return Err(Error::BadParams {
            params: params.to_owned(),
        });
    };
    payload.insert("APIFunc".to_owned(), Value::from(args.function.as_str()));

    let read = |name| std::env::var(name).ok().filter(|v| !v.trim().is_empty());
    let address = address(read(HOST_SETTING), read(PORT_SETTING))?;
    let (status, payload) = call(&address, &payload, CONNECT_WAIT, ANSWER_WAIT)?;
    println!("status: {status}");
    if !payload.is_empty() {
        println!("{payload}");
    }
    if status != "Ok" {
        return Err(Error::StatusNotOk { status });
    }
    Ok(())
}

// region:    --- Support

/// The address from the host and port settings, the same way as the Workbench bridge: without a
/// host, the Windows host under WSL, and 127.0.0.1 elsewhere.
fn address(host: Option<String>, port: Option<String>) -> Result<Address> {
    let port = match port {
        None => DEFAULT_PORT,
        Some(raw) => match raw.trim().parse() {
            Ok(port) if port != 0 => port,
            _ => return Err(Error::BadPort { value: raw }),
        },
    };
    let host = match host {
        Some(host) => host.trim().to_owned(),
        None if is_wsl() => windows_host().ok_or(Error::NoWindowsHost)?.to_string(),
        None => "127.0.0.1".to_owned(),
    };
    Ok(Address { host, port })
}

/// Sends `payload` to the NET API at `address`, and returns the status and the payload of the
/// answer.
fn call(
    address: &Address,
    payload: &Map<String, Value>,
    connect_wait: Duration,
    answer_wait: Duration,
) -> Result<(String, String)> {
    let request = wire::request(&Value::Object(payload.clone()).to_string());
    let mut stream = connect(address, connect_wait)?;
    let deadline = Instant::now() + answer_wait;
    let lost = |source| Error::ConnectionLost {
        address: address.clone(),
        source,
    };
    let no_answer = || Error::NoAnswer {
        address: address.clone(),
        secs: answer_wait.as_secs(),
    };

    stream.set_write_timeout(Some(answer_wait)).map_err(lost)?;
    stream.write_all(&request).map_err(lost)?;
    stream.shutdown(Shutdown::Write).map_err(lost)?;

    let mut bytes = Vec::new();
    let mut chunk = [0_u8; 16 * 1024];
    loop {
        let left = deadline.saturating_duration_since(Instant::now());
        if left.is_zero() {
            return Err(no_answer());
        }
        stream.set_read_timeout(Some(left)).map_err(lost)?;
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(count) => bytes.extend_from_slice(chunk.get(..count).unwrap_or_default()),
            Err(err) if err.kind() == ErrorKind::Interrupted => {}
            Err(err) if matches!(err.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {
                return Err(no_answer());
            }
            Err(err) => return Err(lost(err)),
        }
        if bytes.len() > MAX_ANSWER_BYTES {
            return Err(Error::AnswerTooLong {
                max: MAX_ANSWER_BYTES,
            });
        }
    }
    wire::answer(&bytes).map_err(Error::BadAnswer)
}

/// The connection to the first address of the host that takes it.
fn connect(address: &Address, wait: Duration) -> Result<TcpStream> {
    let targets = (address.host.as_str(), address.port)
        .to_socket_addrs()
        .map_err(|source| Error::BadHost {
            host: address.host.clone(),
            source,
        })?;
    // Under WSL, the Windows firewall can drop the connection to a closed port, so a time-out
    // here also means that nothing listens.
    targets
        .into_iter()
        .find_map(|target| TcpStream::connect_timeout(&target, wait).ok())
        .ok_or_else(|| Error::NotRunning {
            address: address.clone(),
        })
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use std::net::TcpListener;
    use std::thread;

    use super::*;

    #[test]
    fn sends_the_request_bytes_and_reads_the_answer_of_workbench() {
        // -- Setup & Fixtures
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = listener.local_addr().unwrap().port();
        let workbench = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = Vec::new();
            stream.read_to_end(&mut request).unwrap();
            // The bytes that Workbench 1.6 sent to GetLoadedProjects with the BridgeTest mod open.
            stream
                .write_all(b"\x02\x00\x00\x00Ok1\x00\x00\x00{\"Loaded Projects\":[\"ArmaReforger\",\"BridgeTest\"]}")
                .unwrap();
            request
        });
        let address = Address {
            host: "127.0.0.1".to_owned(),
            port,
        };
        let mut payload = Map::new();
        payload.insert("APIFunc".to_owned(), Value::from("GetLoadedProjects"));

        // -- Exec
        let answer = call(&address, &payload, CONNECT_WAIT, ANSWER_WAIT).unwrap();

        // -- Check
        assert_eq!(
            answer,
            (
                "Ok".to_owned(),
                r#"{"Loaded Projects":["ArmaReforger","BridgeTest"]}"#.to_owned()
            )
        );
        let request = workbench.join().unwrap();
        assert!(
            request.ends_with(br#"{"APIFunc":"GetLoadedProjects"}"#),
            "{request:?}"
        );
    }

    #[test]
    fn says_that_workbench_is_not_running_when_nothing_listens() {
        // -- Setup & Fixtures
        let port = TcpListener::bind("127.0.0.1:0")
            .unwrap()
            .local_addr()
            .unwrap()
            .port();
        let address = Address {
            host: "127.0.0.1".to_owned(),
            port,
        };

        // -- Exec
        let err = call(&address, &Map::new(), CONNECT_WAIT, ANSWER_WAIT).unwrap_err();

        // -- Check
        assert_eq!(
            err.to_string(),
            format!("Nothing answers on 127.0.0.1:{port}. Workbench is not running.")
        );
    }

    #[test]
    fn uses_port_5775_without_a_port_setting_and_refuses_a_value_that_is_not_a_port() {
        // -- Exec & Check
        let address = address(Some(" 10.0.0.5 ".to_owned()), None).unwrap();
        assert_eq!(address.to_string(), "10.0.0.5:5775");
        for value in ["0", "65536", "abc", "-1"] {
            let err = super::address(Some("h".to_owned()), Some(value.to_owned())).unwrap_err();
            assert_eq!(
                err.to_string(),
                format!("REFORGER_WORKBENCH_PORT is \"{value}\", which is not a port.")
            );
        }
    }
}

// endregion: --- Tests
