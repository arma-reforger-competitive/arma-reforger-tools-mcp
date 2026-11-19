//! The outer layer of `reference_sync`: it reads the lists and the record, lists the `addons`
//! folder, deletes folders, starts the downloads and waits for them through Workbench, and writes
//! the record. The decisions are in
//! `plan.rs`.

use std::collections::BTreeMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::Duration;

use serde_json::{Map, Value};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;

use super::error::BadLine;
use super::plan::{
    Action, Delete, Line, Listed, ModId, Planned, Progress, Recorded, State, answer, folder_of,
    parse_line, parse_progress, parse_record, plan, record_text, remove_started, set,
    unknown_license,
};
use super::{Error, Result};
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

/// Makes the `addons` folder match the lists, and returns the text of the answer.
pub(super) async fn sync(
    settings: &Settings,
    bridge: &Bridge,
    wait: &Wait,
    cancel: CancellationToken,
) -> Result<String> {
    let mods = settings.mods.clone();
    let documents = settings.documents.clone();
    let clean_cancel = cancel.clone();
    let Cleaned {
        reference,
        deleted,
        addons,
        mut record,
    } = blocking(move || clean(mods.as_deref(), documents.as_deref(), &clean_cancel)).await?;

    // The state of each Reference mod that is final before the wait.
    let mut states: Vec<Option<State>> = Vec::new();
    let mut known: BTreeMap<usize, Known> = BTreeMap::new();
    let mut started_any = false;
    for (index, planned) in reference.iter().enumerate() {
        let id = &planned.listed.id;
        let start = match &planned.action {
            Action::Present { .. } => false,
            Action::Download => true,
            Action::Follow { folder } => match progress(bridge, id).await {
                Known::Answer(Progress::Unknown) if folder.is_some() => {
                    // Workbench lost the download, for example after a restart, but its folder
                    // is on disk.
                    known.insert(
                        index,
                        Known::Answer(Progress::Complete {
                            version: unknown_license(),
                            license: unknown_license(),
                        }),
                    );
                    false
                }
                Known::Answer(Progress::Unknown) => true,
                now => {
                    known.insert(index, now);
                    false
                }
            },
        };
        let state = match &planned.action {
            Action::Present { .. } => Some(State::Present),
            Action::Download | Action::Follow { .. } if !start => None,
            Action::Download | Action::Follow { .. } => {
                match bridge.call("reference_download", id_param(id)).await {
                    Ok(_) => {
                        set(
                            &mut record,
                            Recorded::Started {
                                id: id.to_string(),
                                name: planned.listed.name.clone(),
                            },
                        );
                        started_any = true;
                        known.insert(index, Known::Answer(Progress::Running { percent: 0 }));
                        None
                    }
                    Err(error) => Some(State::Failed {
                        reason: error.to_string(),
                    }),
                }
            }
        };
        states.push(state);
    }
    if started_any {
        let file = addons.join(RECORD_FILE);
        let kept = record.clone();
        blocking(move || write_record(&file, &kept)).await?;
    }

    wait_for(bridge, wait, &reference, &mut known).await;
    let ids: BTreeMap<usize, ModId> = known
        .keys()
        .filter_map(|&index| Some((index, reference.get(index)?.listed.id.clone())))
        .collect();
    let mut settled = blocking(move || settle(&addons, record, &ids, known, &cancel)).await?;

    let mut lines = Vec::new();
    for (index, planned) in reference.iter().enumerate() {
        let (license, state) = match (
            states.get_mut(index).and_then(Option::take),
            &planned.action,
        ) {
            (Some(state), Action::Present { license }) => (license.clone(), state),
            (Some(state), _) => (unknown_license(), state),
            (None, _) => settled.remove(&index).unwrap_or_else(|| {
                (
                    unknown_license(),
                    State::Failed {
                        reason: "the Tool got no progress of the download".to_owned(),
                    },
                )
            }),
        };
        lines.push(line(&planned.listed, license, state));
    }
    lines.extend(deleted);
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
/// It checks `cancel` before each list and each delete. After a cancel, the record names only the
/// folders that are still on disk.
fn clean(
    mods: Option<&str>,
    documents: Option<&str>,
    cancel: &CancellationToken,
) -> Result<Cleaned> {
    let mods = mods.ok_or(Error::ModsNotSet).map(host_path)?;
    if !mods.is_dir() {
        return Err(Error::ModsNotFolder { folder: mods });
    }
    let addons = workbench_documents(documents)
        .ok_or(Error::DocumentsNotFound)?
        .join("addons");
    let lists = read_lists(&mods, cancel)?;
    let record_file = addons.join(RECORD_FILE);
    let mut record = read_record(&record_file)?;
    let folders = folder_names(&addons)?;

    let plan = plan(&lists, &record, &folders);

    let mut deleted = Vec::new();
    for Delete { entry, folder } in &plan.delete {
        if cancel.is_cancelled() {
            break;
        }
        let state = folder
            .as_ref()
            .map_or(State::Deleted, |folder| delete(&addons.join(folder)));
        if matches!(state, State::Deleted) {
            record.retain(|kept| kept != entry);
        }
        deleted.push(Line {
            name: entry.name().to_owned(),
            id: entry.id().to_ascii_uppercase(),
            license: entry.license().to_owned(),
            state,
        });
    }
    if deleted
        .iter()
        .any(|line| matches!(line.state, State::Deleted))
    {
        write_record(&record_file, &record)?;
    }
    if cancel.is_cancelled() {
        return Err(Error::Cancelled);
    }
    Ok(Cleaned {
        reference: plan.reference,
        deleted,
        addons,
        record,
    })
}

/// What the call knows about one download.
enum Known {
    /// The answer of the Handler script.
    Answer(Progress),

    /// Workbench gave no answer. The reason is the message of the bridge.
    NoAnswer { reason: String },
}

/// Asks Workbench for the progress of the download of `id`.
async fn progress(bridge: &Bridge, id: &ModId) -> Known {
    match bridge.call("reference_progress", id_param(id)).await {
        Ok((answer, _)) => Known::Answer(parse_progress(&answer)),
        Err(error) => Known::NoAnswer {
            reason: error.to_string(),
        },
    }
}

/// Asks for the progress of each running download in `known` about every `wait.poll`, until each
/// one ends, or until `wait.total` ends. Keeps the last progress of each in `known`.
async fn wait_for(
    bridge: &Bridge,
    wait: &Wait,
    reference: &[Planned],
    known: &mut BTreeMap<usize, Known>,
) {
    let deadline = Instant::now() + wait.total;
    loop {
        for (&index, now) in known.iter_mut() {
            if !matches!(now, Known::Answer(Progress::Running { .. })) {
                continue;
            }
            let Some(planned) = reference.get(index) else {
                continue;
            };
            *now = progress(bridge, &planned.listed.id).await;
        }
        let running = known
            .values()
            .any(|now| matches!(now, Known::Answer(Progress::Running { .. })));
        if !running || Instant::now() + wait.poll > deadline {
            return;
        }
        tokio::time::sleep(wait.poll).await;
    }
}

/// Puts the end of each download in `known` into `record`, and writes the record when it changed.
/// A complete download whose folder is in `addons` becomes a downloaded line. A complete download
/// with no folder and a failed download lose their started line. A running download, and one that
/// Workbench did not answer for, keep it. Returns the license name and the state of each.
fn settle(
    addons: &Path,
    mut record: Vec<Recorded>,
    ids: &BTreeMap<usize, ModId>,
    known: BTreeMap<usize, Known>,
    cancel: &CancellationToken,
) -> Result<BTreeMap<usize, (String, State)>> {
    let before = record.clone();
    let folders = folder_names(addons)?;
    let mut settled = BTreeMap::new();
    for (index, now) in known {
        if cancel.is_cancelled() {
            break;
        }
        let Some(id) = ids.get(&index) else {
            continue;
        };
        let end = match now {
            Known::Answer(Progress::Running { percent }) => {
                (unknown_license(), State::Downloading { percent })
            }
            Known::Answer(Progress::Complete { version, license }) => {
                if let Some(folder) = folder_of(id, &folders) {
                    set(
                        &mut record,
                        Recorded::Downloaded {
                            id: id.to_string(),
                            folder: folder.clone(),
                            version,
                            license: license.clone(),
                        },
                    );
                    (license, State::Downloaded)
                } else {
                    remove_started(&mut record, id);
                    let reason = format!(
                        "Workbench says that the download is complete, but addons has no folder \
                         <Name>_{id}"
                    );
                    (unknown_license(), State::Failed { reason })
                }
            }
            Known::Answer(Progress::Failed { reason }) => {
                remove_started(&mut record, id);
                (unknown_license(), State::Failed { reason })
            }
            Known::Answer(Progress::Unknown) => (
                unknown_license(),
                State::Failed {
                    reason: "Workbench no longer knows this download, for example because it \
                             loaded the scripts again. Call again."
                        .to_owned(),
                },
            ),
            Known::NoAnswer { reason } => (unknown_license(), State::Failed { reason }),
        };
        settled.insert(index, end);
    }
    if record != before {
        write_record(&addons.join(RECORD_FILE), &record)?;
    }
    if cancel.is_cancelled() {
        return Err(Error::Cancelled);
    }
    Ok(settled)
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

/// The Reference mods of the lists in each direct subfolder of `mods`, in folder order. It checks
/// `cancel` before each list.
fn read_lists(mods: &Path, cancel: &CancellationToken) -> Result<Vec<Listed>> {
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
        if cancel.is_cancelled() {
            return Err(Error::Cancelled);
        }
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

// region:    --- Tests

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;

    #[test]
    fn deletes_nothing_after_a_cancel() {
        // -- Setup & Fixtures
        let mods = TempDir::new().unwrap();
        let documents = TempDir::new().unwrap();
        let addons = documents.path().join("addons");
        let folder = addons.join("OldMod_5965770215E93269");
        fs::create_dir_all(&folder).unwrap();
        let record = "{\"state\":\"downloaded\",\"id\":\"5965770215E93269\",\"folder\":\"OldMod_5965770215E93269\",\
                      \"version\":\"1.0.0\",\"license\":\"MIT\"}\n";
        fs::write(addons.join(RECORD_FILE), record).unwrap();
        let cancel = CancellationToken::new();
        cancel.cancel();

        // -- Exec
        let result = clean(mods.path().to_str(), documents.path().to_str(), &cancel);

        // -- Check
        assert!(matches!(result, Err(Error::Cancelled)));
        assert!(folder.is_dir());
        assert_eq!(
            fs::read_to_string(addons.join(RECORD_FILE)).unwrap(),
            record
        );
    }
}

// endregion: --- Tests
