use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use serde_json::json;
use tempfile::TempDir;

use super::*;
use crate::server::Server;
use crate::test_support::fake_workbench::{Reload, Reply, Workshop, fake_workbench, free_port};
use crate::test_support::{self, Answer};
use crate::workbench::Bridge;

const KEPT: &str = "6AA81EE549F50CB7";
const DROPPED: &str = "5965770215E93269";
const BY_HAND: &str = "591AF5BDA9F7CE8B";
const MISSING: &str = "6A09759015FFDA26";

#[tokio::test]
async fn deletes_only_a_recorded_mod_that_no_list_has_without_workbench() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{KEPT}","name":"tvtLobby"}}"#),
    );
    setup.mod_folder(&format!("tvtLobby_{KEPT}"));
    setup.mod_folder(&format!("OldMod_{DROPPED}"));
    setup.mod_folder(&format!("ByHand_{BY_HAND}"));
    setup.mod_folder("MyProject");
    setup.mod_folder("saves");
    let target = setup.mods.path().join("TvtMission");
    link(&target, &setup.addons().join("Linked"));
    setup.record(&[
        record_line(KEPT, &format!("tvtLobby_{KEPT}"), "MIT"),
        record_line(DROPPED, &format!("OldMod_{DROPPED}"), "APL-SA"),
    ]);

    // -- Exec
    let answer = setup.sync().await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "tvtLobby ({KEPT}), license MIT: present\nOldMod ({DROPPED}), license APL-SA: deleted"
        )
    );
    assert_eq!(
        setup.addons_names(),
        vec![
            format!("ByHand_{BY_HAND}"),
            "Linked".to_owned(),
            "MyProject".to_owned(),
            ".reference-mods.jsonl".to_owned(),
            "saves".to_owned(),
            format!("tvtLobby_{KEPT}"),
        ]
        .into_iter()
        .collect::<std::collections::BTreeSet<_>>()
    );
    assert_eq!(
        setup.record_text(),
        format!(
            "{}\n",
            record_line(KEPT, &format!("tvtLobby_{KEPT}"), "MIT")
        )
    );
    assert!(target.is_dir(), "the target of the link was deleted");
}

#[tokio::test]
async fn gives_a_listed_mod_that_is_on_disk_and_not_recorded_an_unknown_license() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{}","name":"handMod"}}"#, BY_HAND.to_lowercase()),
    );
    setup.mod_folder(&format!("ByHand_{BY_HAND}"));

    // -- Exec
    let answer = setup.sync().await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!("handMod ({BY_HAND}), license unknown: present")
    );
    assert!(setup.addons().join(format!("ByHand_{BY_HAND}")).is_dir());
}

#[tokio::test]
async fn keeps_a_mod_that_one_of_two_lists_has_and_skips_a_repo_with_no_list() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list("RepoA", "");
    setup.list(
        "RepoB",
        &format!("\n{{\"id\":\"{KEPT}\",\"name\":\"tvtLobby\"}}\n\n"),
    );
    fs::create_dir(setup.mods.path().join("NoList")).unwrap();
    setup.mod_folder(&format!("tvtLobby_{KEPT}"));
    setup.record(&[record_line(KEPT, &format!("tvtLobby_{KEPT}"), "MIT")]);

    // -- Exec
    let answer = setup.sync().await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!("tvtLobby ({KEPT}), license MIT: present")
    );
    assert!(setup.addons().join(format!("tvtLobby_{KEPT}")).is_dir());
}

#[tokio::test]
async fn gives_a_missing_mod_the_message_of_the_bridge_when_workbench_is_not_running() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{MISSING}","name":"S3C_CameraFix"}}"#),
    );
    setup.mod_folder(&format!("OldMod_{DROPPED}"));
    setup.record(&[record_line(DROPPED, &format!("OldMod_{DROPPED}"), "MIT")]);

    // -- Exec
    let answer = setup.sync().await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    let (first, second) = answer.text.split_once('\n').unwrap();
    assert!(
        first.starts_with(&format!(
            "S3C_CameraFix ({MISSING}), license unknown: failed: Workbench is not running: \
             nothing answers on 127.0.0.1:"
        )),
        "{first}"
    );
    assert_eq!(second, format!("OldMod ({DROPPED}), license MIT: deleted"));
    assert!(!setup.addons().join(format!("OldMod_{DROPPED}")).exists());
}

