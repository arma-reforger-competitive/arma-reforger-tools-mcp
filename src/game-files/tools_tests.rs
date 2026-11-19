use std::fs;
use std::path::{Path, PathBuf};

use rmcp::handler::server::router::tool::ToolRouter;
use serde_json::json;

use super::*;
use crate::game_files::index::build;
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
             REFORGER_WORKBENCH_DOCUMENTS_PATH is not set and the Tool found no Workbench \
             folder in Documents, so the search covered the game only. To also search the \
             Workshop mods that Workbench downloaded, set REFORGER_WORKBENCH_DOCUMENTS_PATH"
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
async fn finds_a_mod_that_workbench_downloaded_to_documents_after_the_first_call() {
    // -- Setup & Fixtures
    let test = "finds_a_mod_that_workbench_downloaded_to_documents_after_the_first_call";
    let game = game_folder(test, CAMERA_FILES);
    let documents = game.with_file_name("documents");
    fs::create_dir_all(documents.join("addons")).unwrap();
    let settings = Settings {
        documents: Some(documents.clone()),
        ..settings(&game, None)
    };
    let client = test_support::connect(router(settings)).await;
    let query = || json!({ "query": "tvtLobby_AIGroup" });
    let before = test_support::call(&client, "game_find", query()).await;

    // -- Exec
    write_pak(
        &documents.join("addons/tvtLobby_5F2A6A1D2A3B4C5D/data.pak"),
        &["Scripts/Game/tvtLobby/Adapters/tvtLobby_AIGroup.c"],
    );
    let after = test_support::call(&client, "game_find", query()).await;

    // -- Check
    assert!(
        before.text.starts_with("No Game file has"),
        "{}",
        before.text
    );
    assert_eq!(
        after.text,
        "1 Game files match \"tvtLobby_AIGroup\":\n\
         - Scripts/Game/tvtLobby/Adapters/tvtLobby_AIGroup.c (mod tvtLobby)"
    );
}

#[tokio::test]
async fn finds_a_mod_whose_pak_file_comes_after_its_folder() {
    // -- Setup & Fixtures
    let test = "finds_a_mod_whose_pak_file_comes_after_its_folder";
    let game = game_folder(test, CAMERA_FILES);
    let documents = game.with_file_name("documents");
    let mod_folder = documents.join("addons/tvtLobby_5F2A6A1D2A3B4C5D");
    fs::create_dir_all(&mod_folder).unwrap();
    let settings = Settings {
        documents: Some(documents),
        ..settings(&game, None)
    };
    let client = test_support::connect(router(settings)).await;
    let query = || json!({ "query": "tvtLobby_AIGroup" });
    let before = test_support::call(&client, "game_find", query()).await;

    // -- Exec
    write_pak(
        &mod_folder.join("data.pak"),
        &["Scripts/Game/tvtLobby/Adapters/tvtLobby_AIGroup.c"],
    );
    let after = test_support::call(&client, "game_find", query()).await;

    // -- Check
    assert!(
        before.text.starts_with("No Game file has"),
        "{}",
        before.text
    );
    assert_eq!(
        after.text,
        "1 Game files match \"tvtLobby_AIGroup\":\n\
         - Scripts/Game/tvtLobby/Adapters/tvtLobby_AIGroup.c (mod tvtLobby)"
    );
}

