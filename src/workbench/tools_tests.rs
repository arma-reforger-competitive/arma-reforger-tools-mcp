use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use super::*;
use crate::server::Server;
use crate::test_support::{self, Answer};
use crate::workbench::install::HANDLER_SCRIPT;
use crate::workbench::settings::Port;
use crate::workbench::wire::push_string;

/// The end of every answer with a loaded Handler script.
const NEXT_STEP: &str = "The Workbench Tools can use it now. Before you pack or publish the mod, \
                         delete its folder Scripts/WorkbenchGame/ReforgerBridge: Workbench packs \
                         the Handler script with the mod.";

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
            "Workbench is running, and the Handler script of this plugin was not loaded. The \
             bridge copied it to {} and reloaded the scripts, and now it is loaded (version {}). \
             The open mod project is BridgeTest in {}. {NEXT_STEP}",
            file.display(),
            env!("CARGO_PKG_VERSION"),
            setup.project("BridgeTest").display()
        )
    );
    assert_eq!(fs::read_to_string(file).unwrap(), HANDLER_SCRIPT);
}

#[tokio::test]
async fn changes_no_file_outside_its_own_folder_when_it_copies_the_handler() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let project = setup.project("BridgeTest");
    for (file, text) in [
        ("Scripts/WorkbenchGame/MyPlugin/Tool.c", "class Tool {}"),
        ("Scripts/Game/Mode.c", "class Mode {}"),
    ] {
        fs::create_dir_all(project.join(file).parent().unwrap()).unwrap();
        fs::write(project.join(file), text).unwrap();
    }
    let before = snapshot(&project);
    let port = fake_workbench(Reply::workbench(None, Reload::Loads)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    let mut after = snapshot(&project);
    let handler = PathBuf::from("Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c");
    assert_eq!(
        after.remove(&handler).as_deref(),
        Some(HANDLER_SCRIPT.as_bytes())
    );
    assert_eq!(after, before, "it changed a file outside its folder");
}

#[tokio::test]
async fn copies_and_loads_the_new_handler_when_workbench_runs_an_old_one() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let file = setup.handler_file();
    fs::create_dir_all(file.parent().unwrap()).unwrap();
    fs::write(&file, "old").unwrap();
    let port = fake_workbench(Reply::workbench(Some("0.0.1"), Reload::Loads)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "Workbench is running, and it had an old Handler script (version 0.0.1). The bridge \
             copied the new one to {} and reloaded the scripts, and now it is loaded (version \
             {}). The open mod project is BridgeTest in {}. {NEXT_STEP}",
            file.display(),
            env!("CARGO_PKG_VERSION"),
            setup.project("BridgeTest").display()
        )
    );
    assert_eq!(fs::read_to_string(&file).unwrap(), HANDLER_SCRIPT);
    assert_eq!(
        fs::read_dir(file.parent().unwrap()).unwrap().count(),
        1,
        "a temp file is left"
    );
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
            "The bridge copied the Handler script of this plugin to {}, but Workbench could not \
             compile the scripts of the open mod. Open the Script Editor in Workbench, press F7 \
             to see the errors, fix them, and call again.",
            setup.handler_file().display()
        )
    );
}

#[tokio::test]
async fn says_what_to_check_when_the_reload_does_not_load_the_handler() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::workbench(None, Reload::KeepsOld)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "The bridge copied the Handler script of this plugin to {} and reloaded the scripts, \
             but Workbench still does not have version {}. Check that this folder is the mod \
             that is open in Workbench. If it is not, set REFORGER_WORKBENCH_DOCUMENTS_PATH to \
             the Workbench folder in Documents, the folder whose project list names the open \
             mod. Then call again.",
            setup.handler_file().display(),
            env!("CARGO_PKG_VERSION")
        )
    );
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
            "Workbench is running, and the Handler script is loaded (version {}). The open mod \
             project is BridgeTest in {}. {NEXT_STEP}",
            env!("CARGO_PKG_VERSION"),
            setup.project("BridgeTest").display()
        )
    );
    assert!(
        !setup.handler_file().exists(),
        "it copied a Handler script that was loaded"
    );
}

#[tokio::test]
async fn picks_the_loaded_mod_that_needs_the_other_loaded_mod() {
    // -- Setup & Fixtures
    let setup = Setup::with_projects(&[
        ("Lobby", "2222222222222222", &[]),
        ("Mission", "3333333333333333", &["2222222222222222"]),
    ]);
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads).loading(&[
            "ArmaReforger",
            "Lobby",
            "Mission",
        ]),
    )
    .await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer.text.contains(&format!(
            "The open mod project is Mission in {}.",
            setup.project("Mission").display()
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn says_how_to_open_the_mod_when_workbench_has_only_the_game_loaded() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port =
        fake_workbench(Reply::workbench(None, Reload::Loads).loading(&["ArmaReforger"])).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        "Workbench has no mod of yours open: it has loaded ArmaReforger, and none of them is in \
         the project list of Workbench. Open your mod in Workbench. If your mod is open, add it \
         to the project list: in the Workbench launcher, click Add Existing and pick its .gproj \
         file. Then call again."
    );
}

#[tokio::test]
async fn names_both_mods_when_two_unrelated_mods_are_loaded() {
    // -- Setup & Fixtures
    let setup = Setup::with_projects(&[
        ("A", "4444444444444444", &[]),
        ("B", "5555555555555555", &[]),
    ]);
    let port = fake_workbench(Reply::workbench(None, Reload::Loads).loading(&["A", "B"])).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("Workbench has more than one mod of yours loaded"),
        "{}",
        answer.text
    );
    for id in ["A", "B"] {
        assert!(
            answer
                .text
                .contains(&setup.project(id).display().to_string()),
            "{}",
            answer.text
        );
    }
}

