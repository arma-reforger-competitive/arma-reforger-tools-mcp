//! The tests of `workbench_status`.

use std::fs;
use std::path::PathBuf;

use tokio::net::TcpListener;

use super::*;
use crate::test_support::fake_workbench::{Reload, Reply, fake_workbench};
use crate::workbench::install::HANDLER_SCRIPT;
use crate::workbench::settings::Port;
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
            "Workbench is not running: nothing answers on 127.0.0.1:{port}. Call \
             workbench_launch to start it with your mod."
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
async fn never_takes_the_game_or_a_downloaded_mod_as_the_open_mod() {
    // -- Setup & Fixtures
    // On Workbench 1.6, a mod whose dependency was not found did not load, and Workbench loaded
    // only the game and the found dependency. Both are packed and in the project list.
    let setup = Setup::new()
        .with_packed("ArmaReforger", "58D0FB3206B6F859")
        .with_packed("CoopSlotLobby", "672B195EAD3036D4");
    let before = snapshot(setup.documents.path());
    let port = fake_workbench(
        Reply::workbench(None, Reload::Loads).loading(&["ArmaReforger", "CoopSlotLobby"]),
    )
    .await;

    // -- Exec
    let answer = setup.status(port).await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("Workbench has no mod of yours open"),
        "{}",
        answer.text
    );
    assert_eq!(snapshot(setup.documents.path()), before);
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
