use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use super::*;
use crate::server::Server;
use crate::test_support::{self, Answer};
use crate::workbench::install::HANDLER;
use crate::workbench::project::tests::{gproj, write_projects};
use crate::workbench::wire::push_string;

#[tokio::test]
async fn says_workbench_is_not_running_when_nothing_answers() {
    // -- Setup & Fixtures
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    let setup = Setup::new();

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(&format!(
            "Workbench is not running: nothing answers on 127.0.0.1:{port}. Start Workbench and \
             open your mod."
        )),
        "{}",
        answer.text
    );
    assert!(answer.text.contains("REFORGER_WORKBENCH_HOST"));
    assert!(answer.text.contains("REFORGER_WORKBENCH_PORT"));
}

#[tokio::test]
async fn copies_and_loads_the_handler_when_it_is_not_loaded() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::workbench(None, Reload::Loads)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    let file = setup.handler_file();
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "Workbench is running, and the handler script of this plugin was not loaded. The \
             bridge copied it to {} and reloaded the scripts, and now it is loaded (version {}). \
             The open mod project is BridgeTest in {}.",
            file.display(),
            env!("CARGO_PKG_VERSION"),
            setup.project.display()
        )
    );
    assert_eq!(std::fs::read_to_string(file).unwrap(), HANDLER);
}

#[tokio::test]
async fn copies_and_loads_the_new_handler_when_workbench_runs_an_old_one() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::workbench(Some("0.0.1"), Reload::Loads)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    let file = setup.handler_file();
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "Workbench is running, and it had an old handler script (version 0.0.1). The bridge \
             copied the new one to {} and reloaded the scripts, and now it is loaded (version \
             {}). The open mod project is BridgeTest in {}.",
            file.display(),
            env!("CARGO_PKG_VERSION"),
            setup.project.display()
        )
    );
    assert_eq!(std::fs::read_to_string(file).unwrap(), HANDLER);
}

#[tokio::test]
async fn says_to_fix_the_scripts_when_workbench_cannot_compile_them() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::workbench(None, Reload::CompilationFailed)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "The bridge copied the handler script of this plugin to {}, but Workbench could not \
             compile the scripts of the open mod. Open the Script Editor in Workbench, press F7 \
             to see the errors, fix them, and call again.",
            setup.handler_file().display()
        )
    );
}

#[tokio::test]
async fn says_to_restart_when_the_reload_does_not_load_the_handler() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::workbench(None, Reload::KeepsOld)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(&format!(
            "The bridge copied the handler script of this plugin to {} and reloaded the \
             scripts, but Workbench still does not have version {}.",
            setup.handler_file().display(),
            env!("CARGO_PKG_VERSION")
        )),
        "{}",
        answer.text
    );
    assert!(answer.text.contains("Restart Workbench"), "{}", answer.text);
}

#[tokio::test]
async fn gives_the_version_and_the_open_mod_when_the_handler_is_loaded() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::workbench(
        Some(env!("CARGO_PKG_VERSION")),
        Reload::Loads,
    ))
    .await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "Workbench is running, and the handler script is loaded (version {}). The open mod \
             project is BridgeTest in {}.",
            env!("CARGO_PKG_VERSION"),
            setup.project.display()
        )
    );
    assert!(
        !setup.handler_file().exists(),
        "it copied a handler that was loaded"
    );
}

