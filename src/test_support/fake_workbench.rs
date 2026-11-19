//! A fake Workbench NET API on 127.0.0.1, for the tests of every Capability that calls Workbench
//! through the bridge.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// What the fake Workbench answers.
pub enum Reply {
    /// It answers like Workbench 1.6: `GetLoadedProjects` gives `loaded`, the Handler script
    /// answers with the version in `handler` or is not loaded, and `ReloadScripts` does what
    /// `reload` says.
    Workbench {
        loaded: Vec<String>,
        handler: Mutex<Option<String>>,
        reload: Reload,
    },

    /// It takes the connection and never answers.
    Silent,

    /// It takes the connection and closes it without reading the request, so the system sends a
    /// reset.
    Reset,

    /// It answers these bytes to every request.
    Bytes(Vec<u8>),
}

/// What `ReloadScripts` does in the fake Workbench.
#[derive(Clone, Copy)]
pub enum Reload {
    /// It loads the Handler script of this server.
    Loads,

    /// It answers "Compilation failed".
    CompilationFailed,

    /// It answers "Ok" but keeps the Handler script that was loaded.
    KeepsOld,
}

impl Reply {
    /// Workbench with the game and the mod `BridgeTest` loaded.
    pub fn workbench(handler: Option<&str>, reload: Reload) -> Self {
        Self::Workbench {
            loaded: vec!["ArmaReforger".to_owned(), "BridgeTest".to_owned()],
            handler: Mutex::new(handler.map(str::to_owned)),
            reload,
        }
    }

    /// The same Workbench with the projects `ids` loaded.
    pub fn loading(self, ids: &[&str]) -> Self {
        match self {
            Self::Workbench {
                handler, reload, ..
            } => Self::Workbench {
                loaded: ids.iter().map(|id| (*id).to_owned()).collect(),
                handler,
                reload,
            },
            other => other,
        }
    }
}

/// Starts a fake Workbench NET API on 127.0.0.1 and returns its port.
pub async fn fake_workbench(reply: Reply) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    serve(listener, Arc::new(reply));
    port
}

/// A port on 127.0.0.1 where nothing listens.
pub async fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    listener.local_addr().unwrap().port()
}

/// Answers every connection on `listener` with `reply`.
pub fn serve(listener: TcpListener, reply: Arc<Reply>) {
    tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let reply = Arc::clone(&reply);
            tokio::spawn(async move {
                if let Reply::Reset = reply.as_ref() {
                    // Wait until the request has come, then close with it unread.
                    tokio::time::sleep(Duration::from_millis(100)).await;
                    return;
                }
                let mut request = Vec::new();
                stream.read_to_end(&mut request).await.unwrap();
                let bytes = match reply.as_ref() {
                    Reply::Silent => {
                        tokio::time::sleep(Duration::from_secs(60)).await;
                        return;
                    }
                    Reply::Reset => return,
                    Reply::Bytes(bytes) => bytes.clone(),
                    Reply::Workbench {
                        loaded,
                        handler,
                        reload,
                    } => answer_like_workbench(
                        &request,
                        loaded,
                        &mut handler.lock().unwrap(),
                        *reload,
                    ),
                };
                stream.write_all(&bytes).await.unwrap();
            });
        }
    });
}

// region:    --- Support

/// Adds `text` as a Pascal string: a 4-byte little-endian length and then the UTF-8 bytes.
fn push_string(bytes: &mut Vec<u8>, text: &str) {
    let len = u32::try_from(text.len()).unwrap();
    bytes.extend_from_slice(&len.to_le_bytes());
    bytes.extend_from_slice(text.as_bytes());
}

/// The answer of Workbench 1.6 to `request`, with the projects `loaded` and the loaded `handler`
/// version.
fn answer_like_workbench(
    request: &[u8],
    loaded: &[String],
    handler: &mut Option<String>,
    reload: Reload,
) -> Vec<u8> {
    let payload: Value = serde_json::from_slice(last_string(request)).unwrap();
    let (status, body) = match (payload["APIFunc"].as_str().unwrap(), handler.as_deref()) {
        ("GetLoadedProjects", _) => ("Ok", json!({ "Loaded Projects": loaded })),
        ("IsWorkbenchRunning", _) => (
            "Ok",
            json!({
                "IsRunning": true,
                "ScriptsCompiled": !matches!(reload, Reload::CompilationFailed)
            }),
        ),
        ("ReloadScripts", _) => match reload {
            Reload::Loads => {
                *handler = Some(env!("CARGO_PKG_VERSION").to_owned());
                ("Ok", json!({}))
            }
            Reload::CompilationFailed => ("Compilation failed", json!({})),
            Reload::KeepsOld => ("Ok", json!({})),
        },
        ("ReforgerBridge", Some(version)) => (
            "Ok",
            json!({ "version": version, "action": payload["action"], "error": "" }),
        ),
        _ => ("Undefined API func", json!({})),
    };
    let mut bytes = Vec::new();
    push_string(&mut bytes, status);
    push_string(&mut bytes, &body.to_string());
    bytes
}

/// The last Pascal string of a request: its JSON payload.
fn last_string(request: &[u8]) -> &[u8] {
    let mut rest = request.get(4..).unwrap();
    let mut last = rest;
    while let Some((len, after)) = rest.split_first_chunk::<4>() {
        let len = usize::try_from(u32::from_le_bytes(*len)).unwrap();
        let (text, after) = after.split_at(len);
        last = text;
        rest = after;
    }
    last
}

// endregion: --- Support
