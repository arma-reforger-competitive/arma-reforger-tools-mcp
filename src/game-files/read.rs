//! The `game_read` Tool: opens one Game file by its path, with the closest real paths for a
//! wrong path.
//!
//! Capital letters in the path do not matter, a file in a downloaded mod opens too, and a binary
//! file is found by its content.

use std::fmt::Write as _;
use std::path::Path;

use rmcp::model::{CallToolResult, ContentBlock};
use schemars::JsonSchema;
use serde::Deserialize;

use super::Error;
use super::index::{GameFile, Index};

/// How many real paths a wrong path suggests.
const SUGGESTIONS: usize = 5;

/// How many file names the first, fast step of the suggestion keeps for the second step.
const NAME_CANDIDATES: usize = 50;

/// How many first bytes of a file are checked for a zero byte, the sign of a binary file.
const BINARY_CHECK_BYTES: usize = 8000;

#[derive(Deserialize, JsonSchema)]
pub(super) struct ReadInput {
    /// The path of the Game file, as `game_find` gives it, for example
    /// `scripts/Game/Weapon/SCR_WeaponInfo.c`. Capital letters do not matter.
    #[schemars(length(min = 1))]
    path: String,

    /// Where to open the file when the same path is in the game and in a mod: `game`, or the
    /// name of a downloaded mod, for example `tvtLobby`. The default is the game first, then the
    /// mods.
    source: Option<String>,
}

/// Answers one `game_read` call. It reads the .pak file, so it runs outside the server thread.
pub(super) fn read(index: &Index, input: &ReadInput) -> CallToolResult {
    let found = match lookup(index, input) {
        Ok(found) => found,
        Err(answer) => return answer,
    };
    let file = found.file;
    match file.read() {
        Ok(content) => answer(index, &found, &content),
        Err(read_error) => {
            let advice = match read_error {
                Error::FileTooLarge { .. } => "",
                _ => {
                    " The file may be damaged: verify the game files in Steam, or download the \
                     mod again."
                }
            };
            error(&format!(
                "Could not open {} ({}) from {}: {read_error}.{advice}",
                file.path,
                index.source_of(file),
                file.pak.display()
            ))
        }
    }
}

// region:    --- Support

/// The file that a `game_read` call opens.
struct Found<'a> {
    file: &'a GameFile,

    /// The other sources that have the same path, for example "mod tvtLobby".
    other_sources: Vec<String>,

    /// The other .pak files of the same source that have the same path.
    other_paks: Vec<&'a Path>,
}

/// The file that `input` names in `index`. When it finds none, the answer says why.
fn lookup<'a>(index: &'a Index, input: &ReadInput) -> Result<Found<'a>, CallToolResult> {
    let (path, source_in_path) = clean_path(&input.path);
    let path = path.as_str();
    if path.is_empty() {
        let text = "The path is empty. Give the path of a Game file, for example \
                    \"scripts/Game/Weapon/SCR_WeaponInfo.c\". To find a path, call game_find.";
        return Err(error(text));
    }
    let mut matches: Vec<&GameFile> = index
        .files
        .iter()
        .filter(|file| file.path.eq_ignore_ascii_case(path))
        .collect();
    if matches.is_empty() {
        return Err(error(&not_found(index, path)));
    }
    // The game comes first, then the mods. The path with the same capital letters comes first.
    matches.sort_by_key(|file| (file.source, file.path != path));
    let wanted = input
        .source
        .as_deref()
        .map(str::trim)
        .filter(|source| !source.is_empty())
        .or(source_in_path);
    let file = match wanted {
        None => matches.first().copied(),
        Some(wanted) => matches
            .iter()
            .copied()
            .find(|file| is_source(index, file, wanted)),
    };
    let Some(file) = file else {
        let sources = sources_of(index, &matches).join(", ");
        let text = format!(
            "\"{path}\" is not in \"{}\". It is in: {sources}. Give one of them as source, or \
             leave source out to open the first.",
            wanted.unwrap_or_default()
        );
        return Err(error(&text));
    };
    let (same_source, others): (Vec<&GameFile>, Vec<&GameFile>) = matches
        .into_iter()
        .partition(|other| other.source == file.source);
    Ok(Found {
        file,
        other_sources: sources_of(index, &others),
        other_paks: same_source
            .into_iter()
            .filter(|other| other.pak != file.pak)
            .map(|other| &*other.pak)
            .collect(),
    })
}

