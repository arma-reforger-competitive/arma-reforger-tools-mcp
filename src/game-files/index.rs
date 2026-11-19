//! The list of every file in the .pak archives of the game and of the downloaded Workshop mods.

use std::collections::HashSet;
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use super::pak::{FileData, PakFile, list_files, read_file};
use super::settings::{PROFILE_SETTING, Settings};
use super::{Error, Result};
use crate::host_path::host_path;

/// The name of the first source: the game itself.
const GAME: &str = "game";

/// Every Game file, with the names of the sources and notes about what was not searched.
pub(super) struct Index {
    pub(super) files: Vec<GameFile>,

    /// `GAME` first, then the name of each mod.
    pub(super) sources: Vec<String>,

    /// Each line says what the search did not cover and how to fix it.
    pub(super) notes: Vec<String>,

    /// The mod folders that the index was built from. When the folders on disk differ, the index
    /// is old.
    pub(super) mod_folders: Vec<PathBuf>,
}

/// One file in a .pak archive.
pub(super) struct GameFile {
    pub(super) path: String,

    /// The position of its source in `Index::sources`.
    pub(super) source: usize,

    /// The .pak file that holds it.
    pub(super) pak: Arc<Path>,

    data: FileData,
}

impl GameFile {
    /// Reads its full content from its .pak file.
    pub(super) fn read(&self) -> Result<Vec<u8>> {
        read_file(&mut File::open(&self.pak)?, self.data)
    }
}

/// Reads the file lists of all .pak archives of the game and of the mods. It fails when the game
/// folder is not set or has no .pak files. A damaged .pak file or a missing mods folder becomes a
/// note. It checks `cancel` before each .pak file.
pub(super) fn build(settings: &Settings, cancel: &CancellationToken) -> Result<Index> {
    let raw_game = settings.game.as_deref().ok_or(Error::GamePathNotSet)?;
    let game_folder = host_path(raw_game);
    let mut game_paks = Vec::new();
    collect_paks(&game_folder.join("addons"), 2, &mut game_paks);
    if game_paks.is_empty() {
        return Err(Error::NoGamePaks {
            value: raw_game.to_owned(),
            folder: game_folder,
        });
    }
    let mut index = Index {
        files: Vec::new(),
        sources: vec![GAME.to_owned()],
        notes: Vec::new(),
        mod_folders: Vec::new(),
    };
    index.add(0, &game_paks, cancel)?;
    let ModFolders { folders, notes } = mod_folders(settings);
    index.notes.extend(notes);
    for folder in &folders {
        let mut paks = Vec::new();
        collect_paks(folder, 1, &mut paks);
        if paks.is_empty() {
            continue;
        }
        index.sources.push(mod_name(folder));
        index.add(index.sources.len() - 1, &paks, cancel)?;
    }
    index.mod_folders = folders;
    Ok(index)
}

/// The mod folders in both `addons` folders, with notes about a folder that was not searched.
pub(super) struct ModFolders {
    /// Sorted by folder name. A mod ID that is in both `addons` folders is here once, from the
    /// Documents folder.
    pub(super) folders: Vec<PathBuf>,
    notes: Vec<String>,
}

/// Lists the mod folders in `<Documents folder>/addons` and in `<profile>/addons`. A folder that
/// is missing becomes a note.
pub(super) fn mod_folders(settings: &Settings) -> ModFolders {
    let mut notes = Vec::new();
    let mut folders = Vec::new();
    if let Some(documents) = &settings.documents {
        folders.extend(sub_folders(&documents.join("addons")).unwrap_or_default());
    }
    let has_documents = settings.documents.is_some();
    match settings.profile.as_deref() {
        None if !has_documents => notes.push(format!(
            "{PROFILE_SETTING} is not set, so the search covered the game only. To also search the \
             Workshop mods that Workbench downloaded, set it to the Workbench profile folder, for \
             example \"C:\\Users\\<you>\\Documents\\My Games\\ArmaReforgerWorkbench\"."
        )),
        None => {}
        Some(raw_profile) => {
            let mods_folder = host_path(raw_profile).join("addons");
            if let Some(profile_folders) = sub_folders(&mods_folder) {
                let in_documents: HashSet<String> = folders.iter().map(|f| mod_key(f)).collect();
                folders.extend(
                    profile_folders
                        .into_iter()
                        .filter(|folder| !in_documents.contains(&mod_key(folder))),
                );
            } else {
                let covered = if has_documents {
                    "the search did not cover its mods"
                } else {
                    "the search covered the game only"
                };
                notes.push(format!(
                    "{PROFILE_SETTING} is \"{raw_profile}\", but {} is not a folder, so \
                     {covered}. Set it to the Workbench profile folder, the folder that holds \
                     addons.",
                    mods_folder.display()
                ));
            }
        }
    }
    folders.sort_by_key(|folder| folder.file_name().map(ToOwned::to_owned));
    ModFolders { folders, notes }
}

