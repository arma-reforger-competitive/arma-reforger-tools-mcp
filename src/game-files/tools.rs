//! The Tools of the game-files Capability.

use std::sync::{Arc, Mutex, PoisonError};

use rmcp::handler::server::{
    common::schema_for_type, router::tool::ToolRouter, wrapper::Parameters,
};
use rmcp::model::Tool;
use tokio_util::sync::CancellationToken;

use super::find::{FindInput, find};
use super::index::{Index, ModsOnDisk, build};
use super::read::{ReadInput, read};
use super::settings::Settings;
use super::{Error, Result};
use crate::server::{Answer, answer, read_only_labels};

/// Gives the game-files Tools to the server, with the settings from the environment.
pub fn tools<S: Send + Sync + 'static>() -> ToolRouter<S> {
    router(Settings::from_env())
}

/// Gives the game-files Tools with `settings`.
pub(super) fn router<S: Send + Sync + 'static>(settings: Settings) -> ToolRouter<S> {
    let files = Arc::new(Files {
        settings,
        index: Mutex::new(None),
    });
    let read_files = Arc::clone(&files);
    let game_find = Tool::new(
        "game_find",
        "Find Game files, the files inside the .pak archives of the installed game and of the \
         Workshop mods that Workbench downloaded, by a part of their name or path, or by a class \
         name, for example \"CharacterCameraHandler\" or \"SCR_PreGameGameModeStateComponent\". \
         Returns the real paths, best match first: the full file name, then names that start \
         with the query, then names and paths that have it inside. Each path says if it comes \
         from the game or from which mod. If no file matches, the answer says how many files it \
         searched. The first call reads the file lists, which takes about a second.",
        schema_for_type::<FindInput>(),
    )
    .with_title("Find Game files")
    .annotate(read_only_labels());
    let game_read = Tool::new(
        "game_read",
        "Open one Game file, a file inside the .pak archives of the installed game or of a \
         Workshop mod that Workbench downloaded, by its path, and return its full text: a script \
         (.c), a Prefab (.et), a config (.conf), or a layout (.layout). Capital letters in the \
         path do not matter. If no file has the path, the answer lists the closest real paths, \
         so a full path can be opened directly. To find a path by a part of its name, call \
         game_find first.",
        schema_for_type::<ReadInput>(),
    )
    .with_title("Open a Game file")
    .annotate(read_only_labels());
    ToolRouter::new()
        .with_route((
            game_read,
            move |Parameters(input): Parameters<ReadInput>, cancel: CancellationToken| {
                let files = Arc::clone(&read_files);
                async move { answer(read_answer(&files, input, cancel).await) }
            },
        ))
        .with_route((
            game_find,
            move |Parameters(input): Parameters<FindInput>, cancel: CancellationToken| {
                let files = Arc::clone(&files);
                async move {
                    let index = files.index(cancel).await;
                    answer(
                        index
                            .map_err(|error| error.to_string())
                            .and_then(|index| find(&index, &input)),
                    )
                }
            },
        ))
}

// region:    --- Support

/// Answers one `game_read` call. It reads the .pak file outside the server thread.
async fn read_answer(files: &Files, input: ReadInput, cancel: CancellationToken) -> Answer {
    let index = files
        .index(cancel)
        .await
        .map_err(|error| error.to_string())?;
    let read = tokio::task::spawn_blocking(move || read(&index, &input)).await;
    read.map_err(|_| Error::ReaderStopped.to_string())?
}

/// The settings and the index, which the first call builds.
struct Files {
    settings: Settings,
    index: Mutex<Option<Arc<Index>>>,
}

impl Files {
    /// The index of every Game file. The first call builds it outside the server thread. Each
    /// later call builds it again when the mod folders or their .pak files on disk are not the
    /// ones it was built from, so a mod that Workbench downloads shows with no restart. A failed
    /// build is not kept, so the next call tries again.
    async fn index(&self, cancel: CancellationToken) -> Result<Arc<Index>> {
        let settings = self.settings.clone();
        let cached = self.cached();
        let index = tokio::task::spawn_blocking(move || match cached {
            Some(index) if index.mods == ModsOnDisk::list(&settings) => Ok(index),
            _ => build(&settings, &cancel).map(Arc::new),
        });
        let index = match index.await {
            Ok(index) => index?,
            Err(join_error) if join_error.is_panic() => return Err(Error::ReaderStopped),
            Err(_) => return Err(Error::Cancelled),
        };
        *self.index.lock().unwrap_or_else(PoisonError::into_inner) = Some(Arc::clone(&index));
        Ok(index)
    }

    fn cached(&self) -> Option<Arc<Index>> {
        self.index
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tests;

// endregion: --- Tests