#[tokio::test]
async fn stops_before_any_delete_at_a_line_that_is_not_valid() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let file = setup.list(
        "TvtMission",
        &format!(
            "{{\"id\":\"{KEPT}\",\"name\":\"tvtLobby\"}}\n{{\"id\":\"6AA81EE5\",\"name\":\"x\"}}\n"
        ),
    );
    setup.mod_folder(&format!("OldMod_{DROPPED}"));
    setup.record(&[record_line(DROPPED, &format!("OldMod_{DROPPED}"), "MIT")]);

    // -- Exec
    let answer = setup.sync().await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with(&format!("Line 2 of {} is not valid:", file.display())),
        "{}",
        answer.text
    );
    assert!(answer.text.contains("16 hex digits"), "{}", answer.text);
    assert!(setup.addons().join(format!("OldMod_{DROPPED}")).is_dir());
}

#[tokio::test]
async fn stops_at_a_line_with_another_field_or_an_empty_name() {
    for line in [
        format!(r#"{{"id":"{KEPT}","name":"tvtLobby","version":"1.0"}}"#),
        format!(r#"{{"id":"{KEPT}","name":" "}}"#),
        "not json".to_owned(),
    ] {
        // -- Setup & Fixtures
        let setup = Setup::new();
        let file = setup.list("TvtMission", &line);

        // -- Exec
        let answer = setup.sync().await;

        // -- Check
        assert!(answer.is_error, "{line}: {}", answer.text);
        assert!(
            answer
                .text
                .starts_with(&format!("Line 1 of {} is not valid:", file.display())),
            "{line}: {}",
            answer.text
        );
    }
}

#[tokio::test]
async fn stops_before_any_delete_when_the_mods_path_is_not_set_or_not_a_folder() {
    for mods in [None, Some("/no/such/folder/for/reference/mods".to_owned())] {
        // -- Setup & Fixtures
        let setup = Setup::new();
        setup.mod_folder(&format!("OldMod_{DROPPED}"));
        setup.record(&[record_line(DROPPED, &format!("OldMod_{DROPPED}"), "MIT")]);
        let settings = Settings {
            mods: mods.clone(),
            documents: Some(setup.documents.path().to_string_lossy().into_owned()),
        };

        // -- Exec
        let answer = setup.sync_with(settings).await;

        // -- Check
        assert!(answer.is_error, "{mods:?}: {}", answer.text);
        assert!(
            answer.text.contains("REFORGER_MODS_PATH"),
            "{mods:?}: {}",
            answer.text
        );
        assert!(setup.addons().join(format!("OldMod_{DROPPED}")).is_dir());
    }
}

#[tokio::test]
async fn reports_a_running_download_then_records_it_on_the_next_call_without_a_second_start() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{MISSING}","name":"S3C_CameraFix"}}"#),
    );
    let workshop = Workshop::new(setup.addons());
    workshop.add(MISSING, "S3C_CameraFix", "1.0.2", "APL-SA", 40);
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
            .with_workshop(Arc::clone(&workshop)),
    )
    .await;
    let client = setup.client(port).await;

    // -- Exec
    let first = test_support::call(&client, "reference_sync", json!({})).await;
    let record_after_first = setup.record_text();
    workshop.finish(MISSING);
    let second = test_support::call(&client, "reference_sync", json!({})).await;

    // -- Check
    assert!(!first.is_error, "{}", first.text);
    assert_eq!(
        first.text,
        format!("S3C_CameraFix ({MISSING}), license unknown: downloading 40%")
    );
    assert_eq!(record_after_first, started_line(MISSING, "S3C_CameraFix"));
    assert!(!second.is_error, "{}", second.text);
    assert_eq!(
        second.text,
        format!("S3C_CameraFix ({MISSING}), license APL-SA: downloaded")
    );
    assert_eq!(workshop.starts(MISSING), 1);
    assert_eq!(
        setup.record_text(),
        format!(
            "{{\"state\":\"downloaded\",\"id\":\"{MISSING}\",\"folder\":\"S3C_CameraFix_{MISSING}\",\"version\":\"1.0.2\",\"license\":\"APL-SA\"}}\n"
        )
    );
}

