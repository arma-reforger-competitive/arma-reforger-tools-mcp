//! The pure logic of `reference_sync`: the parse of a list line and of the record, the plan, and
//! the text of the answer. It reads no files and makes no calls.

use std::fmt::{self, Write as _};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::error::BadLine;
use crate::mod_folder::ModFolder;

/// The license name of a mod that is not in the record.
const UNKNOWN_LICENSE: &str = "unknown";

/// The Workshop ID of a mod: 16 hex digits, kept in capital letters, so IDs compare without
/// regard to capital letters.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ModId(String);

impl ModId {
    /// The ID in `raw`, or `None` when `raw` is not 16 hex digits.
    pub(super) fn parse(raw: &str) -> Option<Self> {
        (raw.len() == 16 && raw.bytes().all(|b| b.is_ascii_hexdigit()))
            .then(|| Self(raw.to_ascii_uppercase()))
    }

    /// Whether the folder `folder` in `addons` is this mod: its name is `<Name>_<ID>`.
    fn names(&self, folder: &str) -> bool {
        ModFolder::parse(folder).is_some_and(|parts| parts.is(&self.0))
    }
}

impl fmt::Display for ModId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// One line of a list: a Reference mod.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Listed {
    pub(super) id: ModId,
    pub(super) name: String,
}

/// One line of the record: a mod that the Tool asked Workbench to download. The record has at
/// most one line for each ID.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "state", rename_all = "lowercase", deny_unknown_fields)]
pub(super) enum Recorded {
    /// Workbench accepted the start, and the Tool has not seen the download end.
    Started { id: String, name: String },

    /// The download is complete, and its folder `<Name>_<ID>` was on disk.
    Downloaded {
        id: String,
        folder: String,
        version: String,
        license: String,
    },
}

impl Recorded {
    pub(super) fn id(&self) -> &str {
        match self {
            Self::Started { id, .. } | Self::Downloaded { id, .. } => id,
        }
    }

    /// The mod name: the name in the list for a started download, the name in the folder
    /// `<Name>_<ID>` for a downloaded mod.
    pub(super) fn name(&self) -> &str {
        match self {
            Self::Started { name, .. } => name,
            Self::Downloaded { folder, .. } => {
                ModFolder::parse(folder).map_or(folder.as_str(), |parts| parts.name)
            }
        }
    }

    /// The license name, or "unknown" for a started download.
    pub(super) fn license(&self) -> &str {
        match self {
            Self::Started { .. } => UNKNOWN_LICENSE,
            Self::Downloaded { license, .. } => license,
        }
    }
}

/// What one call does.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Plan {
    /// Each Reference mod once, in the order of the lists.
    pub(super) reference: Vec<Planned>,

    /// The mods in the record that no list has.
    pub(super) delete: Vec<Delete>,
}

/// A Reference mod and what the call does with it.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Planned {
    pub(super) listed: Listed,
    pub(super) action: Action,
}

/// What the call does with a Reference mod.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Action {
    /// It is on disk and has no started download. The license name comes from the record, or is
    /// "unknown" when the record has no line for it (the user downloaded it by hand).
    Present { license: String },

    /// It is not on disk and has no started download: start one.
    Download,

    /// The record has a started download: ask Workbench for its progress. `folder`: its folder
    /// `<Name>_<ID>` on disk, if one is there.
    Follow { folder: Option<String> },
}

/// A mod in the record that no list has.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Delete {
    pub(super) entry: Recorded,

    /// The folder to delete. `None`: a started download has no folder on disk, so the call only
    /// removes its line.
    pub(super) folder: Option<String>,
}

/// One line of the answer.
pub(super) struct Line {
    pub(super) name: String,
    pub(super) id: String,
    pub(super) license: String,
    pub(super) state: State,
}

/// The state of one mod after the call.
pub(super) enum State {
    Present,
    Downloaded,
    Downloading { percent: u8 },
    Deleted,
    Failed { reason: String },
}

