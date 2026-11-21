use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde_json::{Value, json};
use tokio::net::TcpListener;

use super::*;
use crate::server::Server;
use crate::test_support::fake_workbench::{Reply, serve};
use crate::test_support::{self, Answer};
use crate::workbench::launch::{FakeWindows, StartCommand};
use crate::workbench::settings::{Port, Starter};

/// The end of every answer with a loaded Handler script.
const NEXT_STEP: &str = "The Workbench Tools can use it now. Before you pack or publish the mod, \
                         delete its folder Scripts/WorkbenchGame/ReforgerBridge: Workbench packs \
                         the Handler script with the mod.";

#[path = "tools_tests_status.rs"]
mod status;

#[path = "tools_tests_launch.rs"]
mod launch;

// region:    --- Test Support

/// A Workbench folder in Documents with a project list, and mod projects in `Mods/<ID>` next to it.
struct Setup {
    documents: tempfile::TempDir,

    /// The setting of the folder of the downloaded mods.
    downloads: Option<String>,
}

impl Setup {
    /// The mod project `BridgeTest` in the project list.
    fn new() -> Self {
        Self::with_projects(&[("BridgeTest", "6AE15995AB330875", &[])])
    }

    /// The mod projects `(ID, GUID, GUIDs of the dependencies)` in the project list.
    fn with_projects(projects: &[(&str, &str, &[&str])]) -> Self {
        let documents = tempfile::tempdir().unwrap();
        let setup = Self {
            documents,
            downloads: None,
        };
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

    /// Adds a packed project, like the game or a mod from the Workshop: a `.gproj` file next to
    /// a `data.pak`, in a second project list, as Workbench 1.6 lists them.
    fn with_packed(self, id: &str, guid: &str) -> Self {
        let folder = self.project(id);
        fs::create_dir_all(&folder).unwrap();
        let file = folder.join("addon.gproj");
        fs::write(
            &file,
            format!("GameProject {{\n ID \"{id}\"\n GUID \"{guid}\"\n TITLE \"{id}\"\n}}"),
        )
        .unwrap();
        fs::write(folder.join("data.pak"), "").unwrap();
        let list = self
            .documents
            .path()
            .join("profile/.projectList_app1874910_user1.conf");
        let mut text = fs::read_to_string(&list)
            .unwrap_or_else(|_| "WBProjectList {\n Projects {\n }\n}\n".to_owned());
        let item = format!(
            "  WBProjectListItem {{\n   FilePath \"{}\"\n  }}\n }}\n}}\n",
            file.display()
        );
        text.truncate(text.rfind(" }\n}").unwrap());
        text.push_str(&item);
        fs::write(&list, text).unwrap();
        self
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
        let settings = self.settings(port, None, Starter::PowerShell);
        let client = test_support::connect(router::<Server>(settings)).await;
        test_support::call(&client, "workbench_status", json!({})).await
    }

    /// Calls `workbench_launch` with `input`, Workbench on 127.0.0.1 at `port`, the Steam
    /// folder `steam`, a user logged in to Steam, and a fake start that calls `start`.
    async fn launch(
        &self,
        port: u16,
        steam: &Path,
        start: impl Fn(&StartCommand) + Send + Sync + 'static,
        input: Value,
    ) -> Answer {
        let windows = Arc::new(fake_windows(start, true, true));
        self.launch_on(port, steam, windows, input).await
    }

    /// Calls `workbench_launch` like `launch`, on the fake Windows `windows`.
    async fn launch_on(
        &self,
        port: u16,
        steam: &Path,
        windows: Arc<FakeWindows>,
        input: Value,
    ) -> Answer {
        let settings = self.settings(
            Port::Valid(port),
            Some(steam.to_str().unwrap().to_owned()),
            Starter::Test(windows),
        );
        let client = test_support::connect(router::<Server>(settings)).await;
        test_support::call(&client, "workbench_launch", input).await
    }

    fn settings(&self, port: Port, steam: Option<String>, starter: Starter) -> Settings {
        Settings {
            host: Some("127.0.0.1".to_owned()),
            port,
            documents: Some(self.documents.path().to_str().unwrap().to_owned()),
            downloads: self.downloads.clone(),
            connect_timeout: Duration::from_secs(2),
            answer_timeout: Duration::from_millis(300),
            reload_timeout: Duration::from_secs(2),
            steam,
            starter,
            steam_timeout: Duration::from_millis(300),
            start_timeout: Duration::from_millis(600),
            poll_interval: Duration::from_millis(50),
        }
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

/// A fake Windows that starts Workbench with `start`. `logged_in`: a user is logged in to Steam.
/// `logs_in`: a start of Steam logs the user in.
fn fake_windows(
    start: impl Fn(&StartCommand) + Send + Sync + 'static,
    logged_in: bool,
    logs_in: bool,
) -> FakeWindows {
    FakeWindows {
        start_workbench: Box::new(start),
        steam_logged_in: AtomicBool::new(logged_in),
        steam_logs_in: logs_in,
        steam_starts: Mutex::new(Vec::new()),
    }
}

/// A fake start: it records the start command in `started` and starts a fake Workbench with
/// `reply` on `port`.
fn start_fake_workbench(
    port: u16,
    reply: Reply,
    started: &Arc<Mutex<Vec<String>>>,
) -> impl Fn(&StartCommand) + Send + Sync + 'static {
    let reply = Arc::new(reply);
    let started = Arc::clone(started);
    move |command: &StartCommand| {
        started.lock().unwrap().push(format!("{command:?}"));
        let listener = std::net::TcpListener::bind(("127.0.0.1", port)).unwrap();
        listener.set_nonblocking(true).unwrap();
        serve(TcpListener::from_std(listener).unwrap(), Arc::clone(&reply));
    }
}

// endregion: --- Test Support
