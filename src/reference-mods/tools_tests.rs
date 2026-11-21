use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::json;
use tempfile::TempDir;

use super::*;
use crate::server::Server;
use crate::test_support::fake_workbench::free_port;
use crate::test_support::{self, Answer};
use crate::workbench::Bridge;

const KEPT: &str = "6AA81EE549F50CB7";
const DROPPED: &str = "5965770215E93269";
const BY_HAND: &str = "591AF5BDA9F7CE8B";
const MISSING: &str = "6A09759015FFDA26";

#[path = "tools_tests_lists.rs"]
mod lists;

#[path = "tools_tests_downloads.rs"]
mod downloads;

// -- Test Support

/// A temp folder of mod repos and a temp Workbench folder in Documents.
struct Setup {
    mods: TempDir,
    documents: TempDir,
}

impl Setup {
    fn new() -> Self {
        let setup = Self {
            mods: TempDir::new().unwrap(),
            documents: TempDir::new().unwrap(),
        };
        fs::create_dir(setup.addons()).unwrap();
        setup
    }

    fn addons(&self) -> PathBuf {
        self.documents.path().join("addons")
    }

    /// Writes `text` as the list of the mod repo `repo` and returns the path of the list.
    fn list(&self, repo: &str, text: &str) -> PathBuf {
        let folder = self.mods.path().join(repo);
        fs::create_dir_all(&folder).unwrap();
        let file = folder.join("reference-mods.jsonl");
        fs::write(&file, text).unwrap();
        file
    }

    /// Makes a folder `name` in `addons` with one file in it.
    fn mod_folder(&self, name: &str) {
        let folder = self.addons().join(name);
        fs::create_dir(&folder).unwrap();
        fs::write(folder.join("data.pak"), "pak").unwrap();
    }

    fn record(&self, lines: &[String]) {
        let mut text = lines.join("\n");
        text.push('\n');
        fs::write(self.addons().join(".reference-mods.jsonl"), text).unwrap();
    }

    fn record_text(&self) -> String {
        fs::read_to_string(self.addons().join(".reference-mods.jsonl")).unwrap()
    }

    fn addons_names(&self) -> std::collections::BTreeSet<String> {
        fs::read_dir(self.addons())
            .unwrap()
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .collect()
    }

    async fn sync(&self) -> Answer {
        self.sync_with(Settings {
            mods: Some(self.mods.path().to_string_lossy().into_owned()),
            documents: Some(self.documents.path().to_string_lossy().into_owned()),
        })
        .await
    }

    async fn sync_with(&self, settings: Settings) -> Answer {
        let bridge = Bridge::for_test(free_port().await, self.documents.path());
        let client =
            test_support::connect(router_with::<Server>(settings, bridge, short_wait())).await;
        test_support::call(&client, "reference_sync", json!({})).await
    }

    /// Calls `reference_sync` once on a new server, as after a restart, which calls the fake
    /// Workbench on `port`.
    async fn call_on_new_server(&self, port: u16) -> Answer {
        let client = self.client(port).await;
        test_support::call(&client, "reference_sync", json!({})).await
    }

    /// A client of one server, which calls the fake Workbench on `port`.
    async fn client(&self, port: u16) -> test_support::TestClient {
        let settings = Settings {
            mods: Some(self.mods.path().to_string_lossy().into_owned()),
            documents: Some(self.documents.path().to_string_lossy().into_owned()),
        };
        let bridge = Bridge::for_test(port, self.documents.path());
        test_support::connect(router_with::<Server>(settings, bridge, short_wait())).await
    }
}

/// The wait for the downloads, short for the tests.
fn short_wait() -> Wait {
    Wait {
        poll: Duration::from_millis(10),
        total: Duration::from_millis(100),
    }
}

fn started_line(id: &str, name: &str) -> String {
    format!("{{\"state\":\"started\",\"id\":\"{id}\",\"name\":\"{name}\"}}\n")
}

fn record_line(id: &str, folder: &str, license: &str) -> String {
    format!(
        r#"{{"state":"downloaded","id":"{id}","folder":"{folder}","version":"1.0.0","license":"{license}"}}"#
    )
}

#[cfg(unix)]
fn link(target: &Path, at: &Path) {
    std::os::unix::fs::symlink(target, at).unwrap();
}

#[cfg(windows)]
fn link(target: &Path, at: &Path) {
    std::os::windows::fs::symlink_dir(target, at).unwrap();
}