/// The answer with the text of the found file, which has `content`.
fn answer(index: &Index, found: &Found, content: &[u8]) -> CallToolResult {
    let file = found.file;
    let check = content.get(..BINARY_CHECK_BYTES).unwrap_or(content);
    if check.contains(&0) {
        let text = format!(
            "{} ({}) is a binary file of {} bytes, not text. game_read opens text files, for \
             example scripts (.c), Prefabs (.et), configs (.conf), and layouts (.layout).",
            file.path,
            index.source_of(file),
            content.len()
        );
        return error(&text);
    }
    let text = String::from_utf8_lossy(content);
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    let mut answer = format!(
        "{} ({}, {} bytes)\n",
        file.path,
        index.source_of(file),
        content.len()
    );
    if !found.other_paks.is_empty() {
        let others: Vec<String> = found
            .other_paks
            .iter()
            .map(|pak| pak.display().to_string())
            .collect();
        let _ = writeln!(
            answer,
            "This text comes from {}. The {} has the same path also in: {}.",
            file.pak.display(),
            index.source_of(file),
            others.join(", ")
        );
    }
    if !found.other_sources.is_empty() {
        let _ = writeln!(
            answer,
            "The same path is also in: {}. To open another one, give it as source.",
            found.other_sources.join(", ")
        );
    }
    let _ = write!(answer, "\n{text}");
    CallToolResult::success(vec![ContentBlock::text(answer)])
}

fn error(text: &str) -> CallToolResult {
    CallToolResult::error(vec![ContentBlock::text(text)])
}

/// The path without the parts that Claude often copies with it: the `(game)` or `(mod tvtLobby)`
/// after a path from `game_find`, and the `{GUID}` before a path from a Prefab. Back slashes
/// become slashes. Also gives the source from the `(...)` part.
fn clean_path(raw: &str) -> (String, Option<&str>) {
    let mut path = raw.trim();
    let mut source = None;
    if let Some((before, after)) = path.rsplit_once(" (")
        && let Some(inside) = after.strip_suffix(')')
        && (inside.eq_ignore_ascii_case("game") || inside.starts_with("mod "))
    {
        path = before.trim_end();
        source = Some(inside);
    }
    if let Some(rest) = path.strip_prefix('{')
        && let Some((guid, after)) = rest.split_once('}')
        && guid.len() == 16
        && guid.chars().all(|c| c.is_ascii_hexdigit())
    {
        path = after;
    }
    let path = path.replace('\\', "/");
    (path.trim_matches('/').to_owned(), source)
}

/// Whether `file` comes from the source that the user named `wanted`: "game", "tvtLobby", or
/// "mod tvtLobby". Capital letters do not matter.
fn is_source(index: &Index, file: &GameFile, wanted: &str) -> bool {
    let name = index.source_of(file);
    name.eq_ignore_ascii_case(wanted)
        || name
            .strip_prefix("mod ")
            .is_some_and(|name| name.eq_ignore_ascii_case(wanted))
}

/// The source names of `files`, each once, in the order of `files`.
fn sources_of(index: &Index, files: &[&GameFile]) -> Vec<String> {
    let mut sources: Vec<String> = Vec::new();
    for file in files {
        let source = index.source_of(file);
        if !sources.contains(&source) {
            sources.push(source);
        }
    }
    sources
}

/// The answer for a path that no Game file has, with the closest real paths.
fn not_found(index: &Index, path: &str) -> String {
    let wanted = path.to_lowercase();
    let wanted_name = wanted.rsplit('/').next().unwrap_or(&wanted);
    // A class name such as SCR_CharacterCameraHandlerComponent is often in a file without the
    // SCR_ prefix, such as CharacterCameraHandlerComponent.c.
    let names: Vec<&str> = [Some(wanted_name), wanted_name.strip_prefix("scr_")]
        .into_iter()
        .flatten()
        .collect();
    // First a fast step on the file names only, then the full paths of the best names.
    let mut by_name: Vec<(f64, &GameFile)> = index
        .files
        .iter()
        .map(|file| {
            let name = file
                .path
                .rsplit('/')
                .next()
                .unwrap_or(&file.path)
                .to_lowercase();
            let score = names
                .iter()
                .map(|wanted| strsim::jaro_winkler(&name, wanted))
                .fold(0.0, f64::max);
            (score, file)
        })
        .collect();
    by_name.sort_by(|a, b| b.0.total_cmp(&a.0));
    by_name.truncate(NAME_CANDIDATES);
    let mut best: Vec<(f64, &GameFile)> = by_name
        .into_iter()
        .map(|(name_score, file)| {
            let path_score = strsim::jaro_winkler(&file.path.to_lowercase(), &wanted);
            (2.0 * name_score + path_score, file)
        })
        .collect();
    best.sort_by(|(a_score, a), (b_score, b)| {
        b_score
            .total_cmp(a_score)
            .then(a.source.cmp(&b.source))
            .then(a.path.cmp(&b.path))
    });
    let mut text = format!(
        "No Game file has the path \"{path}\". Searched {} files in {}.",
        index.files.len(),
        index.searched()
    );
    if !best.is_empty() {
        text.push_str(" The closest real paths:\n");
        for (_, file) in best.iter().take(SUGGESTIONS) {
            let _ = writeln!(text, "- {} ({})", file.path, index.source_of(file));
        }
        text.push_str("Call game_read with one of them, or game_find with a part of the name.");
    }
    for note in &index.notes {
        let _ = write!(text, "\n\n{note}");
    }
    text
}

// endregion: --- Support
