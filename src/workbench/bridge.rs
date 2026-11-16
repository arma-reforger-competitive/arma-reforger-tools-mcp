//! The Workbench bridge: calls the handler script in a running Workbench. When the handler
//! script is missing or old, it copies the new one into the open project and asks Workbench to
//! compile the scripts again. It never starts Workbench, never polls, and never changes a
//! `.gproj` file (ADR 0009).

use std::path::PathBuf;

use serde_json::{Map, Value};

use super::error::Garbled;
use super::install::{HANDLER_FUNC, install};
use super::net::{self, Answer};
use super::project::{Project, open_project};
use super::settings::{Address, Settings};
use super::{Error, Result};

/// The built-in NET API function that gives the IDs of the loaded projects.
const LOADED_PROJECTS_FUNC: &str = "GetLoadedProjects";

/// The built-in NET API function that compiles the scripts again. It answers when the compile
/// ends.
const RELOAD_FUNC: &str = "ReloadScripts";

/// What `ReloadScripts` answers when a script of the open project has an error.
const COMPILE_FAILED: &str = "Compilation failed";

/// The connection to the handler script in Workbench. Each call uses a new TCP connection.
#[derive(Clone)]
pub struct Bridge {
    settings: Settings,
}

/// What the bridge did to the handler script before the call.
#[derive(Debug, PartialEq, Eq)]
pub enum Setup {
    /// The handler script was loaded already.
    Loaded,

    /// The handler script was not loaded. The bridge copied it to `file`.
    Installed { file: PathBuf },

    /// Workbench had the version `old`. The bridge copied the new one to `file`.
    Updated { file: PathBuf, old: String },
}

impl Bridge {
    pub(super) fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Calls `action` on the handler script with `params`, and returns its answer and what the
    /// bridge did to the handler script first. When the handler script is missing or old, the
    /// bridge copies the new one into the open project, reloads the scripts, and calls again.
    pub async fn call(
        &self,
        action: &str,
        mut params: Map<String, Value>,
    ) -> Result<(Map<String, Value>, Setup)> {
        let address = self.settings.address()?;
        params.insert("action".to_owned(), Value::from(action));
        let (file, setup) = match self.call_handler(&address, params.clone()).await? {
            Handler::Current(answer) => return Ok((answer?, Setup::Loaded)),
            Handler::Missing => {
                let file = self.install_and_reload(&address).await?;
                (file.clone(), Setup::Installed { file })
            }
            Handler::Old(old) => {
                let file = self.install_and_reload(&address).await?;
                (file.clone(), Setup::Updated { file, old })
            }
        };
        match self.call_handler(&address, params).await? {
            Handler::Current(answer) => Ok((answer?, setup)),
            Handler::Missing | Handler::Old(_) => Err(Error::HandlerNotLoaded { file }),
        }
    }

    /// The mod project that is open in Workbench.
    pub(super) async fn open_project(&self) -> Result<Project> {
        self.open_project_at(&self.settings.address()?).await
    }

    // region:    --- Support

    /// Sends `params` to the handler script.
    async fn call_handler(&self, address: &Address, params: Map<String, Value>) -> Result<Handler> {
        let wait = self.settings.answer_timeout;
        let answer = match net::call(&self.settings, address, HANDLER_FUNC, params, wait).await? {
            Answer::Ok(answer) => answer,
            Answer::UndefinedFunc => return Ok(Handler::Missing),
        };
        let version = answer
            .get("version")
            .and_then(Value::as_str)
            .ok_or(Garbled::NoField { field: "version" })?;
        if version != env!("CARGO_PKG_VERSION") {
            return Ok(Handler::Old(version.to_owned()));
        }
        Ok(Handler::Current(
            match answer.get("error").and_then(Value::as_str) {
                Some(message) if !message.is_empty() => Err(Error::HandlerFailed {
                    message: message.to_owned(),
                }),
                _ => Ok(answer),
            },
        ))
    }

    /// Copies the handler script into the open project and compiles the scripts again. Returns
    /// the path of the copied file.
    async fn install_and_reload(&self, address: &Address) -> Result<PathBuf> {
        let file = install(&self.open_project_at(address).await?.folder)?;
        let wait = self.settings.reload_timeout;
        match net::call(&self.settings, address, RELOAD_FUNC, Map::new(), wait).await {
            Ok(Answer::Ok(_)) => Ok(file),
            Ok(Answer::UndefinedFunc) => Err(Error::HandlerNotLoaded { file }),
            Err(Error::WorkbenchFailed { status, .. }) if status == COMPILE_FAILED => {
                Err(Error::CompileFailed { file })
            }
            Err(error) => Err(error),
        }
    }

    async fn open_project_at(&self, address: &Address) -> Result<Project> {
        let wait = self.settings.answer_timeout;
        let answer = match net::call(
            &self.settings,
            address,
            LOADED_PROJECTS_FUNC,
            Map::new(),
            wait,
        )
        .await?
        {
            Answer::Ok(answer) => answer,
            Answer::UndefinedFunc => {
                return Err(Error::WorkbenchFailed {
                    func: LOADED_PROJECTS_FUNC,
                    status: "Undefined API func".to_owned(),
                });
            }
        };
        let loaded: Vec<String> = answer
            .get("Loaded Projects")
            .and_then(Value::as_array)
            .ok_or(Garbled::NoField {
                field: "Loaded Projects",
            })?
            .iter()
            .filter_map(|id| id.as_str().map(str::to_owned))
            .collect();
        open_project(&self.settings.documents()?, &loaded)
    }

    // endregion: --- Support
}

/// What the handler script answered.
enum Handler {
    /// The current version answered: its answer, or the error it gave.
    Current(Result<Map<String, Value>>),
    Missing,
    Old(String),
}
