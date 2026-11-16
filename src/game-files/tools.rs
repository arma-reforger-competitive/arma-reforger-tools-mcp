//! The Tools of the game-files Capability.

use std::sync::{Arc, Mutex, PoisonError};

use rmcp::handler::server::{
    common::schema_for_type, router::tool::ToolRouter, wrapper::Parameters,
};
use rmcp::model::{CallToolResult, ContentBlock, Tool};
use tokio_util::sync::CancellationToken;

use super::find::{FindInput, find};
use super::index::{Index, build};
use super::settings::Settings;
use super::{Error, Result};

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
    );
    ToolRouter::new().with_route((
        game_find,
        move |Parameters(input): Parameters<FindInput>, cancel: CancellationToken| {
            let files = Arc::clone(&files);
            async move {
                match files.index(cancel).await {
                    Ok(index) => find(&index, &input),
                    Err(error) => {
                        CallToolResult::error(vec![ContentBlock::text(error.to_string())])
                    }
                }
            }
        },
    ))
}

// region:    --- Support

/// The settings and the index, which the first call builds.
struct Files {
    settings: Settings,
    index: Mutex<Option<Arc<Index>>>,
}

impl Files {
    /// The index of every Game file. The first call builds it outside the server thread. A
    /// failed build is not kept, so the next call tries again.
    async fn index(&self, cancel: CancellationToken) -> Result<Arc<Index>> {
        if let Some(index) = self.cached() {
            return Ok(index);
        }
        let settings = self.settings.clone();
        let index = match tokio::task::spawn_blocking(move || build(&settings, &cancel)).await {
            Ok(index) => Arc::new(index?),
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