#[tokio::test]
async fn gives_an_error_when_workbench_takes_the_connection_but_never_answers() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::Silent).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(&format!(
            "Workbench on 127.0.0.1:{port} took the connection but sent no answer in 0.3 seconds."
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn gives_an_error_when_the_answer_is_not_a_net_api_answer() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::Bytes(b"HTTP/1.1 400 Bad Request\r\n\r\n".to_vec())).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(
            "The program on the Workbench port sent an answer that is not a Workbench NET API \
             answer: it ends in the middle of a string."
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn gives_an_error_when_the_payload_is_not_json() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let mut bytes = Vec::new();
    push_string(&mut bytes, "Ok");
    push_string(&mut bytes, "not json");
    let port = fake_workbench(Reply::Bytes(bytes)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .contains("its payload is not JSON: \"not json\""),
        "{}",
        answer.text
    );
}

// -- Test Support

/// A Workbench folder in Documents with a project list, and the mod project `BridgeTest` in it.
struct Setup {
    documents: tempfile::TempDir,
    project: PathBuf,
}

impl Setup {
    fn new() -> Self {
        let documents = tempfile::tempdir().unwrap();
        let project = documents.path().join("Mods/BridgeTest");
        write_projects(
            documents.path(),
            &[(&project, gproj("BridgeTest", "6AE15995AB330875", &[]))],
        );
        Self { documents, project }
    }

    fn handler_file(&self) -> PathBuf {
        self.project
            .join("Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c")
    }

    /// Calls `workbench_status` with Workbench on 127.0.0.1 at `port`.
    async fn status(&self, port: u16) -> Answer {
        let settings = Settings {
            host: Some("127.0.0.1".to_owned()),
            port: Some(port.to_string()),
            documents: Some(path_text(self.documents.path())),
            connect_timeout: Duration::from_secs(2),
            answer_timeout: Duration::from_millis(300),
            reload_timeout: Duration::from_secs(2),
        };
        let client = test_support::connect(router::<Server>(settings)).await;
        test_support::call(&client, "workbench_status", json!({})).await
    }
}

fn path_text(path: &Path) -> String {
    path.to_str().unwrap().to_owned()
}

/// What the fake Workbench answers.
enum Reply {
    /// It answers like Workbench 1.6 with the `BridgeTest` mod open: `GetLoadedProjects` gives
    /// the game and `BridgeTest`, the handler answers with the version in `handler` or is not
    /// loaded, and `ReloadScripts` does what `reload` says.
    Workbench {
        handler: Mutex<Option<String>>,
        reload: Reload,
    },

    /// It takes the connection and never answers.
    Silent,

    /// It answers these bytes to every request.
    Bytes(Vec<u8>),
}

/// What `ReloadScripts` does in the fake Workbench.
#[derive(Clone, Copy)]
enum Reload {
    /// It loads the handler script of this server.
    Loads,

    /// It answers "Compilation failed".
    CompilationFailed,

    /// It answers "Ok" but keeps the handler that was loaded.
    KeepsOld,
}

impl Reply {
    fn workbench(handler: Option<&str>, reload: Reload) -> Self {
        Self::Workbench {
            handler: Mutex::new(handler.map(str::to_owned)),
            reload,
        }
    }
}

/// Starts a fake Workbench NET API on 127.0.0.1 and returns its port.
async fn fake_workbench(reply: Reply) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let reply = Arc::new(reply);
    tokio::spawn(async move {
        loop {
            let (mut stream, _) = listener.accept().await.unwrap();
            let reply = Arc::clone(&reply);
            tokio::spawn(async move {
                let mut request = Vec::new();
                stream.read_to_end(&mut request).await.unwrap();
                let bytes = match reply.as_ref() {
                    Reply::Silent => {
                        tokio::time::sleep(Duration::from_secs(60)).await;
                        return;
                    }
                    Reply::Bytes(bytes) => bytes.clone(),
                    Reply::Workbench { handler, reload } => {
                        answer_like_workbench(&request, &mut handler.lock().unwrap(), *reload)
                    }
                };
                stream.write_all(&bytes).await.unwrap();
            });
        }
    });
    port
}

/// The answer of Workbench 1.6 to `request`, with the loaded `handler` version.
fn answer_like_workbench(request: &[u8], handler: &mut Option<String>, reload: Reload) -> Vec<u8> {
    let payload: Value = serde_json::from_slice(last_string(request)).unwrap();
    let (status, body) = match (payload["APIFunc"].as_str().unwrap(), handler.as_deref()) {
        ("GetLoadedProjects", _) => (
            "Ok",
            json!({ "Loaded Projects": ["ArmaReforger", "BridgeTest"] }),
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