#[tokio::test]
async fn gives_an_id_that_the_workshop_does_not_know_a_failed_line_and_downloads_the_others() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(
            "{{\"id\":\"{DROPPED}\",\"name\":\"NoSuchMod\"}}\n{{\"id\":\"{MISSING}\",\"name\":\"S3C_CameraFix\"}}\n"
        ),
    );
    let workshop = Workshop::new(setup.addons());
    workshop.add(MISSING, "S3C_CameraFix", "1.0.2", "APL-SA", 100);
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
            .with_workshop(Arc::clone(&workshop)),
    )
    .await;
    let client = setup.client(port).await;

    // -- Exec
    let answer = test_support::call(&client, "reference_sync", json!({})).await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "NoSuchMod ({DROPPED}), license unknown: failed: the Workshop has no mod with this \
             ID\nS3C_CameraFix ({MISSING}), license APL-SA: downloaded"
        )
    );
}

#[tokio::test]
async fn records_a_download_that_outlives_the_server_and_deletes_it_when_its_line_goes() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let list_line = format!(r#"{{"id":"{MISSING}","name":"S3C_CameraFix"}}"#);
    setup.list("TvtMission", &list_line);
    let workshop = Workshop::new(setup.addons());
    workshop.add(MISSING, "S3C_CameraFix", "1.0.2", "APL-SA", 40);
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
            .with_workshop(Arc::clone(&workshop)),
    )
    .await;

    // -- Exec
    let first = setup.call_on_new_server(port).await;
    workshop.finish(MISSING);
    let after_restart = setup.call_on_new_server(port).await;
    setup.list("TvtMission", "");
    let after_line_goes = setup.call_on_new_server(port).await;

    // -- Check
    assert_eq!(
        first.text,
        format!("S3C_CameraFix ({MISSING}), license unknown: downloading 40%")
    );
    assert_eq!(
        after_restart.text,
        format!("S3C_CameraFix ({MISSING}), license APL-SA: downloaded")
    );
    assert_eq!(workshop.starts(MISSING), 1);
    assert_eq!(
        after_line_goes.text,
        format!("S3C_CameraFix ({MISSING}), license APL-SA: deleted")
    );
    assert!(
        !setup
            .addons()
            .join(format!("S3C_CameraFix_{MISSING}"))
            .exists()
    );
    assert_eq!(setup.record_text(), "");
}

#[tokio::test]
async fn records_a_started_download_that_workbench_forgot_when_its_folder_is_on_disk() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{MISSING}","name":"S3C_CameraFix"}}"#),
    );
    let workshop = Workshop::new(setup.addons());
    workshop.add(MISSING, "S3C_CameraFix", "1.0.2", "APL-SA", 40);
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
            .with_workshop(Arc::clone(&workshop)),
    )
    .await;
    setup.call_on_new_server(port).await;
    workshop.finish(MISSING);
    workshop.forget();

    // -- Exec
    let answer = setup.call_on_new_server(port).await;

    // -- Check
    assert_eq!(
        answer.text,
        format!("S3C_CameraFix ({MISSING}), license unknown: downloaded")
    );
    assert_eq!(workshop.starts(MISSING), 1);
    assert_eq!(
        setup.record_text(),
        format!(
            "{{\"state\":\"downloaded\",\"id\":\"{MISSING}\",\"folder\":\"S3C_CameraFix_{MISSING}\",\"version\":\"unknown\",\"license\":\"unknown\"}}\n"
        )
    );
}

