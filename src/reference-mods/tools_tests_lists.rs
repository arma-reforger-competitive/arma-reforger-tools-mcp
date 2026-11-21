//! The tests of `reference_sync` that read the lists and the record, and delete mods.

use std::fs;

use super::*;

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
