//! The outer layer of `reference_sync`: it reads the lists and the record, lists the `addons`
//! folder, deletes folders, starts the downloads and waits for them through Workbench, and writes
//! the record. The decisions are in
//! `plan.rs`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use serde_json::{Map, Value};
use tokio::time::Instant;

use super::error::{BadLine, Error, Result};
use super::plan::{
    Line, Listed, ModId, Planned, Progress, Recorded, State, answer, folder_of, parse_line,
    parse_progress, parse_record, plan, record_text, recorded_name, unknown_license,
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

/// How long one call waits for its downloads.
pub(super) struct Wait {
    /// The time between two progress questions.
    pub(super) poll: Duration,

    /// The longest wait of one call.
    pub(super) total: Duration,
}

impl Wait {
    /// About every 2 seconds, up to 30 seconds in total.
    pub(super) fn workshop() -> Self {
        Self {
            poll: Duration::from_secs(2),
            total: Duration::from_secs(30),
        }
    }
}

/// The downloads that this server started and that have not ended. A later call asks for their
/// progress and starts none of them again.
pub(super) type Started = Mutex<BTreeSet<ModId>>;

/// Makes the `addons` folder match the lists, and returns the text of the answer.
pub(super) async fn sync(
    settings: &Settings,
    bridge: &Bridge,
    wait: &Wait,
    started: &Started,
) -> Result<String> {
    let mods = settings.mods.clone();
    let documents = settings.documents.clone();
    let cleaned = blocking(move || clean(mods.as_deref(), documents.as_deref())).await?;

    let mut states: Vec<Option<State>> = Vec::new();
    let mut waiting = Vec::new();
    for (index, planned) in cleaned.reference.iter().enumerate() {
        let id = &planned.listed.id;
        let state = if is_started(started, id) {
            waiting.push(index);
            None
        } else if planned.present.is_some() {
            Some(State::Present)
        } else {
            match bridge.call("reference_download", id_param(id)).await {
                Ok(_) => {
                    lock(started).insert(id.clone());
                    waiting.push(index);
                    None
                }
                Err(error) => Some(State::Failed {
                    reason: error.to_string(),
                }),
            }
        };
        states.push(state);
    }

    let progress = wait_for(bridge, wait, &cleaned.reference, &waiting).await;
    let complete: Vec<(ModId, String, String)> = progress
        .iter()
        .filter_map(|(index, progress)| match progress {
            Progress::Complete { version, license } => Some((
                cleaned.reference.get(*index)?.listed.id.clone(),
                version.clone(),
                license.clone(),
            )),
            Progress::Running { .. } | Progress::Failed { .. } => None,
        })
        .collect();
    let addons = cleaned.addons.clone();
    let mut record = cleaned.record;
    let recorded = blocking(move || add_to_record(&addons, &mut record, &complete)).await?;

    let mut lines = Vec::new();
    for (index, planned) in cleaned.reference.iter().enumerate() {
        let id = &planned.listed.id;
        let mut license = planned.present.clone().unwrap_or_else(unknown_license);
        let state = match (
            states.get_mut(index).and_then(Option::take),
            progress.get(&index),
        ) {
            (Some(state), _) => state,
            (None, Some(Progress::Running { percent })) => State::Downloading { percent: *percent },
            (None, Some(Progress::Failed { reason })) => {
                lock(started).remove(id);
                State::Failed {
                    reason: reason.clone(),
                }
            }
            (None, Some(Progress::Complete { license: got, .. })) => {
                lock(started).remove(id);
                if recorded.contains(id) {
                    license.clone_from(got);
                    State::Downloaded
                } else {
                    State::Failed {
                        reason: format!(
                            "Workbench says that the download is complete, but addons has no \
                             folder whose name ends in _{id}"
                        ),
                    }
                }
            }
            (None, None) => State::Failed {
                reason: "the Tool got no progress of the download".to_owned(),
            },
        };
        lines.push(line(&planned.listed, license, state));
    }
    lines.extend(cleaned.deleted);
    Ok(answer(&lines))
}

// region:    --- Support

/// What `clean` did and what the downloads need.
struct Cleaned {
    /// The Reference mods of the plan.
    reference: Vec<Planned>,

    /// The lines of the mods that the call deleted or could not delete.
    deleted: Vec<Line>,

    addons: PathBuf,

    /// The record after the deletes.
    record: Vec<Recorded>,
}

/// Reads the settings, the lists, and the record, makes the plan, deletes, and writes the record.
fn clean(mods: Option<&str>, documents: Option<&str>) -> Result<Cleaned> {
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
    Ok(Cleaned {
        reference: plan.reference,
        deleted,
        addons,
        record,
    })
}

/// Asks for the progress of the downloads of the Reference mods at `waiting` about every
/// `wait.poll`, until each one is complete or failed, or until `wait.total` ends. Returns the
/// last progress of each.
async fn wait_for(
    bridge: &Bridge,
    wait: &Wait,
    reference: &[Planned],
    waiting: &[usize],
) -> BTreeMap<usize, Progress> {
    let deadline = Instant::now() + wait.total;
    let mut progress = BTreeMap::new();
    loop {
        for &index in waiting {
            if matches!(
                progress.get(&index),
                Some(Progress::Complete { .. } | Progress::Failed { .. })
            ) {
                continue;
            }
            let Some(planned) = reference.get(index) else {
                continue;
            };
            let now = match bridge
                .call("reference_progress", id_param(&planned.listed.id))
                .await
            {
                Ok((answer, _)) => parse_progress(&answer),
                Err(error) => Progress::Failed {
                    reason: error.to_string(),
                },
            };
            progress.insert(index, now);
        }
        let running = progress
            .values()
            .any(|now| matches!(now, Progress::Running { .. }));
        if !running || Instant::now() + wait.poll > deadline {
            return progress;
        }
        tokio::time::sleep(wait.poll).await;
    }
}

/// Adds each complete download whose folder is in `addons` to the record, and writes the record
/// when it changed. Returns the IDs that it added.
fn add_to_record(
    addons: &Path,
    record: &mut Vec<Recorded>,
    complete: &[(ModId, String, String)],
) -> Result<BTreeSet<ModId>> {
    if complete.is_empty() {
        return Ok(BTreeSet::new());
    }
    let folders = folder_names(addons)?;
    let mut added = BTreeSet::new();
    for (id, version, license) in complete {
        let Some(folder) = folder_of(id, &folders) else {
            continue;
        };
        let raw = id.to_string();
        record.retain(|entry| !entry.id.eq_ignore_ascii_case(&raw));
        record.push(Recorded {
            id: raw,
            folder: folder.clone(),
            version: version.clone(),
            license: license.clone(),
        });
        added.insert(id.clone());
    }
    if !added.is_empty() {
        write_record(&addons.join(RECORD_FILE), record)?;
    }
    Ok(added)
}

/// Runs the file work `work` away from the thread of the server.
async fn blocking<T: Send + 'static>(
    work: impl FnOnce() -> Result<T> + Send + 'static,
) -> Result<T> {
    tokio::task::spawn_blocking(work)
        .await
        .map_err(|_| Error::Stopped)?
}

/// The parameter `id` of the two Handler actions.
fn id_param(id: &ModId) -> Map<String, Value> {
    let mut params = Map::new();
    params.insert("id".to_owned(), Value::from(id.to_string()));
    params
}

fn is_started(started: &Started, id: &ModId) -> bool {
    lock(started).contains(id)
}

/// The set of started downloads. Each change to it is one insert or one remove, so a lock that
/// a panic poisoned still holds a whole set.
fn lock(started: &Started) -> MutexGuard<'_, BTreeSet<ModId>> {
    started.lock().unwrap_or_else(PoisonError::into_inner)
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

fn line(listed: &Listed, license: String, state: State) -> Line {
    Line {
        name: listed.name.clone(),
        id: listed.id.to_string(),
        license,
        state,
    }
}

// endregion: --- Support
