//! The tests of `workbench_launch`.

use std::fs;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

use serde_json::json;

use super::*;
use crate::test_support::fake_workbench::{Reload, Reply, fake_workbench, free_port};
use crate::workbench::install::HANDLER_SCRIPT;
use crate::workbench::launch::{FakeWindows, StartCommand};
use crate::workbench::steam::{App, write_steam};

#[tokio::test]
async fn starts_workbench_with_the_one_mod_and_waits_for_the_scripts() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let reply = Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads);

    // -- Exec
    let answer = setup
        .launch(
            port,
            steam.path(),
            start_fake_workbench(port, reply, &started),
            json!({}),
        )
        .await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        format!(
            "Workbench started with the mod BridgeTest and compiled the scripts. Workbench is \
             running, and the Handler script is loaded (version {}). The open mod project is \
             BridgeTest in {}. {NEXT_STEP}",
            env!("CARGO_PKG_VERSION"),
            setup.project("BridgeTest").display()
        )
    );
    let started = started.lock().unwrap();
    let [command] = started.as_slice() else {
        panic!("it started Workbench {} times", started.len());
    };
    assert!(
        command.contains(r#"\\Workbench\\ArmaReforgerWorkbenchSteamDiag.exe""#),
        "{command}"
    );
    assert!(
        command.contains(r#"\\steamapps\\common\\Arma Reforger""#),
        "{command}"
    );
    assert!(
        command.contains(&format!(
            "gproj: Some({:?})",
            setup
                .project("BridgeTest")
                .join("addon.gproj")
                .display()
                .to_string()
        )),
        "{command}"
    );
    assert_eq!(
        fs::read_to_string(setup.handler_file()).unwrap(),
        HANDLER_SCRIPT,
        "it did not copy the Handler script before the start"
    );
}

#[tokio::test]
async fn starts_nothing_when_workbench_runs_already() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    let port = fake_workbench(Reply::workbench(
        Some(env!("CARGO_PKG_VERSION")),
        Reload::Loads,
    ))
    .await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let record = Arc::clone(&started);

    // -- Exec
    let answer = setup
        .launch(
            port,
            steam.path(),
            move |command: &StartCommand| record.lock().unwrap().push(format!("{command:?}")),
            json!({}),
        )
        .await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(
            "Workbench was running already, so workbench_launch started nothing. Workbench is \
             running, and the Handler script is loaded"
        ),
        "{}",
        answer.text
    );
    assert!(started.lock().unwrap().is_empty());
}

#[tokio::test]
async fn starts_steam_first_when_no_user_is_logged_in() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let start = start_fake_workbench(
        port,
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads),
        &started,
    );
    // Workbench closes when it starts without Steam, so the fake checks the login at the start.
    let windows = Arc::new_cyclic(|windows: &std::sync::Weak<FakeWindows>| {
        let windows = windows.clone();
        fake_windows(
            move |command: &StartCommand| {
                let windows = windows.upgrade().unwrap();
                assert!(windows.steam_logged_in.load(Ordering::SeqCst));
                start(command);
            },
            false,
            true,
        )
    });

    // -- Exec
    let answer = setup
        .launch_on(port, steam.path(), Arc::clone(&windows), json!({}))
        .await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(
            "No user was logged in to Steam, so workbench_launch started Steam. Workbench \
             started with the mod BridgeTest and compiled the scripts."
        ),
        "{}",
        answer.text
    );
    assert_eq!(
        *windows.steam_starts.lock().unwrap(),
        [format!(r"{}\steam.exe", steam.path().display())]
    );
    assert_eq!(started.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn starts_no_steam_when_a_user_is_logged_in() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let start = start_fake_workbench(
        port,
        Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads),
        &started,
    );
    let windows = Arc::new(fake_windows(start, true, true));

    // -- Exec
    let answer = setup
        .launch_on(port, steam.path(), Arc::clone(&windows), json!({}))
        .await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("Workbench started with the mod BridgeTest"),
        "{}",
        answer.text
    );
    assert!(windows.steam_starts.lock().unwrap().is_empty());
}

