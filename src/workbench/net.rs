//! One NET API call over TCP: a new connection, one request, one answer.

use std::time::Duration;

use serde_json::{Map, Value};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

use super::error::Garbled;
use super::settings::Address;
use super::wire::{self, MAX_ANSWER_BYTES};
use super::{Error, Result};

/// What Workbench answers to a function that it does not know: a Handler script that is not
/// loaded, or a built-in function that this Workbench does not have.
const UNDEFINED_FUNC: &str = "Undefined API func";

/// The answer to one call: the JSON payload, or the sign that Workbench does not know `func`.
#[derive(Debug)]
pub(super) enum Answer {
    Ok(Map<String, Value>),
    UndefinedFunc,
}

/// How long one call waits: for the connect, then for the whole answer.
pub(super) struct Wait {
    pub(super) connect: Duration,
    pub(super) answer: Duration,
}

/// Calls `func` on the NET API at `address` with `params`, and returns the JSON payload of the
/// answer.
pub(super) async fn call(
    address: Address,
    func: &'static str,
    mut params: Map<String, Value>,
    wait: Wait,
) -> Result<Answer> {
    params.insert("APIFunc".to_owned(), Value::from(func));
    let request = wire::request(&Value::Object(params).to_string());
    let target = (address.host.as_str(), address.port);
    // Under WSL, the Windows firewall can drop the connection to a closed port, so a time-out
    // here also means that nothing listens.
    let Ok(Ok(mut stream)) = timeout(wait.connect, TcpStream::connect(target)).await else {
        return Err(Error::NotRunning { address });
    };
    let exchange = async {
        stream.write_all(&request).await?;
        stream.shutdown().await?;
        let mut bytes = Vec::new();
        let limit = u64::try_from(MAX_ANSWER_BYTES + 1).unwrap_or(u64::MAX);
        (&mut stream).take(limit).read_to_end(&mut bytes).await?;
        Ok::<_, std::io::Error>(bytes)
    };
    let bytes = match timeout(wait.answer, exchange).await {
        Ok(Ok(bytes)) => bytes,
        Ok(Err(err)) => return Err(Error::ConnectionLost { address, err }),
        Err(_) => {
            return Err(Error::NoAnswer {
                address,
                secs: wait.answer.as_secs_f32(),
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
