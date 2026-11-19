//! The pure logic of `reference_sync`: the parse of a list line and of the record, the plan, and
//! the text of the answer. It reads no files and makes no calls.

use std::fmt::{self, Write as _};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::error::BadLine;

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

    /// Whether the folder `name` in `addons` is this mod: its name ends in `_<ID>`.
    fn names(&self, folder: &str) -> bool {
        folder
            .len()
            .checked_sub(17)
            .and_then(|start| folder.get(start..))
            .is_some_and(|end| {
                end.starts_with('_')
                    && end
                        .get(1..)
                        .is_some_and(|id| id.eq_ignore_ascii_case(&self.0))
            })
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

/// One line of the record: a mod that the Tool downloaded.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Recorded {
    pub(super) id: String,
    pub(super) folder: String,
    pub(super) version: String,
    pub(super) license: String,
}

/// What one call does.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Plan {
    /// Each Reference mod once, in the order of the lists.
    pub(super) reference: Vec<Planned>,

    /// The mods in the record that no list has.
    pub(super) delete: Vec<Recorded>,
}

/// A Reference mod and what the call does with it.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct Planned {
    pub(super) listed: Listed,

    /// `None`: the mod is not on disk, so the call downloads it. `Some`: it is on disk, with the
    /// license name from the record, or "unknown".
    pub(super) present: Option<String>,
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

/// The mods in the record text. A record line is valid when it has the four fields, an ID of 16
/// hex digits, and a folder `<Name>_<ID>` with no path in it. `Err`: the number and the reason of
/// the first line that is not valid.
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
        // A struct of four strings always serializes.
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
        let present = folders
            .iter()
            .any(|folder| listed.id.names(folder))
            .then(|| {
                recorded(record, &listed.id)
                    .map_or_else(|| UNKNOWN_LICENSE.to_owned(), |entry| entry.license.clone())
            });
        reference.push(Planned {
            listed: listed.clone(),
            present,
        });
    }
    let delete = record
        .iter()
        .filter(|entry| !lists.iter().any(|listed| same_id(&listed.id, &entry.id)))
        .cloned()
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

/// The folder of the mod `id` in `folders`: the name that ends in `_<ID>`.
pub(super) fn folder_of<'a>(id: &ModId, folders: &'a [String]) -> Option<&'a String> {
    folders.iter().find(|folder| id.names(folder))
}

/// The name of a recorded mod: its folder name without `_<ID>`.
pub(super) fn recorded_name(entry: &Recorded) -> &str {
    entry
        .folder
        .len()
        .checked_sub(17)
        .and_then(|end| entry.folder.get(..end))
        .unwrap_or(&entry.folder)
}

/// The license name of a mod that is not in the record.
pub(super) fn unknown_license() -> String {
    UNKNOWN_LICENSE.to_owned()
}

// region:    --- Support

fn parse_record_line(line: &str) -> Result<Recorded, BadLine> {
    let entry: Recorded = serde_json::from_str(line).map_err(|err| BadLine::NotJson {
        detail: err.to_string(),
    })?;
    let id = ModId::parse(&entry.id).ok_or_else(|| BadLine::BadId {
        id: entry.id.clone(),
    })?;
    let plain = !entry.folder.contains(['/', '\\']) && !entry.folder.starts_with('.');
    if !(plain && id.names(&entry.folder)) {
        return Err(BadLine::BadFolder {
            folder: entry.folder,
        });
    }
    Ok(entry)
}

fn recorded<'a>(record: &'a [Recorded], id: &ModId) -> Option<&'a Recorded> {
    record.iter().find(|entry| same_id(id, &entry.id))
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
    fn plans_delete_download_and_present() {
        // -- Setup & Fixtures
        let listed = |id: &str, name: &str| Listed {
            id: ModId::parse(id).unwrap(),
            name: name.to_owned(),
        };
        let recorded = |id: &str, folder: &str| Recorded {
            id: id.to_owned(),
            folder: folder.to_owned(),
            version: "1.0.0".to_owned(),
            license: "MIT".to_owned(),
        };
        let lists = [
            listed("6AA81EE549F50CB7", "tvtLobby"),
            listed("6a09759015ffda26", "S3C_CameraFix"),
            listed("6AA81EE549F50CB7", "tvtLobby again"),
        ];
        let record = [
            recorded("6AA81EE549F50CB7", "tvtLobby_6AA81EE549F50CB7"),
            recorded("5965770215E93269", "OldMod_5965770215E93269"),
        ];
        let folders = ["tvtLobby_6aa81ee549f50cb7".to_owned(), "saves".to_owned()];

        // -- Exec
        let plan = plan(&lists, &record, &folders);

        // -- Check
        assert_eq!(
            plan,
            Plan {
                reference: vec![
                    Planned {
                        listed: listed("6AA81EE549F50CB7", "tvtLobby"),
                        present: Some("MIT".to_owned()),
                    },
                    Planned {
                        listed: listed("6A09759015FFDA26", "S3C_CameraFix"),
                        present: None,
                    },
                ],
                delete: vec![recorded("5965770215E93269", "OldMod_5965770215E93269")],
            }
        );
    }

    #[test]
    fn refuses_a_record_folder_with_a_path_in_it() {
        // -- Exec
        let result = parse_record(
            r#"{"id":"5965770215E93269","folder":"../x_5965770215E93269","version":"1","license":"MIT"}"#,
        );

        // -- Check
        assert!(matches!(result, Err((1, BadLine::BadFolder { .. }))));
    }
}

// endregion: --- Tests
