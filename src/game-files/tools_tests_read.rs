//! The tests of `game_read`.

use std::path::Path;

use serde_json::json;

use super::*;
use crate::game_files::index::build;
use crate::game_files::test_pak::PakBuilder;

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