/// The progress of one download, as the Handler script answers `reference_progress`.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum Progress {
    /// The download runs. The search on the Workshop counts as 0 percent. The percent is a whole
    /// number from 0 to 100.
    Running {
        percent: u8,
    },
    Complete {
        version: String,
        license: String,
    },
    Failed {
        reason: String,
    },

    /// Workbench started no download of this ID since it loaded the scripts.
    Unknown,
}

/// The fields of a list line, before the check of their values.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLine {
    id: String,
    name: String,
}

/// The Reference mod on one list line. `Ok(None)`: the line is empty.
pub(super) fn parse_line(line: &str) -> Result<Option<Listed>, BadLine> {
    if line.trim().is_empty() {
        return Ok(None);
    }
    let raw: RawLine = serde_json::from_str(line).map_err(|err| BadLine::NotJson {
        detail: err.to_string(),
    })?;
    let id = ModId::parse(&raw.id).ok_or(BadLine::BadId { id: raw.id })?;
    if raw.name.trim().is_empty() {
        return Err(BadLine::EmptyName);
    }
    Ok(Some(Listed { id, name: raw.name }))
}

/// The mods in the record text. A record line is valid when it has the fields of its state and an
/// ID of 16 hex digits. A started line has a name that is not empty. A downloaded line has a folder
/// `<Name>_<ID>` with no path in it. `Err`: the number and the reason of the first line that is not
/// valid.
pub(super) fn parse_record(text: &str) -> Result<Vec<Recorded>, (usize, BadLine)> {
    let mut record = Vec::new();
    for (index, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let entry = parse_record_line(line).map_err(|reason| (index + 1, reason))?;
        record.push(entry);
    }
    Ok(record)
}

/// The text of the record file.
pub(super) fn record_text(record: &[Recorded]) -> String {
    record.iter().fold(String::new(), |mut text, entry| {
        // An enum of strings always serializes.
        if let Ok(line) = serde_json::to_string(entry) {
            text.push_str(&line);
            text.push('\n');
        }
        text
    })
}

/// The plan from the Reference mods of all lists, the record, and the folder names in `addons`.
pub(super) fn plan(lists: &[Listed], record: &[Recorded], folders: &[String]) -> Plan {
    let mut reference: Vec<Planned> = Vec::new();
    for listed in lists {
        if reference
            .iter()
            .any(|planned| planned.listed.id == listed.id)
        {
            continue;
        }
        let folder = folder_of(&listed.id, folders);
        let action = match (recorded(record, &listed.id), folder) {
            (Some(Recorded::Started { .. }), folder) => Action::Follow {
                folder: folder.cloned(),
            },
            (entry, Some(_)) => Action::Present {
                license: entry.map_or(UNKNOWN_LICENSE, Recorded::license).to_owned(),
            },
            (_, None) => Action::Download,
        };
        reference.push(Planned {
            listed: listed.clone(),
            action,
        });
    }
    let delete = record
        .iter()
        .filter(|entry| !lists.iter().any(|listed| same_id(&listed.id, entry.id())))
        .map(|entry| Delete {
            entry: entry.clone(),
            folder: match entry {
                Recorded::Started { .. } => {
                    ModId::parse(entry.id()).and_then(|id| folder_of(&id, folders).cloned())
                }
                Recorded::Downloaded { folder, .. } => Some(folder.clone()),
            },
        })
        .collect();
    Plan { reference, delete }
}

/// The text of the answer: one line for each mod.
pub(super) fn answer(lines: &[Line]) -> String {
    if lines.is_empty() {
        return "No list has a Reference mod, and the record has no mod to delete.".to_owned();
    }
    let mut text = String::new();
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            text.push('\n');
        }
        let _ = write!(
            text,
            "{} ({}), license {}: ",
            line.name, line.id, line.license
        );
        match &line.state {
            State::Present => text.push_str("present"),
            State::Downloaded => text.push_str("downloaded"),
            State::Downloading { percent } => {
                let _ = write!(text, "downloading {percent}%");
            }
            State::Deleted => text.push_str("deleted"),
            State::Failed { reason } => {
                let _ = write!(text, "failed: {reason}");
            }
        }
    }
    text
}

