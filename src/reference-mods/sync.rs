//! The outer layer of `reference_sync`: it reads the lists and the record, lists the `addons`
//! folder, deletes folders, writes the record, and calls Workbench. The decisions are in
//! `plan.rs`.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use serde_json::Map;

use super::error::{BadLine, Error, Result};
use super::plan::{
    Line, Listed, Planned, Recorded, State, answer, parse_line, parse_record, plan, record_text,
    recorded_name, unknown_license,
};
use crate::host_path::host_path;
use crate::workbench::Bridge;
use crate::workbench_documents::workbench_documents;

/// The setting of the folder of the mod repos.
pub(super) const MODS_SETTING: &str = "REFORGER_MODS_PATH";

/// The list of Reference mods in the root of a mod repo.
pub(super) const LIST_FILE: &str = "reference-mods.jsonl";

/// The record of the mods that the Tool downloaded, in the `addons` folder. The dot keeps it from
/// looking like a mod folder.
const RECORD_FILE: &str = ".reference-mods.jsonl";

/// The raw values of the settings, as the user wrote them. An empty value counts as not set.
pub(super) struct Settings {
    pub(super) mods: Option<String>,
    pub(super) documents: Option<String>,
}

impl Settings {
    /// Reads the settings from the environment of the server.
    pub(super) fn from_env() -> Self {
        let read = |name| std::env::var(name).ok().filter(|v| !v.trim().is_empty());
        Self {
            mods: read(MODS_SETTING),
            documents: read(crate::workbench_documents::DOCUMENTS_SETTING),
        }
    }
}

/// Makes the `addons` folder match the lists, and returns the text of the answer.
pub(super) async fn sync(settings: &Settings, bridge: &Bridge) -> Result<String> {
    let mods = settings.mods.clone();
    let documents = settings.documents.clone();
    let (reference, deleted) =
        tokio::task::spawn_blocking(move || clean(mods.as_deref(), documents.as_deref()))
            .await
            .map_err(|_| Error::Stopped)??;

    let mut lines = Vec::new();
    let mut failure = None;
    for planned in reference {
        let state = if let Some(license) = planned.present {
            line(&planned.listed, license, State::Present)
        } else {
            let reason = match &failure {
                Some(reason) => reason,
                None => failure.insert(download_failure(bridge).await),
            };
            line(
                &planned.listed,
                unknown_license(),
                State::Failed {
                    reason: reason.clone(),
                },
            )
        };
        lines.push(state);
    }
    lines.extend(deleted);
    Ok(answer(&lines))
}

// region:    --- Support

/// Reads the settings, the lists, and the record, makes the plan, deletes, and writes the record.
/// Returns the Reference mods of the plan and the lines of the deleted mods.
fn clean(mods: Option<&str>, documents: Option<&str>) -> Result<(Vec<Planned>, Vec<Line>)> {
    let mods = mods.ok_or(Error::ModsNotSet).map(host_path)?;
    if !mods.is_dir() {
        return Err(Error::ModsNotFolder { folder: mods });
    }
    let addons = workbench_documents(documents)
        .ok_or(Error::DocumentsNotFound)?
        .join("addons");
    let lists = read_lists(&mods)?;
    let record_file = addons.join(RECORD_FILE);
    let mut record = read_record(&record_file)?;
    let folders = folder_names(&addons)?;

    let plan = plan(&lists, &record, &folders);

    let mut deleted = Vec::new();
    for entry in &plan.delete {
        let state = delete(&addons.join(&entry.folder));
        if matches!(state, State::Deleted) {
            record.retain(|kept| kept != entry);
        }
        deleted.push(Line {
            name: recorded_name(entry).to_owned(),
            id: entry.id.to_ascii_uppercase(),
            license: entry.license.clone(),
            state,
        });
    }
    if deleted
        .iter()
        .any(|line| matches!(line.state, State::Deleted))
    {
        write_record(&record_file, &record)?;
    }
    Ok((plan.reference, deleted))
}

/// The Reference mods of the lists in each direct subfolder of `mods`, in folder order.
fn read_lists(mods: &Path) -> Result<Vec<Listed>> {
    let mut repos: Vec<PathBuf> = fs::read_dir(mods)
        .map_err(|err| Error::ReadFailed {
            path: mods.to_owned(),
            err,
        })?
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| path.is_dir())
        .collect();
    repos.sort();
    let mut lists = Vec::new();
    for repo in repos {
        let file = repo.join(LIST_FILE);
        let text = match fs::read_to_string(&file) {
            Ok(text) => text,
            Err(err) if err.kind() == ErrorKind::NotFound => continue,
            Err(err) => return Err(Error::ReadFailed { path: file, err }),
        };
        for (index, line) in text.lines().enumerate() {
            let listed = parse_line(line).map_err(|reason| Error::BadListLine {
                file: file.clone(),
                line: index + 1,
                reason,
            })?;
            lists.extend(listed);
        }
    }
    Ok(lists)
}

/// The record in `file`. No file: an empty record.
fn read_record(file: &Path) -> Result<Vec<Recorded>> {
    let text = match fs::read_to_string(file) {
        Ok(text) => text,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(Error::ReadFailed {
                path: file.to_owned(),
                err,
            });
        }
    };
    parse_record(&text).map_err(|(line, reason): (usize, BadLine)| Error::BadRecordLine {
        file: file.to_owned(),
        line,
        reason,
    })
}

/// The names of the folders in `addons`. No `addons` folder: no names.
fn folder_names(addons: &Path) -> Result<Vec<String>> {
    let entries = match fs::read_dir(addons) {
        Ok(entries) => entries,
        Err(err) if err.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(err) => {
            return Err(Error::ReadFailed {
                path: addons.to_owned(),
                err,
            });
        }
    };
    Ok(entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry
                .path()
                .is_dir()
                .then(|| entry.file_name().to_string_lossy().into_owned())
        })
        .collect())
}

/// Deletes the mod folder `folder`. A folder that is gone counts as deleted. A link or a file in
/// its place stays.
fn delete(folder: &Path) -> State {
    match fs::symlink_metadata(folder) {
        Err(err) if err.kind() == ErrorKind::NotFound => State::Deleted,
        Err(err) => State::Failed {
            reason: format!("could not read {}: {err}", folder.display()),
        },
        Ok(meta) if !meta.is_dir() => State::Failed {
            reason: format!(
                "{} is not a folder, so the Tool did not delete it",
                folder.display()
            ),
        },
        Ok(_) => match fs::remove_dir_all(folder) {
            Ok(()) => State::Deleted,
            Err(err) => State::Failed {
                reason: format!("could not delete {}: {err}", folder.display()),
            },
        },
    }
}

/// Writes the record to `file` through a temp file and a rename.
fn write_record(file: &Path, record: &[Recorded]) -> Result<()> {
    let temp = file.with_extension("jsonl.tmp");
    fs::write(&temp, record_text(record))
        .and_then(|()| fs::rename(&temp, file))
        .map_err(|err| Error::WriteRecord {
            file: file.to_owned(),
            err,
        })
}

/// Why a missing mod was not downloaded: the message of the bridge when Workbench does not
/// answer.
async fn download_failure(bridge: &Bridge) -> String {
    match bridge.call("ping", Map::new()).await {
        Err(error) => error.to_string(),
        Ok(_) => "Workbench runs, but this version of reference_sync does not download mods yet"
            .to_owned(),
    }
}

fn line(listed: &Listed, license: String, state: State) -> Line {
    Line {
        name: listed.name.clone(),
        id: listed.id.to_string(),
        license,
        state,
    }
}

// endregion: --- Support
