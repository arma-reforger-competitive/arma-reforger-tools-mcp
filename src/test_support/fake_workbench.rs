//! A fake Workbench NET API on 127.0.0.1, for the tests of every Capability that calls Workbench
//! through the bridge.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

/// What the fake Workbench answers.
pub enum Reply {
    /// It answers like Workbench 1.6: `GetLoadedProjects` gives `loaded`, the Handler script
    /// answers with the version in `handler` or is not loaded, and `ReloadScripts` does what
    /// `reload` says. The Handler actions `reference_download` and `reference_progress` use
    /// `workshop`.
    Workbench {
        loaded: Vec<String>,
        handler: Mutex<Option<String>>,
        reload: Reload,
        workshop: Option<Arc<Workshop>>,
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
            workshop: None,
        }
    }

    /// The same Workbench, which downloads the mods of `workshop`.
    pub fn with_workshop(self, workshop: Arc<Workshop>) -> Self {
        match self {
            Self::Workbench {
                loaded,
                handler,
                reload,
                ..
            } => Self::Workbench {
                loaded,
                handler,
                reload,
                workshop: Some(workshop),
            },
            other => other,
        }
    }

    /// The same Workbench with the projects `ids` loaded.
    pub fn loading(self, ids: &[&str]) -> Self {
        match self {
            Self::Workbench {
                handler,
                reload,
                workshop,
                ..
            } => Self::Workbench {
                loaded: ids.iter().map(|id| (*id).to_owned()).collect(),
                handler,
                reload,
                workshop,
            },
            other => other,
        }
    }
}

/// The fake Workshop: the mods that the fake Workbench can download into `addons`. A download
/// stays at its percent until the test calls `finish`. A download at 100 percent ends when it
/// starts. Like the Handler script, it starts a download again only when it is complete, and it
/// knows only the downloads that started since `forget`.
pub struct Workshop {
    addons: PathBuf,
    mods: Mutex<BTreeMap<String, WorkshopMod>>,

    /// The IDs that the Workshop does not have, and that the Handler script was asked to start.
    started_unknown: Mutex<BTreeSet<String>>,

    /// How long the Handler script waits before it answers `reference_progress`.
    progress_delay: Mutex<Duration>,
}

struct WorkshopMod {
    name: String,
    version: String,
    license: String,
    percent: u8,
    starts: usize,

    /// The Handler script has a download of it in its memory.
    known: bool,
    complete: bool,
}

impl Workshop {
    /// A Workshop with no mods, which downloads into `addons`.
    pub fn new(addons: PathBuf) -> Arc<Self> {
        Arc::new(Self {
            addons,
            mods: Mutex::new(BTreeMap::new()),
            started_unknown: Mutex::new(BTreeSet::new()),
            progress_delay: Mutex::new(Duration::ZERO),
        })
    }

    /// Adds the mod `id`. Its download stays at `percent` until `finish`, or ends when it starts
    /// when `percent` is 100.
    pub fn add(&self, id: &str, name: &str, version: &str, license: &str, percent: u8) {
        self.mods.lock().unwrap().insert(
            id.to_owned(),
            WorkshopMod {
                name: name.to_owned(),
                version: version.to_owned(),
                license: license.to_owned(),
                percent,
                starts: 0,
                known: false,
                complete: false,
            },
        );
    }

    /// Ends the download of `id`: its folder `<Name>_<ID>` comes into `addons`.
    pub fn finish(&self, id: &str) {
        let mut mods = self.mods.lock().unwrap();
        self.complete(id, mods.get_mut(id).unwrap());
    }

    /// Makes each answer to `reference_progress` come after `delay`.
    pub fn answer_progress_after(&self, delay: Duration) {
        *self.progress_delay.lock().unwrap() = delay;
    }

    /// Workbench loads the scripts again: the Handler script forgets every download. The folders
    /// stay on disk.
    pub fn forget(&self) {
        for workshop_mod in self.mods.lock().unwrap().values_mut() {
            workshop_mod.known = false;
            workshop_mod.complete = false;
        }
        self.started_unknown.lock().unwrap().clear();
    }

    /// How many times the Handler script was asked to start a download of `id`.
    pub fn starts(&self, id: &str) -> usize {
        self.mods.lock().unwrap().get(id).map_or(0, |m| m.starts)
    }

    /// The answer of the Handler script to `action` with the parameter `id`, as the contract in
    /// issue #75 gives it.
    fn answer(&self, action: &str, id: &str) -> Value {
        let mut mods = self.mods.lock().unwrap();
        let Some(workshop_mod) = mods.get_mut(id) else {
            let mut started = self.started_unknown.lock().unwrap();
            return match action {
                "reference_download" => {
                    started.insert(id.to_owned());
                    json!({})
                }
                _ if started.contains(id) => json!({
                    "state": "failed",
                    "reason": "the Workshop has no mod with this ID"
                }),
                _ => json!({ "state": "unknown" }),
            };
        };
        match action {
            "reference_download" => {
                workshop_mod.starts += 1;
                if workshop_mod.known && !workshop_mod.complete {
                    return json!({});
                }
                workshop_mod.known = true;
                workshop_mod.complete = false;
                if workshop_mod.percent == 100 {
                    self.complete(id, workshop_mod);
                }
                json!({})
            }
            _ if !workshop_mod.known => json!({ "state": "unknown" }),
            _ if workshop_mod.complete => json!({
                "state": "complete",
                "name": workshop_mod.name,
                "mod_version": workshop_mod.version,
                "license": workshop_mod.license
            }),
            _ => json!({ "state": "downloading", "percent": workshop_mod.percent }),
        }
    }

    fn complete(&self, id: &str, workshop_mod: &mut WorkshopMod) {
        workshop_mod.complete = true;
        let folder = self.addons.join(format!("{}_{id}", workshop_mod.name));
        fs::create_dir_all(&folder).unwrap();
        fs::write(folder.join("data.pak"), "pak").unwrap();
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
                if let Reply::Workbench {
                    workshop: Some(workshop),
                    ..
                } = reply.as_ref()
                {
                    let payload: Value = serde_json::from_slice(last_string(&request)).unwrap();
                    if payload["action"] == "reference_progress" {
                        let delay = *workshop.progress_delay.lock().unwrap();
                        tokio::time::sleep(delay).await;
                    }
                }
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
                        workshop,
                    } => answer_like_workbench(
                        &request,
                        loaded,
                        &mut handler.lock().unwrap(),
                        *reload,
                        workshop.as_deref(),
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
    workshop: Option<&Workshop>,
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
        ("ReforgerBridge", Some(version)) => {
            let action = payload["action"].as_str().unwrap_or_default();
            let mut body = match (action, workshop) {
                ("reference_download" | "reference_progress", Some(workshop)) => {
                    workshop.answer(action, payload["id"].as_str().unwrap_or_default())
                }
                _ => json!({}),
            };
            body["version"] = json!(version);
            body["action"] = json!(action);
            body["error"] = json!("");
            ("Ok", body)
        }
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
