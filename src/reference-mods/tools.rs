//! The Tool of the reference-mods Capability.

use std::sync::Arc;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{Tool, ToolAnnotations};
use tokio_util::sync::CancellationToken;

use super::sync::{Settings, Wait, sync};
use crate::server::{answer, no_inputs};
use crate::workbench::Bridge;

/// Gives the reference-mods Tool to the server, with the settings from the environment.
pub fn tools<S: Send + Sync + 'static>() -> ToolRouter<S> {
    router_with(Settings::from_env(), Bridge::from_env(), Wait::workshop())
}

/// Gives the reference-mods Tool with `settings`. It calls Workbench through `bridge` and waits
/// for the downloads as `wait` says.
fn router_with<S: Send + Sync + 'static>(
    settings: Settings,
    bridge: Bridge,
    wait: Wait,
) -> ToolRouter<S> {
    let state = Arc::new((settings, bridge, wait));
    let reference_sync = Tool::new(
        "reference_sync",
        "Make the Workshop mods in the Workbench addons folder match the Reference mods: the \
         mods that the user lists in reference-mods.jsonl in the mod repos, for the agent to \
         read and learn how a feature works. It downloads each listed mod that is not on disk, \
         through Workbench, and deletes each mod that it downloaded before and that no list has \
         now. It never touches anything else: the user's projects, links, saves, and mods that \
         the user downloaded by hand stay. It answers one line for each mod: name, ID, license, \
         and state (present, downloaded, downloading, deleted, or failed with the reason). A \
         list line that is not valid stops it before any delete, and the answer names the file \
         and the line. License rule: read any Reference mod; copy code only when its license \
         allows it; otherwise write the feature again in your own way.",
        no_inputs(),
    )
    .with_title("Sync the Reference mods")
    .annotate(labels());
    ToolRouter::new().with_route((reference_sync, move |cancel: CancellationToken| {
        let state = Arc::clone(&state);
        async move {
            let (settings, bridge, wait) = state.as_ref();
            answer(sync(settings, bridge, wait, cancel).await)
        }
    }))
}

// region:    --- Support

/// It deletes mod folders, a second call has no more effect, and it downloads from the Workshop.
fn labels() -> ToolAnnotations {
    ToolAnnotations::new()
        .read_only(false)
        .destructive(true)
        .idempotent(true)
        .open_world(true)
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tests;

// endregion: --- Tests
