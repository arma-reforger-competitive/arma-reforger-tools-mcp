//! The list of every file in the .pak archives of the game and of the downloaded Workshop mods.

use std::fs::{self, File};
use std::path::{Path, PathBuf};

use tokio_util::sync::CancellationToken;

use super::pak::list_files;
use super::settings::{PROFILE_SETTING, Settings, host_path};
use super::{Error, Result};

/// The name of the first source: the game itself.
pub(super) const GAME: &str = "game";

/// Every Game file, with the names of the sources and notes about what was not searched.
pub(super) struct Index {
    pub(super) files: Vec<GameFile>,

    /// `GAME` first, then the name of each mod.
    pub(super) sources: Vec<String>,

    /// Each line says what the search did not cover and how to fix it.
    pub(super) notes: Vec<String>,
}

/// One file in a .pak archive.
pub(super) struct GameFile {
    pub(super) path: String,

    /// The position of its source in `Index::sources`.
    pub(super) source: usize,
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
            setting: raw_game.to_owned(),
            folder: game_folder,
        });
    }
    let mut index = Index {
        files: Vec::new(),
        sources: vec![GAME.to_owned()],
        notes: Vec::new(),
    };
    index.add(0, &game_paks, cancel)?;
    let Some(raw_profile) = settings.profile.as_deref() else {
        index.notes.push(format!(
            "{PROFILE_SETTING} is not set, so the search covered the game only. To also search the \
             Workshop mods that Workbench downloaded, set it to the Workbench profile folder, for \
             example \"C:\\Users\\<you>\\Documents\\My Games\\ArmaReforgerWorkbench\"."
        ));
        return Ok(index);
    };
    let mods_folder = host_path(raw_profile).join("addons");
    let Ok(entries) = fs::read_dir(&mods_folder) else {
        index.notes.push(format!(
            "{PROFILE_SETTING} is \"{raw_profile}\", but {} is not a folder, so the search covered \
             the game only. Set it to the Workbench profile folder, the folder that holds addons.",
            mods_folder.display()
        ));
        return Ok(index);
    };
    let mut mod_folders: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|folder| folder.is_dir())
        .collect();
    mod_folders.sort();
    for folder in mod_folders {
        let mut paks = Vec::new();
        collect_paks(&folder, 1, &mut paks);
        if paks.is_empty() {
            continue;
        }
        index.sources.push(mod_name(&folder));
        index.add(index.sources.len() - 1, &paks, cancel)?;
    }
    Ok(index)
}

impl Index {
    /// Adds the files of `paks` from the source at `source`.
    fn add(&mut self, source: usize, paks: &[PathBuf], cancel: &CancellationToken) -> Result<()> {
        for pak in paks {
            if cancel.is_cancelled() {
                return Err(Error::Cancelled);
            }
            match read_pak(pak) {
                Ok(paths) => self
                    .files
                    .extend(paths.into_iter().map(|path| GameFile { path, source })),
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

fn read_pak(pak: &Path) -> Result<Vec<String>> {
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

/// The name of a mod from its folder `<Name>_<ID>`, where the ID has 16 hex digits.
fn mod_name(folder: &Path) -> String {
    let folder = folder.file_name().unwrap_or_default().to_string_lossy();
    match folder.rsplit_once('_') {
        Some((name, id))
            if !name.is_empty() && id.len() == 16 && id.chars().all(|c| c.is_ascii_hexdigit()) =>
        {
            name.to_owned()
        }
        _ => folder.into_owned(),
    }
}

// endregion: --- Support
