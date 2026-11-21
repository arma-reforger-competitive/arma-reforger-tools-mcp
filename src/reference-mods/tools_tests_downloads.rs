//! The tests of `reference_sync` that download mods with Workbench.

use std::fs;
use std::sync::Arc;
use std::time::Duration;

use serde_json::json;

use super::*;
use crate::test_support::fake_workbench::{Reload, Reply, Workshop, fake_workbench, free_port};
use crate::test_support::{self};

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

#[tokio::test]
async fn labels_reference_sync_as_destructive_and_gives_the_license_rule() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let client = setup.client(free_port().await).await;

    // -- Exec
    let tools = client.list_all_tools().await.unwrap();

    // -- Check
    let tool = tools
        .iter()
        .find(|tool| tool.name == "reference_sync")
        .unwrap();
    let labels = tool.annotations.clone().unwrap();
    assert_eq!(labels.read_only_hint, Some(false));
    assert_eq!(labels.destructive_hint, Some(true));
    assert_eq!(labels.idempotent_hint, Some(true));
    assert_eq!(labels.open_world_hint, Some(true));
    let description = tool.description.as_deref().unwrap_or_default();
    assert!(
        description.contains(
            "read any Reference mod; copy code only when its license allows it; otherwise \
             write the feature again in your own way."
        ),
        "{description}"
    );
}
