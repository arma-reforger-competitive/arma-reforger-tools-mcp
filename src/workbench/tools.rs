//! The Tools of the Workbench Capability.

use std::sync::Arc;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::model::{CallToolResult, ContentBlock, Tool, ToolAnnotations};
use serde_json::Map;

use super::Error;
use super::bridge::{Bridge, Setup};
use super::install::HANDLER_FOLDER;
use super::settings::Settings;

/// Gives the Workbench Tools to the server, with the settings from the environment.
pub fn tools<S: Send + Sync + 'static>() -> ToolRouter<S> {
    router(Settings::from_env())
}

/// Gives the Workbench Tools with `settings`.
pub(super) fn router<S: Send + Sync + 'static>(settings: Settings) -> ToolRouter<S> {
    let bridge = Arc::new(Bridge::new(settings));
    let workbench_status = Tool::new(
        "workbench_status",
        "Check the connection to Arma Reforger Workbench, the editor for mods, and get it ready \
         for the Workbench Tools. When Workbench runs, it returns the version of the Handler \
         script of this plugin and the open mod project. When the Handler script is missing or \
         old, it first copies the new one into the open mod and reloads the scripts, and says \
         so. When Workbench is not running, it says how to start it and which settings to \
         check. It never starts or stops Workbench, and it writes only its own file \
         Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c in the open mod.",
        Arc::new(no_inputs()),
    )
    .with_title("Check the Workbench connection")
    .annotate(labels());
    ToolRouter::new().with_route((workbench_status, move || {
        let bridge = Arc::clone(&bridge);
        async move { status(&bridge).await }
    }))
}

// region:    --- Support

/// The input schema of a Tool with no inputs. MCP needs `"type": "object"`, and a client drops a
/// Tool whose schema does not have it.
fn no_inputs() -> Map<String, serde_json::Value> {
    let mut schema = Map::new();
    schema.insert("type".to_owned(), "object".into());
    schema
}

/// The Tool labels of `workbench_status`: it copies and overwrites its Handler script, a second
/// call has no more effect, and it does not use the internet.
fn labels() -> ToolAnnotations {
    ToolAnnotations::new()
        .read_only(false)
        .destructive(true)
        .idempotent(true)
        .open_world(false)
}

/// The answer of `workbench_status`. "Not running" and the three ways to a loaded Handler script
/// are the states that this Tool reports, so they are normal answers, also when `NotRunning` is an
/// error for the other Workbench Tools. Other failures are errors.
async fn status(bridge: &Bridge) -> CallToolResult {
    let setup = match bridge.call("ping", Map::new()).await {
        Ok((_, setup)) => setup,
        Err(error @ Error::NotRunning { .. }) => {
            return CallToolResult::success(vec![ContentBlock::text(error.to_string())]);
        }
        Err(error) => return CallToolResult::error(vec![ContentBlock::text(error.to_string())]),
    };
    // The bridge answers only when the Handler script has the version of this server.
    let version = env!("CARGO_PKG_VERSION");
    let loaded = match setup {
        Setup::Loaded => format!("the Handler script is loaded (version {version})"),
        Setup::Installed { file } => format!(
            "the Handler script of this plugin was not loaded. The bridge copied it to {} and \
             reloaded the scripts, and now it is loaded (version {version})",
            file.display()
        ),
        Setup::Updated { file, old } => format!(
            "it had an old Handler script (version {old}). The bridge copied the new one to {} \
             and reloaded the scripts, and now it is loaded (version {version})",
            file.display()
        ),
    };
    let project = match bridge.open_project().await {
        Ok(project) => format!(
            "The open mod project is {} in {}.",
            project.id,
            project.folder.display()
        ),
        Err(error) => format!("The open mod project is not known: {error}"),
    };
    CallToolResult::success(vec![ContentBlock::text(format!(
        "Workbench is running, and {loaded}. {project} The Workbench Tools can use it now. Before \
         you pack or publish the mod, delete its folder {HANDLER_FOLDER}: Workbench packs the \
         Handler script with the mod."
    ))])
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tests;

// endregion: --- Tests