/// The progress in the answer of the Handler script to `reference_progress`. The mod version is
/// in `mod_version`, because the bridge uses `version` for the version of the Handler script.
pub(super) fn parse_progress(answer: &Map<String, Value>) -> Progress {
    let text = |field| {
        answer
            .get(field)
            .and_then(Value::as_str)
            .unwrap_or(UNKNOWN_LICENSE)
            .to_owned()
    };
    match answer.get("state").and_then(Value::as_str) {
        Some("searching") => Progress::Running { percent: 0 },
        Some("downloading") => Progress::Running {
            percent: answer
                .get("percent")
                .and_then(Value::as_u64)
                .map_or(0, |percent| u8::try_from(percent.min(100)).unwrap_or(100)),
        },
        Some("complete") => Progress::Complete {
            version: text("mod_version"),
            license: text("license"),
        },
        Some("unknown") => Progress::Unknown,
        Some("failed") => Progress::Failed {
            reason: answer
                .get("reason")
                .and_then(Value::as_str)
                .unwrap_or("Workbench gave no reason")
                .to_owned(),
        },
        other => Progress::Failed {
            reason: format!(
                "the Handler script gave the download state {}, which the Tool does not know",
                other.map_or_else(|| "none".to_owned(), |state| format!("\"{state}\""))
            ),
        },
    }
}

/// The folder of the mod `id` in `folders`: the name `<Name>_<ID>`.
pub(super) fn folder_of<'a>(id: &ModId, folders: &'a [String]) -> Option<&'a String> {
    folders.iter().find(|folder| id.names(folder))
}

/// The license name of a mod that is not in the record.
pub(super) fn unknown_license() -> String {
    UNKNOWN_LICENSE.to_owned()
}

/// Puts `entry` into `record` in place of the line of its ID.
pub(super) fn set(record: &mut Vec<Recorded>, entry: Recorded) {
    record.retain(|kept| !kept.id().eq_ignore_ascii_case(entry.id()));
    record.push(entry);
}

/// Removes the started line of `id`. A downloaded line stays.
pub(super) fn remove_started(record: &mut Vec<Recorded>, id: &ModId) {
    record.retain(|kept| !(matches!(kept, Recorded::Started { .. }) && same_id(id, kept.id())));
}

// region:    --- Support

fn parse_record_line(line: &str) -> Result<Recorded, BadLine> {
    let entry: Recorded = serde_json::from_str(line).map_err(|err| BadLine::NotJson {
        detail: err.to_string(),
    })?;
    let id = ModId::parse(entry.id()).ok_or_else(|| BadLine::BadId {
        id: entry.id().to_owned(),
    })?;
    match &entry {
        Recorded::Started { name, .. } if name.trim().is_empty() => Err(BadLine::EmptyName),
        Recorded::Downloaded { folder, .. }
            if folder.contains(['/', '\\']) || folder.starts_with('.') || !id.names(folder) =>
        {
            Err(BadLine::BadFolder {
                folder: folder.clone(),
            })
        }
        Recorded::Started { .. } | Recorded::Downloaded { .. } => Ok(entry),
    }
}

fn recorded<'a>(record: &'a [Recorded], id: &ModId) -> Option<&'a Recorded> {
    record.iter().find(|entry| same_id(id, entry.id()))
}

