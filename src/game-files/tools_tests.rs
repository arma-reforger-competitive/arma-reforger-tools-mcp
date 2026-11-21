use std::fs;
use std::path::{Path, PathBuf};

use rmcp::handler::server::router::tool::ToolRouter;

use super::*;
use crate::game_files::test_pak::PakBuilder;
use crate::server::Server;
use crate::test_support::{self, Answer};

#[path = "tools_tests_find.rs"]
mod find;

#[path = "tools_tests_read.rs"]
mod read;

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
