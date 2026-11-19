//! The Workbench bridge: calls the Handler script in a running Workbench. When the Handler
//! script is missing or old, it copies the new one into the open project and asks Workbench to
//! compile the scripts again. It never starts Workbench, never polls, and never changes a
//! `.gproj` file (ADR 0009).

use std::path::PathBuf;
use std::time::Duration;

use serde_json::{Map, Value};

use super::error::Garbled;
use super::install::{HANDLER_FUNC, install};
use super::net::{self, Answer, Wait};
use super::project::{Project, open_project};
use super::settings::Settings;
use super::{Error, Result};

/// The built-in NET API function that gives the IDs of the loaded projects.
const LOADED_PROJECTS_FUNC: &str = "GetLoadedProjects";

/// The built-in NET API function that compiles the scripts again. It answers when the compile
/// ends.
const RELOAD_FUNC: &str = "ReloadScripts";

/// The built-in NET API function that says whether Workbench runs and has compiled its scripts.
const RUNNING_FUNC: &str = "IsWorkbenchRunning";

/// What `ReloadScripts` answers when a script of the open project has an error.
const COMPILE_FAILED: &str = "Compilation failed";

/// The connection to the Handler script in Workbench. Each call uses a new TCP connection.
pub struct Bridge {
    settings: Settings,
}

/// What the bridge did to the Handler script before the call.
#[derive(Debug, PartialEq, Eq)]
pub enum Setup {
    /// The Handler script was loaded already.
    Loaded,

    /// The Handler script was not loaded. The bridge copied it to `file`.
    Installed { file: PathBuf },

    /// Workbench had the version `old`. The bridge copied the new one to `file`.
    Updated { file: PathBuf, old: String },
}

impl Bridge {
    /// The bridge with the settings from the environment of the server. Every Capability that
    /// calls Workbench makes its bridge here.
    pub fn from_env() -> Self {
        Self::new(Settings::from_env())
    }

    pub(super) fn new(settings: Settings) -> Self {
        Self { settings }
    }

    /// Calls `action` on the Handler script with `params`, and returns its answer and what the
    /// bridge did to the Handler script first. When the Handler script is missing or old, the
    /// bridge copies the new one into the open project, reloads the scripts, and calls again.
    pub async fn call(
        &self,
        action: &str,
        mut params: Map<String, Value>,
    ) -> Result<(Map<String, Value>, Setup)> {
        params.insert("action".to_owned(), Value::from(action));
        let old = match self.call_handler_script(params.clone()).await? {
            HandlerScript::Current(answer) => return Ok((answer?, Setup::Loaded)),
            HandlerScript::Missing => None,
            HandlerScript::Old(old) => Some(old),
        };
        let file = self.install_and_reload().await?;
        let answer = match self.call_handler_script(params).await? {
            HandlerScript::Current(answer) => answer?,
            HandlerScript::Missing | HandlerScript::Old(_) => {
                return Err(Error::HandlerNotLoaded { file });
            }
        };
        let setup = match old {
            None => Setup::Installed { file },
            Some(old) => Setup::Updated { file, old },
        };
        Ok((answer, setup))
    }

    /// The settings of the bridge.
    pub(super) fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Whether Workbench has compiled its scripts. `Error::NotRunning` when nothing answers.
    pub(super) async fn scripts_compiled(&self) -> Result<bool> {
        match self
            .call_func(RUNNING_FUNC, self.settings.answer_timeout)
            .await?
        {
            Answer::Ok(answer) => Ok(answer
                .get("ScriptsCompiled")
                .and_then(Value::as_bool)
                .ok_or(Garbled::NoField {
                    field: "ScriptsCompiled",
                })?),
            Answer::UndefinedFunc => Err(Error::WorkbenchFailed {
                func: RUNNING_FUNC,
                status: "Undefined API func".to_owned(),
            }),
        }
    }

    /// The mod project that is open in Workbench.
    pub(super) async fn open_project(&self) -> Result<Project> {
        let loaded = self.loaded_projects().await?;
        open_project(&self.settings.documents()?, &loaded)
    }

    /// The IDs of the projects that Workbench has loaded: the game, the open mod, and its
    /// dependencies.
    pub(super) async fn loaded_projects(&self) -> Result<Vec<String>> {
        let answer = match self
            .call_func(LOADED_PROJECTS_FUNC, self.settings.answer_timeout)
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
        Ok(answer
            .get("Loaded Projects")
            .and_then(Value::as_array)
            .ok_or(Garbled::NoField {
                field: "Loaded Projects",
            })?
            .iter()
            .filter_map(|id| id.as_str().map(str::to_owned))
            .collect())
    }
}

// region:    --- Support

/// What the Handler script answered.
enum HandlerScript {
    /// The current version answered: its answer, or the error it gave.
    Current(Result<Map<String, Value>>),
    Missing,
    Old(String),
}

impl Bridge {
    /// Sends `params` to the Handler script.
    async fn call_handler_script(&self, params: Map<String, Value>) -> Result<HandlerScript> {
        let wait = self.wait(self.settings.answer_timeout);
        let answer = match net::call(self.settings.address()?, HANDLER_FUNC, params, wait).await? {
            Answer::Ok(answer) => answer,
            Answer::UndefinedFunc => return Ok(HandlerScript::Missing),
        };
        let version = answer
            .get("version")
            .and_then(Value::as_str)
            .ok_or(Garbled::NoField { field: "version" })?;
        if version != env!("CARGO_PKG_VERSION") {
            return Ok(HandlerScript::Old(version.to_owned()));
        }
        Ok(HandlerScript::Current(
            match answer.get("error").and_then(Value::as_str) {
                Some(message) if !message.is_empty() => Err(Error::HandlerFailed {
                    message: message.to_owned(),
                }),
                _ => Ok(answer),
            },
        ))
    }

    /// Copies the Handler script into the open project and compiles the scripts again. Returns
    /// the path of the copied file.
    async fn install_and_reload(&self) -> Result<PathBuf> {
        let file = install(&self.open_project().await?.folder)?;
        match self
            .call_func(RELOAD_FUNC, self.settings.reload_timeout)
            .await
        {
            Ok(Answer::Ok(_)) => Ok(file),
            Ok(Answer::UndefinedFunc) => Err(Error::HandlerNotLoaded { file }),
            Err(Error::WorkbenchFailed { status, .. }) if status == COMPILE_FAILED => {
                Err(Error::CompileFailed { file })
            }
            Err(error) => Err(error),
        }
    }

    /// Calls the built-in NET API function `func` with no parameters, and waits up to `answer`
    /// for its answer.
    async fn call_func(&self, func: &'static str, answer: Duration) -> Result<Answer> {
        net::call(
            self.settings.address()?,
            func,
            Map::new(),
            self.wait(answer),
        )
        .await
    }

    fn wait(&self, answer: Duration) -> Wait {
        Wait {
            connect: self.settings.connect_timeout,
            answer,
        }
    }
}

// endregion: --- Support