#[tokio::test]
async fn names_the_profile_setting_when_it_has_no_addons_folder() {
    // -- Setup & Fixtures
    let test = "names_the_profile_setting_when_it_has_no_addons_folder";
    let game = game_folder(test, CAMERA_FILES);
    let profile = game.with_file_name("no-profile");

    // -- Exec
    let answer = find_in(
        settings(&game, Some(&profile)),
        json!({ "query": "CharacterCameraHandlerComponent.c" }),
    )
    .await;

    // -- Check
    assert!(
        answer.text.contains(&format!(
            "REFORGER_WORKBENCH_PROFILE_PATH is \"{}\", but {} is not a folder, so the search \
             covered the game only. Set it to a folder of Workshop mods that holds addons",
            profile.display(),
            profile.join("addons").display()
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn lists_a_mod_in_the_profile_and_in_documents_once_from_documents() {
    // -- Setup & Fixtures
    let test = "lists_a_mod_in_the_profile_and_in_documents_once_from_documents";
    let game = game_folder(test, CAMERA_FILES);
    let profile = game.with_file_name("profile");
    let documents = game.with_file_name("documents");
    write_pak(
        &profile.join("addons/tvtLobby_5F2A6A1D2A3B4C5D/data.pak"),
        &["Scripts/Game/tvtLobby/Old.c"],
    );
    write_pak(
        &documents.join("addons/tvtLobby_5f2a6a1d2a3b4c5d/data.pak"),
        &["Scripts/Game/tvtLobby/New.c"],
    );
    let settings = Settings {
        documents: Some(documents),
        ..settings(&game, Some(&profile))
    };

    // -- Exec
    let answer = find_in(settings, json!({ "query": "Scripts/Game/tvtLobby/" })).await;

    // -- Check
    assert_eq!(
        answer.text,
        "1 Game files match \"Scripts/Game/tvtLobby/\":\n\
         - Scripts/Game/tvtLobby/New.c (mod tvtLobby)"
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
        ..Settings::default()
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
            ..Settings::default()
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

// -- game_read

#[tokio::test]
async fn opens_the_full_text_of_a_stored_and_of_a_compressed_file() {
    // -- Setup & Fixtures
    let game = out_folder("opens_the_full_text_of_a_stored_and_of_a_compressed_file").join("game");
    let stored = "class Player {}\n".repeat(30);
    let compressed = "class Vehicle {}\n".repeat(300);
    let pak = PakBuilder::new()
        .dir("", 1)
        .dir("Scripts", 2)
        .file("Player.c", stored.as_bytes())
        .compressed_file("Vehicle.c", compressed.as_bytes());
    write_built_pak(&game.join("addons/data/data.pak"), &pak);

    // -- Exec
    let player = read_in(settings(&game, None), json!({ "path": "Scripts/Player.c" })).await;
    let vehicle = read_in(
        settings(&game, None),
        json!({ "path": "Scripts/Vehicle.c" }),
    )
    .await;

    // -- Check
    assert!(!player.is_error, "{}", player.text);
    assert_eq!(
        player.text,
        format!("Scripts/Player.c (game, 480 bytes)\n\n{stored}")
    );
    assert_eq!(
        vehicle.text,
        format!("Scripts/Vehicle.c (game, 5100 bytes)\n\n{compressed}")
    );
}

#[tokio::test]
async fn opens_the_pre_game_game_mode_state_script() {
    // -- Setup & Fixtures
    let game = game_folder("opens_the_pre_game_game_mode_state_script", CAMERA_FILES);

    // -- Exec
    let path = "scripts/Game/GameMode/SCR_PreGameGameModeStateComponent.c";
    let answer = read_in(settings(&game, None), json!({ "path": path })).await;

    // -- Check
    assert_eq!(
        answer.text,
        format!("{path} (game, 13 bytes)\n\n// test file\n")
    );
}

#[tokio::test]
async fn opens_a_path_with_other_capital_letters_and_gives_the_real_path() {
    // -- Setup & Fixtures
    let game = game_folder(
        "opens_a_path_with_other_capital_letters_and_gives_the_real_path",
        CAMERA_FILES,
    );

    // -- Exec
    // Taken from real usage: Claude wrote "Scripts" with a capital S.
    let path = "Scripts/Game/Character/CharacterCameraHandlerComponent.c";
    let answer = read_in(settings(&game, None), json!({ "path": path })).await;

    // -- Check
    assert!(!answer.is_error, "{}", answer.text);
    assert!(
        answer
            .text
            .starts_with("scripts/Game/Character/CharacterCameraHandlerComponent.c (game, "),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn suggests_the_closest_real_paths_for_a_wrong_path() {
    // -- Setup & Fixtures
    let game = game_folder(
        "suggests_the_closest_real_paths_for_a_wrong_path",
        CAMERA_FILES,
    );

    // -- Exec
    // A wrong folder and the SCR_ prefix of the class name, which the file name does not have.
    let path = "Scripts/Game/Characters/SCR_CharacterCameraHandlerComponent.c";
    let answer = read_in(settings(&game, None), json!({ "path": path })).await;

    // -- Check
    assert!(answer.is_error);
    assert!(
        answer.text.starts_with(&format!(
            "No Game file has the path \"{path}\". Searched 4 files in the game. The closest \
             real paths:\n\
             - scripts/Game/Character/CharacterCameraHandlerComponent.c (game)\n"
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn opens_the_game_file_first_and_the_mod_file_when_the_source_names_the_mod() {
    // -- Setup & Fixtures
    let test = "opens_the_game_file_first_and_the_mod_file_when_the_source_names_the_mod";
    let game = out_folder(test).join("game");
    let profile = game.with_file_name("profile");
    let path = "Configs/Shared.conf";
    let game_pak = PakBuilder::new()
        .dir("", 1)
        .dir("Configs", 1)
        .file("Shared.conf", b"game");
    let mod_pak = PakBuilder::new()
        .dir("", 1)
        .dir("Configs", 1)
        .file("Shared.conf", b"mod");
    write_built_pak(&game.join("addons/data/data.pak"), &game_pak);
    write_built_pak(
        &profile.join("addons/tvtLobby_5F2A6A1D2A3B4C5D/data.pak"),
        &mod_pak,
    );
    let settings = || settings(&game, Some(&profile));

    // -- Exec
    let first = read_in(settings(), json!({ "path": path })).await;
    let by_input = read_in(settings(), json!({ "path": path, "source": "tvtLobby" })).await;
    let by_copied_line = read_in(
        settings(),
        json!({ "path": "Configs/Shared.conf (mod tvtLobby)" }),
    )
    .await;
    let wrong_source = read_in(settings(), json!({ "path": path, "source": "Other" })).await;

    // -- Check
    assert_eq!(
        first.text,
        "Configs/Shared.conf (game, 4 bytes)\n\
         The same path is also in: mod tvtLobby. To open another one, give it as source.\n\ngame"
    );
    let from_mod = "Configs/Shared.conf (mod tvtLobby, 3 bytes)\n\
                    The same path is also in: game. To open another one, give it as source.\n\nmod";
    assert_eq!(by_input.text, from_mod);
    assert_eq!(by_copied_line.text, from_mod);
    assert!(wrong_source.is_error);
    assert!(
        wrong_source.text.starts_with(
            "\"Configs/Shared.conf\" is not in \"Other\". It is in: game, mod tvtLobby."
        ),
        "{}",
        wrong_source.text
    );
}

#[tokio::test]
async fn opens_a_path_with_back_slashes() {
    // -- Setup & Fixtures
    let game = game_folder("opens_a_path_with_back_slashes", CAMERA_FILES);

    // -- Exec
    let path = "scripts\\Game\\GameMode\\SCR_PreGameGameModeStateComponent.c";
    let answer = read_in(settings(&game, None), json!({ "path": path })).await;

    // -- Check
    assert!(
        answer
            .text
            .starts_with("scripts/Game/GameMode/SCR_PreGameGameModeStateComponent.c (game, "),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn names_the_other_pak_file_when_the_game_has_a_path_twice() {
    // -- Setup & Fixtures
    // In the real game, 34 paths are in two .pak files, for example prefabs/ai/scr_aiworld.et in
    // data/data007.pak and rally/data.pak.
    let game = out_folder("names_the_other_pak_file_when_the_game_has_a_path_twice").join("game");
    let first = game.join("addons/data/data007.pak");
    let second = game.join("addons/rally/data.pak");
    let pak = |text: &[u8]| PakBuilder::new().dir("", 1).file("World.et", text);
    write_built_pak(&first, &pak(b"data"));
    write_built_pak(&second, &pak(b"rally"));

    // -- Exec
    let answer = read_in(settings(&game, None), json!({ "path": "World.et" })).await;

    // -- Check
    assert_eq!(
        answer.text,
        format!(
            "World.et (game, 4 bytes)\n\
             This text comes from {}. The game has the same path also in: {}.\n\ndata",
            first.display(),
            second.display()
        )
    );
}

#[tokio::test]
async fn opens_a_prefab_path_with_its_guid_in_front() {
    // -- Setup & Fixtures
    let game = game_folder("opens_a_prefab_path_with_its_guid_in_front", CAMERA_FILES);

    // -- Exec
    let path = "{37578B1666981FCE}Prefabs/Characters/Core/Character_Base.et";
    let answer = read_in(settings(&game, None), json!({ "path": path })).await;

    // -- Check
    assert!(
        answer
            .text
            .starts_with("Prefabs/Characters/Core/Character_Base.et (game, 13 bytes)"),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn cuts_a_long_file_with_a_note_that_gives_its_full_length() {
    // -- Setup & Fixtures
    let game = out_folder("cuts_a_long_file_with_a_note_that_gives_its_full_length").join("game");
    let text = "a".repeat(150_000);
    let pak = PakBuilder::new()
        .dir("", 1)
        .compressed_file("Long.layout", text.as_bytes());
    write_built_pak(&game.join("addons/data/data.pak"), &pak);

    // -- Exec
    let answer = read_in(settings(&game, None), json!({ "path": "Long.layout" })).await;

    // -- Check
    // The first line and the empty line have 34 characters, the text 150,000.
    let end: String = answer.text.chars().rev().take(60).collect();
    assert!(
        answer
            .text
            .starts_with("Long.layout (game, 150000 bytes)\n\naaa"),
        "{}",
        answer.text.chars().take(40).collect::<String>()
    );
    assert!(
        answer
            .text
            .ends_with("a\n\n(Cut at 100,000 of 150,034 characters.)"),
        "{}",
        end.chars().rev().collect::<String>()
    );
}

#[tokio::test]
async fn says_that_a_binary_file_is_not_text() {
    // -- Setup & Fixtures
    let game = out_folder("says_that_a_binary_file_is_not_text").join("game");
    let pak = PakBuilder::new()
        .dir("", 1)
        .file("icon.edds", b"DDS \0\0\x01\x02");
    write_built_pak(&game.join("addons/data/data.pak"), &pak);

    // -- Exec
    let answer = read_in(settings(&game, None), json!({ "path": "icon.edds" })).await;

    // -- Check
    assert!(answer.is_error);
    assert!(
        answer
            .text
            .starts_with("icon.edds (game) is a binary file of 8 bytes, not text."),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn names_the_pak_file_when_a_file_does_not_unpack() {
    // -- Setup & Fixtures
    let game = out_folder("names_the_pak_file_when_a_file_does_not_unpack").join("game");
    let pak = PakBuilder::new()
        .dir("", 1)
        .packed_file("Broken.c", b"not zlib", 100);
    let pak_path = game.join("addons/data/data.pak");
    write_built_pak(&pak_path, &pak);

    // -- Exec
    let answer = read_in(settings(&game, None), json!({ "path": "Broken.c" })).await;

    // -- Check
    assert!(answer.is_error);
    assert!(
        answer.text.starts_with(&format!(
            "Could not open Broken.c (game) from {}: its compressed content does not unpack",
            pak_path.display()
        )),
        "{}",
        answer.text
    );
}

#[tokio::test]
async fn asks_for_a_path_when_the_path_is_empty() {
    // -- Setup & Fixtures
    let game = game_folder("asks_for_a_path_when_the_path_is_empty", CAMERA_FILES);

    // -- Exec
    let answer = read_in(settings(&game, None), json!({ "path": " / " })).await;

    // -- Check
    assert!(answer.is_error);
    assert!(
        answer.text.starts_with("The path is empty."),
        "{}",
        answer.text
    );
}

/// Needs the game and the downloaded `tvtLobby` mod: set `REFORGER_GAME_PATH` and
/// `REFORGER_WORKBENCH_PROFILE_PATH`, then run `cargo nextest run --run-ignored only`.
#[tokio::test]
#[ignore = "needs the installed game and the tvtLobby mod"]
async fn opens_a_game_script_and_a_mod_config_in_the_installed_game() {
    // -- Exec
    let script = read_in(
        Settings::from_env(),
        json!({ "path": "scripts/Game/GameMode/SCR_PreGameGameModeStateComponent.c" }),
    )
    .await;
    let config = read_in(
        Settings::from_env(),
        json!({ "path": "Configs/tvtLobby/tvtLobby_DefaultMission.conf" }),
    )
    .await;

    // -- Check
    assert!(!script.is_error, "{}", script.text);
    assert!(
        script
            .text
            .contains("class SCR_PreGameGameModeStateComponent"),
        "{}",
        script.text
    );
    assert!(!config.is_error, "{}", config.text);
    assert!(
        config
            .text
            .starts_with("Configs/tvtLobby/tvtLobby_DefaultMission.conf (mod tvtLobby, "),
        "{}",
        config.text
    );
}

/// Opens every 20th script of the installed game, so that a second unpack format cannot hide.
/// Needs the game: set `REFORGER_GAME_PATH`, then run `cargo nextest run --run-ignored only`.
#[test]
#[ignore = "needs the installed game"]
fn opens_a_sample_of_all_scripts_in_the_installed_game() {
    // -- Setup & Fixtures
    let settings = Settings {
        game: Settings::from_env().game,
        ..Settings::default()
    };
    let index = build(&settings, &CancellationToken::new()).unwrap();
    let scripts: Vec<_> = index
        .files
        .iter()
        .filter(|file| {
            Path::new(&file.path)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("c"))
        })
        .step_by(20)
        .collect();

    // -- Exec
    let failed: Vec<String> = scripts
        .iter()
        .filter_map(|file| file.read().err().map(|e| format!("{}: {e}", file.path)))
        .collect();

    // -- Check
    assert!(scripts.len() > 100, "only {} scripts", scripts.len());
    assert!(
        failed.is_empty(),
        "{} of {} failed: {failed:#?}",
        failed.len(),
        scripts.len()
    );
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

async fn read_in(settings: Settings, arguments: serde_json::Value) -> Answer {
    let tools: ToolRouter<Server> = router(settings);
    let client = test_support::connect(tools).await;
    test_support::call(&client, "game_read", arguments).await
}

async fn find_in(settings: Settings, arguments: serde_json::Value) -> Answer {
    let tools: ToolRouter<Server> = router(settings);
    let client = test_support::connect(tools).await;
    test_support::call(&client, "game_find", arguments).await
}

fn settings(game: &Path, profile: Option<&Path>) -> Settings {
    Settings {
        game: Some(game.display().to_string()),
        profile: profile.map(|p| p.display().to_string()),
        documents: None,
    }
}

/// A game folder with `files` in `addons/data/data.pak`.
fn game_folder(test: &str, files: &[&str]) -> PathBuf {
    let game = out_folder(test).join("game");
    write_pak(&game.join("addons/data/data.pak"), files);
    game
}

fn write_pak(pak: &Path, files: &[&str]) {
    write_built_pak(pak, &PakBuilder::with_files(files));
}

fn write_built_pak(pak: &Path, builder: &PakBuilder) {
    fs::create_dir_all(pak.parent().unwrap()).unwrap();
    fs::write(pak, builder.build()).unwrap();
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