fn same_id(id: &ModId, raw: &str) -> bool {
    id.0.eq_ignore_ascii_case(raw)
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn plans_delete_download_present_and_follow() {
        // -- Setup & Fixtures
        let listed = |id: &str, name: &str| Listed {
            id: ModId::parse(id).unwrap(),
            name: name.to_owned(),
        };
        let downloaded = |id: &str, folder: &str| Recorded::Downloaded {
            id: id.to_owned(),
            folder: folder.to_owned(),
            version: "1.0.0".to_owned(),
            license: "MIT".to_owned(),
        };
        let started = |id: &str, name: &str| Recorded::Started {
            id: id.to_owned(),
            name: name.to_owned(),
        };
        let lists = [
            listed("6AA81EE549F50CB7", "tvtLobby"),
            listed("6a09759015ffda26", "S3C_CameraFix"),
            listed("6AA81EE549F50CB7", "tvtLobby again"),
            listed("591AF5BDA9F7CE8B", "ByHand"),
            listed("1111111111111111", "Running"),
            listed("2222222222222222", "DoneAfterRestart"),
        ];
        let record = [
            downloaded("6AA81EE549F50CB7", "tvtLobby_6AA81EE549F50CB7"),
            downloaded("5965770215E93269", "OldMod_5965770215E93269"),
            started("1111111111111111", "Running"),
            started("2222222222222222", "DoneAfterRestart"),
            started("3333333333333333", "StartedOnDisk"),
            started("4444444444444444", "StartedNotOnDisk"),
        ];
        let folders = [
            "tvtLobby_6aa81ee549f50cb7".to_owned(),
            "_6A09759015FFDA26".to_owned(),
            "ByHand_591AF5BDA9F7CE8B".to_owned(),
            "DoneAfterRestart_2222222222222222".to_owned(),
            "StartedOnDisk_3333333333333333".to_owned(),
            "saves".to_owned(),
        ];

        // -- Exec
        let plan = plan(&lists, &record, &folders);

        // -- Check
        let planned = |listed, action| Planned { listed, action };
        assert_eq!(
            plan,
            Plan {
                reference: vec![
                    planned(
                        listed("6AA81EE549F50CB7", "tvtLobby"),
                        Action::Present {
                            license: "MIT".to_owned()
                        }
                    ),
                    planned(
                        listed("6A09759015FFDA26", "S3C_CameraFix"),
                        Action::Download
                    ),
                    planned(
                        listed("591AF5BDA9F7CE8B", "ByHand"),
                        Action::Present {
                            license: "unknown".to_owned()
                        }
                    ),
                    planned(
                        listed("1111111111111111", "Running"),
                        Action::Follow { folder: None }
                    ),
                    planned(
                        listed("2222222222222222", "DoneAfterRestart"),
                        Action::Follow {
                            folder: Some("DoneAfterRestart_2222222222222222".to_owned())
                        }
                    ),
                ],
                delete: vec![
                    Delete {
                        entry: downloaded("5965770215E93269", "OldMod_5965770215E93269"),
                        folder: Some("OldMod_5965770215E93269".to_owned()),
                    },
                    Delete {
                        entry: started("3333333333333333", "StartedOnDisk"),
                        folder: Some("StartedOnDisk_3333333333333333".to_owned()),
                    },
                    Delete {
                        entry: started("4444444444444444", "StartedNotOnDisk"),
                        folder: None,
                    },
                ],
            }
        );
    }

    #[test]
    fn reads_back_the_record_that_it_writes() {
        // -- Setup & Fixtures
        let record = vec![
            Recorded::Started {
                id: "6A09759015FFDA26".to_owned(),
                name: "S3C_CameraFix".to_owned(),
            },
            Recorded::Downloaded {
                id: "6AA81EE549F50CB7".to_owned(),
                folder: "tvtLobby_6AA81EE549F50CB7".to_owned(),
                version: "1.0.0".to_owned(),
                license: "MIT".to_owned(),
            },
        ];

        // -- Exec
        let text = record_text(&record);

        // -- Check
        assert_eq!(
            text,
            "{\"state\":\"started\",\"id\":\"6A09759015FFDA26\",\"name\":\"S3C_CameraFix\"}\n\
             {\"state\":\"downloaded\",\"id\":\"6AA81EE549F50CB7\",\
             \"folder\":\"tvtLobby_6AA81EE549F50CB7\",\"version\":\"1.0.0\",\"license\":\"MIT\"}\n"
        );
        assert_eq!(parse_record(&text).unwrap(), record);
    }

    #[test]
    fn refuses_a_record_folder_with_a_path_in_it() {
        // -- Exec
        let result = parse_record(
            r#"{"state":"downloaded","id":"5965770215E93269","folder":"../x_5965770215E93269","version":"1","license":"MIT"}"#,
        );

        // -- Check
        assert!(matches!(result, Err((1, BadLine::BadFolder { .. }))));
    }
}

// endregion: --- Tests
