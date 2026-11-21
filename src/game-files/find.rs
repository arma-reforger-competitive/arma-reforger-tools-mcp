//! The `game_find` Tool: finds Game files by a part of their name or path, or by a class name.

use std::fmt::Write as _;

use schemars::JsonSchema;
use serde::Deserialize;

use super::index::{GameFile, Index};
use crate::server::Answer;

const DEFAULT_LIMIT: usize = 20;
const MAX_LIMIT: usize = 100;

#[derive(Deserialize, JsonSchema)]
pub(super) struct FindInput {
    /// A part of a file name or a path, or a class name, for example `CharacterCameraHandler`,
    /// `SCR_PreGameGameModeStateComponent`, or `GameMode/SCR_`. Capital letters do not matter.
    #[schemars(length(min = 1))]
    query: String,

    /// How many paths to list, from 1 to 100, for example 50. The default is 20.
    #[schemars(range(min = 1, max = 100))]
    limit: Option<usize>,
}

/// Answers one `game_find` call.
pub(super) fn find(index: &Index, input: &FindInput) -> Answer {
    let shown = input.query.trim();
    let query = shown.replace('\\', "/").to_lowercase();
    if query.is_empty() {
        let text = "The query is empty. Give a part of a file name or a class name, for example \
                    \"CharacterCameraHandler\" or \"SCR_PreGameGameModeStateComponent\".";
        return Err(text.to_owned());
    }
    // A class name such as SCR_CharacterCameraHandlerComponent is often in a file without the
    // SCR_ prefix, such as CharacterCameraHandlerComponent.c.
    let names: Vec<&str> = [Some(query.as_str()), query.strip_prefix("scr_")]
        .into_iter()
        .flatten()
        .filter(|name| !name.is_empty())
        .collect();
    let mut hits: Vec<(Score, &GameFile)> = index
        .files
        .iter()
        .filter_map(|file| {
            let score = names.iter().filter_map(|name| score(file, name)).max()?;
            Some((score, file))
        })
        .collect();
    hits.sort_by(|(a_score, a), (b_score, b)| {
        b_score
            .cmp(a_score)
            .then(a.source.cmp(&b.source))
            .then(a.path.len().cmp(&b.path.len()))
            .then(a.path.cmp(&b.path))
    });
    let limit = input.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);
    let mut text = String::new();
    if hits.is_empty() {
        let _ = writeln!(
            text,
            "No Game file has \"{shown}\" in its name or path. Searched {} files in {}. Try a shorter \
             part of the name, for example one word of a class name.",
            index.files.len(),
            index.searched()
        );
    } else if hits.len() > limit {
        let _ = writeln!(
            text,
            "{} Game files match \"{shown}\". The first {limit} come below. Give a longer part of the \
             name to narrow the search, or a larger limit (at most {MAX_LIMIT}).",
            hits.len()
        );
    } else {
        let _ = writeln!(text, "{} Game files match \"{shown}\":", hits.len());
    }
    for (_, file) in hits.iter().take(limit) {
        let _ = writeln!(text, "- {} ({})", file.path, index.source_of(file));
    }
    for note in &index.notes {
        let _ = write!(text, "\n{note}\n");
    }
    Ok(text.trim_end().to_owned())
}

// region:    --- Support

/// How well a file matches. A larger score comes first.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Score {
    InPath,
    InName,
    StartOfName,
    FullName,
}

/// How well `file` matches `name`, which is in lowercase, or `None` when it does not.
fn score(file: &GameFile, name: &str) -> Option<Score> {
    let path = file.path.to_lowercase();
    let file_name = path.rsplit('/').next().unwrap_or(&path);
    let stem = file_name
        .rsplit_once('.')
        .map_or(file_name, |(stem, _)| stem);
    if stem == name || file_name == name {
        Some(Score::FullName)
    } else if file_name.starts_with(name) {
        Some(Score::StartOfName)
    } else if file_name.contains(name) {
        Some(Score::InName)
    } else if path.contains(name) {
        Some(Score::InPath)
    } else {
        None
    }
}

// endregion: --- Support
