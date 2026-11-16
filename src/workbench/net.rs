//! One NET API call over TCP: a new connection, one request, one answer. Translated from
//! `rawCall` in `src/workbench/client.ts` of enfusion-mcp.

use std::time::Duration;

use serde_json::{Map, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

use super::error::Garbled;
use super::settings::{Address, Settings};
use super::wire::{self, MAX_ANSWER_BYTES};
use super::{Error, Result};

/// What Workbench answers to a function that it does not know: a handler script that is not
/// loaded, or a built-in function that this Workbench does not have.
const UNDEFINED_FUNC: &str = "Undefined API func";

/// The answer to one call: the JSON payload, or the sign that Workbench does not know `func`.
#[derive(Debug)]
pub(super) enum Answer {
    Ok(Map<String, Value>),
    UndefinedFunc,
}

/// Calls `func` on the NET API at `address` with `params`, waits up to `wait` for the answer
/// after the connect, and returns the JSON payload of the answer.
pub(super) async fn call(
    settings: &Settings,
    address: &Address,
    func: &'static str,
    mut params: Map<String, Value>,
    wait: Duration,
) -> Result<Answer> {
    params.insert("APIFunc".to_owned(), Value::from(func));
    let request = wire::request(&Value::Object(params).to_string());
    let not_running = || Error::NotRunning {
        host: address.host.clone(),
        port: address.port,
    };
    let target = (address.host.as_str(), address.port);
    // Under WSL, the Windows firewall can drop the connection to a closed port, so a time-out
    // here also means that nothing listens.
    let Ok(Ok(mut stream)) = timeout(settings.connect_timeout, TcpStream::connect(target)).await
    else {
        return Err(not_running());
    };
    let exchange = async {
        stream.write_all(&request).await?;
        stream.shutdown().await?;
        let mut bytes = Vec::new();
        let limit = u64::try_from(MAX_ANSWER_BYTES + 1).unwrap_or(u64::MAX);
        (&mut stream).take(limit).read_to_end(&mut bytes).await?;
        Ok::<_, std::io::Error>(bytes)
    };
    let bytes = match timeout(wait, exchange).await {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(_)) => return Err(not_running()),
        Err(_) => {
            return Err(Error::NoAnswer {
                host: address.host.clone(),
                port: address.port,
                secs: wait.as_secs_f32(),
            });
        }
    };
    if bytes.len() > MAX_ANSWER_BYTES {
        return Err(Garbled::TooLong {
            max: MAX_ANSWER_BYTES,
        }
        .into());
    }
    let (status, payload) = wire::answer(&bytes)?;
    match status.as_str() {
        "Ok" => {}
        UNDEFINED_FUNC => return Ok(Answer::UndefinedFunc),
        _ => return Err(Error::WorkbenchFailed { func, status }),
    }
    match serde_json::from_str(&payload) {
        Ok(Value::Object(object)) => Ok(Answer::Ok(object)),
        Ok(_) | Err(_) => Err(Garbled::NotJson { payload }.into()),
    }
}