#[tokio::test]
async fn says_to_log_in_to_steam_when_no_user_logs_in() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let started = Arc::new(Mutex::new(Vec::new()));
    let record = Arc::clone(&started);
    let windows = Arc::new(fake_windows(
        move |command: &StartCommand| record.lock().unwrap().push(format!("{command:?}")),
        false,
        false,
    ));

    // -- Exec
    let answer = setup
        .launch_on(
            free_port().await,
            steam.path(),
            Arc::clone(&windows),
            json!({}),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        "Workbench needs Steam with a logged-in user. The bridge started Steam, but no user \
         logged in within 0 seconds. Ask the user to log in to Steam, or to wait until the Steam \
         update ends. Then call workbench_launch again."
    );
    assert_eq!(windows.steam_starts.lock().unwrap().len(), 1);
    assert!(started.lock().unwrap().is_empty(), "it started Workbench");
    assert!(
        !setup.handler_file().exists(),
        "it copied the Handler script into the mod"
    );
}

#[tokio::test]
async fn says_how_to_install_arma_reforger_tools_when_steam_does_not_have_them() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game]);

    // -- Exec
    let answer = setup
        .launch(
            free_port().await,
            steam.path(),
            |_: &StartCommand| {},
            json!({}),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        "Arma Reforger Tools is not installed in Steam. Install it: open \
         steam://install/1874910 in a browser, or find Arma Reforger Tools in the Steam library \
         and click Install. Then call workbench_launch again."
    );
}

#[tokio::test]
async fn says_how_to_install_the_game_when_steam_does_not_have_it() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Tools]);

    // -- Exec
    let answer = setup
        .launch(
            free_port().await,
            steam.path(),
            |_: &StartCommand| {},
            json!({}),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("Arma Reforger is not installed in Steam."),
        "{}",
        answer.text
    );
    assert!(answer.text.contains("steam://install/1874880"));
}

#[tokio::test]
async fn says_how_to_install_steam_when_the_steam_folder_has_no_library() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();

    // -- Exec
    let answer = setup
        .launch(
            free_port().await,
            steam.path(),
            |_: &StartCommand| {},
            json!({}),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with("The bridge did not find Steam"),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn names_the_mods_when_there_are_many_and_claude_named_none() {
    // -- Setup & Fixtures
    let setup = Setup::with_projects(&[
        ("Alpha", "4444444444444444", &[]),
        ("Bravo", "5555555555555555", &[]),
    ]);
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);

    // -- Exec
    let answer = setup
        .launch(
            free_port().await,
            steam.path(),
            |_: &StartCommand| {},
            json!({}),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        "The project list of Workbench has more than one mod: Alpha, Bravo. Call \
         workbench_launch again with `mod` set to one of them."
    );
}

#[tokio::test]
async fn names_the_real_mods_when_the_mod_is_unknown() {
    // -- Setup & Fixtures
    let setup = Setup::with_projects(&[
        ("Alpha", "4444444444444444", &[]),
        ("Bravo", "5555555555555555", &[]),
    ]);
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);

    // -- Exec
    let answer = setup
        .launch(
            free_port().await,
            steam.path(),
            |_: &StartCommand| {},
            json!({ "mod": "Charlie" }),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert_eq!(
        answer.text,
        "The project list of Workbench has no mod \"Charlie\". Its mods: Alpha, Bravo. Call \
         workbench_launch again with one of them."
    );
}

#[tokio::test]
async fn opens_the_mod_that_claude_named_in_any_case() {
    // -- Setup & Fixtures
    let setup = Setup::with_projects(&[
        ("Alpha", "4444444444444444", &[]),
        ("Bravo", "5555555555555555", &[]),
    ]);
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let reply = Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads)
        .loading(&["ArmaReforger", "Bravo"]);

    // -- Exec
    let answer = setup
        .launch(
            port,
            steam.path(),
            start_fake_workbench(port, reply, &started),
            json!({ "mod": "bravo" }),
        )
        .await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("Workbench started with the mod Bravo and compiled the scripts."),
        "{}",
        answer.text
    );
    assert!(started.lock().unwrap()[0].contains("Bravo"));
}

#[tokio::test]
async fn opens_the_launcher_when_the_user_has_no_mod() {
    // -- Setup & Fixtures
    let setup = Setup::with_projects(&[]);
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let started = Arc::new(Mutex::new(Vec::new()));
    let record = Arc::clone(&started);

    // -- Exec
    let answer = setup
        .launch(
            free_port().await,
            steam.path(),
            move |command: &StartCommand| record.lock().unwrap().push(format!("{command:?}")),
            json!({}),
        )
        .await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("Workbench started on its launcher, because the project list"),
        "{}",
        answer.text
    );
    assert!(
        started.lock().unwrap()[0].contains("gproj: None"),
        "{:?}",
        started.lock().unwrap()
    );
}