#[tokio::test]
async fn names_the_setting_when_the_documents_folder_has_no_project_list() {
    // -- Setup & Fixtures
    let setup = Setup::with_projects(&[]);
    fs::remove_dir_all(setup.documents.path().join("profile")).unwrap();
    let port = fake_workbench(Reply::workbench(None, Reload::Loads)).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(&format!(
            "{} has no project list of Workbench (profile/.projectList_*.conf). Set \
             REFORGER_WORKBENCH_DOCUMENTS_PATH",
            setup.documents.path().display()
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn names_the_setting_when_the_port_is_not_a_port() {
    // -- Exec & Check
    for raw in ["abc", "0", "70000", "-1"] {
        let answer = Setup::new().status_on(Port::parse(Some(raw))).await;
        assert!(answer.is_error, "{raw}: {}", answer.text);
        assert!(
            answer.text.starts_with(&format!(
                "REFORGER_WORKBENCH_PORT is \"{raw}\", which is not a port."
            )),
            "{}",
            answer.text
        );
    }
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
async fn gives_an_error_when_the_connection_breaks_before_the_answer() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let port = fake_workbench(Reply::Reset).await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(&format!(
            "The program on 127.0.0.1:{port} took the connection, then closed it before it \
             answered:"
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

// region:    --- Test Support

/// A Workbench folder in Documents with a project list, and mod projects in `Mods/<ID>` next to it.
struct Setup {
    documents: tempfile::TempDir,
}

impl Setup {
    /// The mod project `BridgeTest` in the project list.
    fn new() -> Self {
        Self::with_projects(&[("BridgeTest", "6AE15995AB330875", &[])])
    }

    /// The mod projects `(ID, GUID, GUIDs of the dependencies)` in the project list.
    fn with_projects(projects: &[(&str, &str, &[&str])]) -> Self {
        let documents = tempfile::tempdir().unwrap();
        let setup = Self { documents };
        // The project list and a `.gproj` file like the ones that Workbench 1.6 writes.
        let mut list = "WBProjectList {\n Projects {\n".to_owned();
        for (id, guid, dependencies) in projects {
            let folder = setup.project(id);
            fs::create_dir_all(&folder).unwrap();
            let mut gproj =
                format!("GameProject {{\n ID \"{id}\"\n GUID \"{guid}\"\n TITLE \"{id}\"\n");
            gproj.push_str(" Dependencies {\n  \"58D0FB3206B6F859\"\n");
            for dependency in *dependencies {
                let _ = writeln!(gproj, "  \"{dependency}\"");
            }
            gproj.push_str(" }\n}");
            let file = folder.join("addon.gproj");
            fs::write(&file, gproj).unwrap();
            let _ = writeln!(
                list,
                "  WBProjectListItem {{\n   FilePath \"{}\"\n  }}",
                file.display()
            );
        }
        list.push_str(" }\n}\n");
        let profile = setup.documents.path().join("profile");
        fs::create_dir_all(&profile).unwrap();
        fs::write(profile.join(".projectList_app1874880_user1.conf"), list).unwrap();
        setup
    }

    fn project(&self, id: &str) -> PathBuf {
        self.documents.path().join("Mods").join(id)
    }

    fn handler_file(&self) -> PathBuf {
        self.project("BridgeTest")
            .join("Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c")
    }

    /// Calls `workbench_status` with Workbench on 127.0.0.1 at `port`.
    async fn status(&self, port: u16) -> Answer {
        self.status_on(Port::Valid(port)).await
    }

    /// Calls `workbench_status` with Workbench on 127.0.0.1 and the port setting `port`.
    async fn status_on(&self, port: Port) -> Answer {
        let settings = Settings {
            host: Some("127.0.0.1".to_owned()),
            port,
            documents: Some(self.documents.path().to_str().unwrap().to_owned()),
            connect_timeout: Duration::from_secs(2),
            answer_timeout: Duration::from_millis(300),
            reload_timeout: Duration::from_secs(2),
        };
        let client = test_support::connect(router::<Server>(settings)).await;
        test_support::call(&client, "workbench_status", json!({})).await
    }
}

/// Every file under `root` with its bytes.
fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    let mut files = BTreeMap::new();
    let mut folders = vec![root.to_owned()];
    while let Some(folder) = folders.pop() {
        for entry in fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                folders.push(path);
            } else {
                files.insert(
                    path.strip_prefix(root).unwrap().to_owned(),
                    fs::read(&path).unwrap(),
                );
            }
        }
    }
    files
}

/// What the fake Workbench answers.
enum Reply {
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
enum Reload {
    /// It loads the Handler script of this server.
    Loads,

    /// It answers "Compilation failed".
    CompilationFailed,

    /// It answers "Ok" but keeps the Handler script that was loaded.
    KeepsOld,
}

impl Reply {
    /// Workbench with the game and the mod `BridgeTest` loaded.
    fn workbench(handler: Option<&str>, reload: Reload) -> Self {
        Self::Workbench {
            loaded: vec!["ArmaReforger".to_owned(), "BridgeTest".to_owned()],
            handler: Mutex::new(handler.map(str::to_owned)),
            reload,
        }
    }

    /// The same Workbench with the projects `ids` loaded.
    fn loading(self, ids: &[&str]) -> Self {
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
async fn fake_workbench(reply: Reply) -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let reply = Arc::new(reply);
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
    port
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

// endregion: --- Test Support
