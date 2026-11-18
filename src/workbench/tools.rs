//! The Tools of the Workbench Capability.

use std::sync::Arc;

use rmcp::handler::server::{
    common::schema_for_type, router::tool::ToolRouter, wrapper::Parameters,
};
use rmcp::model::{CallToolResult, ContentBlock, Tool, ToolAnnotations};
use schemars::JsonSchema;
use serde::Deserialize;
use serde_json::Map;

use super::Error;
use super::bridge::{Bridge, Setup};
use super::install::HANDLER_FOLDER;
use super::launch::{Launched, launch};
use super::settings::Settings;

/// The input of `workbench_launch`.
#[derive(Deserialize, JsonSchema)]
pub(super) struct LaunchInput {
    /// The mod to open: its name in the project list of Workbench, for example `TrainingRange`.
    /// Leave it out when the user has one mod, or to open the Workbench launcher when the user
    /// has none.
    #[serde(rename = "mod")]
    #[schemars(rename = "mod")]
    wanted: Option<String>,
}

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
         so. It never starts or stops Workbench: to start Workbench or open a mod in it, use \
         workbench_launch. It writes only its own file \
         Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c in the open mod.",
        Arc::new(no_inputs()),
    )
    .with_title("Check the Workbench connection")
    .annotate(labels());
    let workbench_launch = Tool::new(
        "workbench_launch",
        "Start or open Arma Reforger Workbench, the editor for mods, with one mod of the user \
         open, and wait until it has compiled the scripts (up to 3 minutes). It finds \
         Workbench through Steam. When Arma Reforger Tools or the game is not installed, it says how to install \
         it. When Workbench already runs, it starts nothing and gives the open mod. Before the \
         start, it copies the Handler script of this plugin into the mod, like \
         workbench_status. Without a mod and with no mods in the project list of Workbench, it \
         opens the Workbench launcher. An unknown mod name gives the names of the user's mods.",
        schema_for_type::<LaunchInput>(),
    )
    .with_title("Start Workbench with a mod")
    .annotate(labels());
    let launch_bridge = Arc::clone(&bridge);
    ToolRouter::new()
        .with_route((workbench_status, move || {
            let bridge = Arc::clone(&bridge);
            async move { answer(report(&bridge).await) }
        }))
        .with_route((
            workbench_launch,
            move |Parameters(input): Parameters<LaunchInput>| {
                let bridge = Arc::clone(&launch_bridge);
                async move { launch_answer(&bridge, input.wanted.as_deref()).await }
            },
        ))
}

// region:    --- Support

/// The input schema of a Tool with no inputs. MCP needs `"type": "object"`, and a client drops a
/// Tool whose schema does not have it.
fn no_inputs() -> Map<String, serde_json::Value> {
    let mut schema = Map::new();
    schema.insert("type".to_owned(), "object".into());
    schema
}

/// The Tool labels of the Workbench Tools: they copy and overwrite the Handler script, a second
/// call has no more effect, and they do not use the internet.
fn labels() -> ToolAnnotations {
    ToolAnnotations::new()
        .read_only(false)
        .destructive(true)
        .idempotent(true)
        .open_world(false)
}

/// A Tool answer: its text, and whether it is an error.
struct Report {
    text: String,
    is_error: bool,
}

/// The answer of `workbench_status`. "Not running" and the three ways to a loaded Handler script
/// are the states that this Tool reports, so they are normal answers, also when `NotRunning` is an
/// error for the other Workbench Tools. Other failures are errors.
async fn report(bridge: &Bridge) -> Report {
    let setup = match bridge.call("ping", Map::new()).await {
        Ok((_, setup)) => setup,
        Err(error @ Error::NotRunning { .. }) => return Report::normal(error.to_string()),
        Err(error) => return Report::error(&error),
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
    Report::normal(format!(
        "Workbench is running, and {loaded}. {project} The Workbench Tools can use it now. Before \
         you pack or publish the mod, delete its folder {HANDLER_FOLDER}: Workbench packs the \
         Handler script with the mod."
    ))
}

/// The answer of `workbench_launch`: what it started, then the state of Workbench.
async fn launch_answer(bridge: &Bridge, wanted: Option<&str>) -> CallToolResult {
    let started = match launch(bridge, wanted).await {
        Ok(Launched::AlreadyRunning) => {
            "Workbench was running already, so workbench_launch started nothing.".to_owned()
        }
        Ok(Launched::Started { project, steam }) => format!(
            "{}Workbench started with the mod {} and compiled the scripts.",
            steam_started(steam),
            project.id
        ),
        Ok(Launched::StartedLauncher { steam }) => {
            return answer(Report::normal(format!(
                "{}Workbench started on its launcher, because the project list of Workbench has \
                 no mod. Ask the user to create a mod there (Create New) or to add one (Add \
                 Existing). Then call workbench_status.",
                steam_started(steam)
            )));
        }
        Err(error) => return answer(Report::error(&error)),
    };
    let report = report(bridge).await;
    answer(Report {
        text: format!("{started} {}", report.text),
        is_error: report.is_error,
    })
}

/// The start of the answer when `workbench_launch` started Steam.
fn steam_started(steam: bool) -> &'static str {
    if steam {
        "No user was logged in to Steam, so workbench_launch started Steam. "
    } else {
        ""
    }
}

impl Report {
    fn normal(text: String) -> Self {
        Self {
            text,
            is_error: false,
        }
    }

    fn error(error: &Error) -> Self {
        Self {
            text: error.to_string(),
            is_error: true,
        }
    }
}

fn answer(report: Report) -> CallToolResult {
    let content = vec![ContentBlock::text(report.text)];
    if report.is_error {
        CallToolResult::error(content)
    } else {
        CallToolResult::success(content)
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
#[path = "tools_tests.rs"]
mod tests;

// endregion: --- Tests