#[tokio::test]
async fn says_to_fix_the_scripts_when_a_started_workbench_does_not_compile_them() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let reply = Reply::workbench(None, Reload::CompilationFailed);

    // -- Exec
    let answer = setup
        .launch(
            port,
            steam.path(),
            start_fake_workbench(port, reply, &started),
            json!({}),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(
            "Workbench was started, but its scripts were not compiled after 0 seconds. Open the \
             Script Editor"
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn says_to_turn_on_the_net_api_when_a_started_workbench_never_answers() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);

    // -- Exec
    let answer = setup
        .launch(
            free_port().await,
            steam.path(),
            |_: &StartCommand| {},
            json!({}),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(
            "Workbench was started, but its NET API did not answer in 0 seconds. In Workbench, \
             turn on File > Options > General > Net API"
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn names_the_folder_of_the_downloaded_mods_in_the_start() {
    // -- Setup & Fixtures
    let mut setup = Setup::new();
    setup.downloads = Some("/mnt/d/DownloadedMods".to_owned());
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let reply = Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads);

    // -- Exec
    let answer = setup
        .launch(
            port,
            steam.path(),
            start_fake_workbench(port, reply, &started),
            json!({}),
        )
        .await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    let started = started.lock().unwrap();
    assert!(
        started[0].contains(r#"addons: Some("D:\\DownloadedMods\\addons")"#),
        "{}",
        started[0]
    );
}

#[tokio::test]
async fn gives_the_log_errors_when_a_started_workbench_does_not_load_the_mod() {
    // -- Setup & Fixtures
    let setup = Setup::new();
    // The error.log of Workbench 1.6 when a dependency of BridgeTest was not found. An older
    // log folder has another error.
    for (folder, line) in [
        (
            "logs_2026-11-16_22-56-50",
            "Addon 'Old' dependency 'AAAAAAAAAAAAAAAA' can't be added",
        ),
        (
            "logs_2026-11-17_02-19-25",
            "Addon 'BridgeTest' dependency '672B195EAD3036D4' can't be added",
        ),
    ] {
        let logs = setup.documents.path().join("logs").join(folder);
        fs::create_dir_all(&logs).unwrap();
        fs::write(
            logs.join("error.log"),
            format!(
                "Log error.log started at 2026-11-17 02:19:26\n02:19:26.290   ENGINE    (E): {line}\n\
                 02:19:29.368    SCRIPT    (W): 'GetRefCount' is obsolete\n"
            ),
        )
        .unwrap();
    }
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let reply = Reply::workbench(None, Reload::Loads).loading(&["ArmaReforger"]);

    // -- Exec
    let answer = setup
        .launch(
            port,
            steam.path(),
            start_fake_workbench(port, reply, &started),
            json!({ "mod": "BridgeTest" }),
        )
        .await;

    // -- Check
    assert!(answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(
            "Workbench started, but it did not load the mod BridgeTest: it has only the game \
             open. Errors in the Workbench log: Addon 'BridgeTest' dependency '672B195EAD3036D4' \
             can't be added. When a dependency"
        ),
        "{}",
        answer.text
    );
    assert!(
        answer.text.contains("REFORGER_WORKBENCH_PROFILE_PATH"),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn never_offers_a_downloaded_mod_to_open() {
    // -- Setup & Fixtures
    let setup = Setup::new().with_packed("CoopSlotLobby", "672B195EAD3036D4");
    let steam = tempfile::tempdir().unwrap();
    write_steam(steam.path(), &[App::Game, App::Tools]);
    let port = free_port().await;
    let started = Arc::new(Mutex::new(Vec::new()));
    let reply = Reply::workbench(Some(env!("CARGO_PKG_VERSION")), Reload::Loads);

    // -- Exec
    let named = setup
        .launch(
            free_port().await,
            steam.path(),
            |_: &StartCommand| {},
            json!({ "mod": "CoopSlotLobby" }),
        )
        .await;
    let unnamed = setup
        .launch(
            port,
            steam.path(),
            start_fake_workbench(port, reply, &started),
            json!({}),
        )
        .await;

    // -- Check
    assert!(named.is_error, "{}", named.text);
    assert!(
        named.text.starts_with(
            "The project list of Workbench has no mod \"CoopSlotLobby\". Its mods: BridgeTest."
        ),
        "{}",
        named.text
    );
    assert!(!unnamed.is_error, "{}", unnamed.text);
    assert!(
        unnamed
            .text
            .starts_with("Workbench started with the mod BridgeTest"),
        "{}",
        unnamed.text
    );
}
