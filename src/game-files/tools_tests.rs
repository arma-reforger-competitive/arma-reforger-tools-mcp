use std::fs;
use std::path::{Path, PathBuf};

use rmcp::handler::server::router::tool::ToolRouter;
use serde_json::json;

use super::*;
use crate::game_files::test_pak::PakBuilder;
use crate::server::Server;
use crate::test_support::{self, Answer};

#[tokio::test]
async fn finds_the_script_of_a_class_by_a_part_of_its_name() {
    // -- Setup & Fixtures
    let game = game_folder(
        "finds_the_script_of_a_class_by_a_part_of_its_name",
        CAMERA_FILES,
    );

    // -- Exec
    let answer = find_in(
        settings(&game, None),
        json!({ "query": "CharacterCameraHandler" }),
    )
    .await;

    // -- Check
    // From the game: CharacterCameraHandlerComponent.c defines SCR_CharacterCameraHandlerComponent.
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer.text.starts_with(
            "2 Game files match \"CharacterCameraHandler\":\n\
             - scripts/Game/Character/CharacterCameraHandlerComponent.c (game)\n\
             - scripts/Game/Plugins/Persistence/System/Serializers/Components/Character/\
             SCR_CharacterCameraHandlerComponentSerializer.c (game)\n\n\
             REFORGER_WORKBENCH_PROFILE_PATH is not set, so the search covered the game only."
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn finds_the_script_of_a_class_by_its_full_name_with_the_scr_prefix() {
    // -- Setup & Fixtures
    let game = game_folder(
        "finds_the_script_of_a_class_by_its_full_name_with_the_scr_prefix",
        CAMERA_FILES,
    );

    // -- Exec
    let query = json!({ "query": "SCR_CharacterCameraHandlerComponent" });
    let answer = find_in(settings(&game, None), query).await;

    // -- Check
    let first = answer.text.lines().nth(1).unwrap_or_default();
    assert_eq!(
        first,
        "- scripts/Game/Character/CharacterCameraHandlerComponent.c (game)"
    );
}

#[tokio::test]
async fn names_the_mod_of_a_file_in_a_downloaded_workshop_mod() {
    // -- Setup & Fixtures
    let test = "names_the_mod_of_a_file_in_a_downloaded_workshop_mod";
    let game = game_folder(test, CAMERA_FILES);
    let profile = game.with_file_name("profile");
    write_pak(
        &profile.join("addons/tvtLobby_5F2A6A1D2A3B4C5D/data.pak"),
        &["Scripts/Game/tvtLobby/Adapters/tvtLobby_AIGroup.c"],
    );

    // -- Exec
    let answer = find_in(
        settings(&game, Some(&profile)),
        json!({ "query": "tvtLobby_AIGroup" }),
    )
    .await;

    // -- Check
    assert_eq!(
        answer.text,
        "1 Game files match \"tvtLobby_AIGroup\":\n\
         - Scripts/Game/tvtLobby/Adapters/tvtLobby_AIGroup.c (mod tvtLobby)"
    );
}

#[tokio::test]
async fn names_the_setting_when_the_game_folder_is_not_set() {
    // -- Exec
    let answer = find_in(Settings::default(), json!({ "query": "Camera" })).await;

    // -- Check
    assert!(answer.is_error);
    assert!(
        answer
            .text
            .starts_with("REFORGER_GAME_PATH is not set. Set it to the game folder"),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn names_the_setting_when_the_game_folder_has_no_pak_files() {
    // -- Setup & Fixtures
    let folder = out_folder("names_the_setting_when_the_game_folder_has_no_pak_files");
    let settings = Settings {
        game: Some(folder.display().to_string()),
        profile: None,
    };

    // -- Exec
    let answer = find_in(settings, json!({ "query": "Camera" })).await;

    // -- Check
    assert!(answer.is_error);
    assert!(
        answer.text.starts_with(&format!(
            "REFORGER_GAME_PATH is \"{}\", but {} has no addons folder with .pak files.",
            folder.display(),
            folder.display()
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn lists_only_the_limit_and_says_how_many_files_match() {
    // -- Setup & Fixtures
    let game = game_folder(
        "lists_only_the_limit_and_says_how_many_files_match",
        CAMERA_FILES,
    );

    // -- Exec
    let answer = find_in(
        settings(&game, None),
        json!({ "query": "Character", "limit": 1 }),
    )
    .await;

    // -- Check
    assert!(
        answer.text.starts_with(
            "3 Game files match \"Character\". The first 1 come below. Give a longer part of the \
             name to narrow the search, or a larger limit (at most 100).\n\
             - Prefabs/Characters/Core/Character_Base.et (game)\n\n"
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn says_how_many_files_it_searched_when_no_file_matches() {
    // -- Setup & Fixtures
    let test = "says_how_many_files_it_searched_when_no_file_matches";
    let game = game_folder(test, CAMERA_FILES);
    let profile = game.with_file_name("profile");
    write_pak(
        &profile.join("addons/tvtLobby_5F2A6A1D2A3B4C5D/data.pak"),
        &["a/b.c"],
    );

    // -- Exec
    let answer = find_in(
        settings(&game, Some(&profile)),
        json!({ "query": "Helicopter" }),
    )
    .await;

    // -- Check
    assert!(!answer.is_error);
    assert!(
        answer.text.starts_with(
            "No Game file has \"Helicopter\" in its name or path. Searched 5 files in the game \
             and 1 mod (tvtLobby)."
        ),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn skips_a_damaged_pak_file_and_names_it() {
    // -- Setup & Fixtures
    let game = game_folder("skips_a_damaged_pak_file_and_names_it", CAMERA_FILES);
    let damaged = game.join("addons/data/data001.pak");
    fs::write(&damaged, b"FORM\0\0\0\0PAC1").unwrap();

    // -- Exec
    let answer = find_in(
        settings(&game, None),
        json!({ "query": "CharacterCameraHandler" }),
    )
    .await;

    // -- Check
    assert!(
        answer.text.starts_with("2 Game files match"),
        "{}",
        answer.text
    );
    assert!(
        answer.text.contains(&format!(
            "Skipped {}: it has no FILE table. The file may be damaged",
            damaged.display()
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn asks_for_a_name_when_the_query_is_empty() {
    // -- Setup & Fixtures
    let game = game_folder("asks_for_a_name_when_the_query_is_empty", CAMERA_FILES);

    // -- Exec
    let answer = find_in(settings(&game, None), json!({ "query": "  " })).await;

    // -- Check
    assert!(answer.is_error);
    assert!(
        answer.text.contains("\"CharacterCameraHandler\""),
        "{}",
        answer.text
    );
}

/// Needs the game: set `REFORGER_GAME_PATH` to the game folder in WSL form, then run
/// `cargo nextest run --run-ignored only`.
#[tokio::test]
#[ignore = "needs the installed game"]
async fn finds_the_camera_handler_script_in_the_installed_game_by_a_windows_and_a_wsl_path() {
    // -- Setup & Fixtures
    let wsl = Settings::from_env()
        .game
        .expect("REFORGER_GAME_PATH is not set");
    let windows = wsl
        .strip_prefix("/mnt/")
        .and_then(|rest| rest.split_once('/'))
        .map(|(drive, rest)| format!("{}:\\{}", drive.to_uppercase(), rest.replace('/', "\\")))
        .expect("REFORGER_GAME_PATH is not a WSL path under /mnt/<drive>");

    // -- Exec & Check
    for game in [wsl, windows] {
        let settings = Settings {
            game: Some(game.clone()),
            profile: None,
        };
        let answer = find_in(settings, json!({ "query": "CharacterCameraHandler" })).await;
        let first = answer.text.lines().nth(1).unwrap_or_default();
        assert_eq!(
            first, "- scripts/Game/Character/CharacterCameraHandlerComponent.c (game)",
            "{game}: {}",
            answer.text
        );
    }
}

// -- Test Support

/// Three scripts from the game, and one prefab.
const CAMERA_FILES: &[&str] = &[
    "scripts/Game/Character/CharacterCameraHandlerComponent.c",
    "scripts/Game/Plugins/Persistence/System/Serializers/Components/Character/\
     SCR_CharacterCameraHandlerComponentSerializer.c",
    "scripts/Game/GameMode/SCR_PreGameGameModeStateComponent.c",
    "Prefabs/Characters/Core/Character_Base.et",
];

async fn find_in(settings: Settings, arguments: serde_json::Value) -> Answer {
    let tools: ToolRouter<Server> = router(settings);
    let client = test_support::connect(tools).await;
    test_support::call(&client, "game_find", arguments).await
}

fn settings(game: &Path, profile: Option<&Path>) -> Settings {
    Settings {
        game: Some(game.display().to_string()),
        profile: profile.map(|p| p.display().to_string()),
    }
}

/// A game folder with `files` in `addons/data/data.pak`.
fn game_folder(test: &str, files: &[&str]) -> PathBuf {
    let game = out_folder(test).join("game");
    write_pak(&game.join("addons/data/data.pak"), files);
    game
}

fn write_pak(pak: &Path, files: &[&str]) {
    fs::create_dir_all(pak.parent().unwrap()).unwrap();
    fs::write(pak, PakBuilder::with_files(files).build()).unwrap();
}

/// An empty folder for the files of one test: `tests/.out/<test>`.
fn out_folder(test: &str) -> PathBuf {
    let folder = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/.out")
        .join(test);
    assert!(folder.to_string_lossy().contains("tests/.out"));
    let _ = fs::remove_dir_all(&folder);
    fs::create_dir_all(&folder).unwrap();
    folder
}