#[tokio::test]
async fn starts_a_started_download_again_when_workbench_forgot_it_and_no_folder_is_on_disk() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{MISSING}","name":"S3C_CameraFix"}}"#),
    );
    let workshop = Workshop::new(setup.addons());
    workshop.add(MISSING, "S3C_CameraFix", "1.0.2", "APL-SA", 40);
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
            .with_workshop(Arc::clone(&workshop)),
    )
    .await;
    setup.call_on_new_server(port).await;
    workshop.forget();

    // -- Exec
    let answer = setup.call_on_new_server(port).await;

    // -- Check
    assert_eq!(
        answer.text,
        format!("S3C_CameraFix ({MISSING}), license unknown: downloading 40%")
    );
    assert_eq!(workshop.starts(MISSING), 2);
    assert_eq!(setup.record_text(), started_line(MISSING, "S3C_CameraFix"));
}

#[tokio::test]
async fn fails_a_complete_download_with_no_folder_and_does_not_record_it() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{MISSING}","name":"S3C_CameraFix"}}"#),
    );
    let workshop = Workshop::new(setup.addons());
    workshop.add(MISSING, "S3C_CameraFix", "1.0.2", "APL-SA", 40);
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
            .with_workshop(Arc::clone(&workshop)),
    )
    .await;
    setup.call_on_new_server(port).await;
    workshop.finish(MISSING);
    fs::remove_dir_all(setup.addons().join(format!("S3C_CameraFix_{MISSING}"))).unwrap();

    // -- Exec
    let answer = setup.call_on_new_server(port).await;

    // -- Check
    assert_eq!(
        answer.text,
        format!(
            "S3C_CameraFix ({MISSING}), license unknown: failed: Workbench says that the \
             download is complete, but addons has no folder <Name>_{MISSING}"
        )
    );
    assert_eq!(setup.record_text(), "");
}

#[tokio::test]
async fn never_records_or_deletes_a_listed_mod_that_the_user_downloaded_by_hand() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{BY_HAND}","name":"handMod"}}"#),
    );
    setup.mod_folder(&format!("ByHand_{BY_HAND}"));
    let listed = setup.sync().await;
    setup.list("TvtMission", "");

    // -- Exec
    let unlisted = setup.sync().await;

    // -- Check
    assert_eq!(
        listed.text,
        format!("handMod ({BY_HAND}), license unknown: present")
    );
    assert_eq!(
        unlisted.text,
        "No list has a Reference mod, and the record has no mod to delete."
    );
    assert!(setup.addons().join(format!("ByHand_{BY_HAND}")).is_dir());
    assert!(!setup.addons().join(".reference-mods.jsonl").exists());
}

#[tokio::test]
async fn stops_a_progress_question_at_the_end_of_the_wait() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    setup.list(
        "TvtMission",
        &format!(r#"{{"id":"{MISSING}","name":"S3C_CameraFix"}}"#),
    );
    let workshop = Workshop::new(setup.addons());
    workshop.add(MISSING, "S3C_CameraFix", "1.0.2", "APL-SA", 40);
    // Each answer comes after the wait of 100 ms, but inside the 300 ms limit of the bridge.
    workshop.answer_progress_after(Duration::from_millis(250));
    let port = fake_workbench(
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
            .with_workshop(Arc::clone(&workshop)),
    )
    .await;
    let client = setup.client(port).await;

    // -- Exec
    let begin = std::time::Instant::now();
    let answer = test_support::call(&client, "reference_sync", json!({})).await;
    let took = begin.elapsed();

    // -- Check
    assert_eq!(
        answer.text,
        format!("S3C_CameraFix ({MISSING}), license unknown: downloading 0%")
    );
    assert!(took < Duration::from_millis(200), "the call took {took:?}");
    assert_eq!(setup.record_text(), started_line(MISSING, "S3C_CameraFix"));
}

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