impl Index {
    /// Where `file` comes from: "game", or "mod tvtLobby".
    pub(super) fn source_of(&self, file: &GameFile) -> String {
        match self.sources.get(file.source) {
            Some(name) if file.source > 0 => format!("mod {name}"),
            _ => GAME.to_owned(),
        }
    }

    /// What the search covered: "the game", or "the game and 2 mods (tvtLobby, Other)".
    pub(super) fn searched(&self) -> String {
        let mods = self.sources.get(1..).unwrap_or_default();
        match mods.len() {
            0 => "the game".to_owned(),
            1 => format!("the game and 1 mod ({})", mods.join(", ")),
            count => format!("the game and {count} mods ({})", mods.join(", ")),
        }
    }

    /// Adds the files of `paks` from the source at `source`.
    fn add(&mut self, source: usize, paks: &[PathBuf], cancel: &CancellationToken) -> Result<()> {
        for pak in paks {
            if cancel.is_cancelled() {
                return Err(Error::Cancelled);
            }
            match read_pak(pak) {
                Ok(files) => {
                    let pak: Arc<Path> = Arc::from(pak.as_path());
                    self.files
                        .extend(files.into_iter().map(|PakFile { path, data }| GameFile {
                            path,
                            source,
                            pak: Arc::clone(&pak),
                            data,
                        }));
                }
                Err(error) => self.notes.push(format!(
                    "Skipped {}: {error}. The file may be damaged: verify the game files in Steam, \
                     or download the mod again.",
                    pak.display()
                )),
            }
        }
        Ok(())
    }
}

// region:    --- Support

fn read_pak(pak: &Path) -> Result<Vec<PakFile>> {
    let mut file = File::open(pak)?;
    let len = file.metadata()?.len();
    list_files(&mut file, len)
}

/// Adds the .pak files in `folder` to `paks`, and those in its subfolders down to `depth` levels.
/// The game keeps them in `addons/<name>/`, a mod in `addons/<mod>/`.
fn collect_paks(folder: &Path, depth: usize, paks: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(folder) else {
        return;
    };
    let mut entries: Vec<PathBuf> = entries.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            if depth > 1 {
                collect_paks(&entry, depth - 1, paks);
            }
        } else if entry
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("pak"))
        {
            paks.push(entry);
        }
    }
}

/// The folders in `folder`, or `None` when it is not a folder.
fn sub_folders(folder: &Path) -> Option<Vec<PathBuf>> {
    let entries = fs::read_dir(folder).ok()?;
    Some(
        entries
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|folder| folder.is_dir())
            .collect(),
    )
}

/// What makes two mod folders the same mod: the ID of `<Name>_<ID>` in capitals, or else the
/// whole folder name.
fn mod_key(folder: &Path) -> String {
    let name = folder.file_name().unwrap_or_default().to_string_lossy();
    match split_id(&name) {
        Some((_, id)) => id.to_ascii_uppercase(),
        None => name.into_owned(),
    }
}

/// The name and the 16-digit hex ID of a mod folder `<Name>_<ID>`.
fn split_id(folder: &str) -> Option<(&str, &str)> {
    let (name, id) = folder.rsplit_once('_')?;
    (!name.is_empty() && id.len() == 16 && id.chars().all(|c| c.is_ascii_hexdigit()))
        .then_some((name, id))
}

/// The name of a mod from its folder `<Name>_<ID>`, where the ID has 16 hex digits.
fn mod_name(folder: &Path) -> String {
    let folder = folder.file_name().unwrap_or_default().to_string_lossy();
    match split_id(&folder) {
        Some((name, _)) => name.to_owned(),
        None => folder.into_owned(),
    }
}

// endregion: --- Support
