//! Finds the folder of the mod project that is open in Workbench.
//!
//! The NET API function `GetLoadedProjects` gives only the IDs of the loaded projects, with the
//! game and the dependencies. Workbench keeps the `.gproj` path of every project that the user
//! made or added in its project list, `<documents>/profile/.projectList_*.conf`. The open project
//! is the listed project whose ID is loaded and that no other loaded listed project needs.

use std::fs;
use std::path::{Path, PathBuf};

use super::Result;
use super::error::NoProject;
use crate::host_path::host_path;

/// One project from the project list of Workbench.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct Project {
    /// The folder that holds the `.gproj` file.
    pub(super) folder: PathBuf,
    pub(super) id: String,
    pub(super) guid: String,
    pub(super) dependencies: Vec<String>,
}

/// The open project, from the Workbench folder in Documents and the IDs that
/// `GetLoadedProjects` gave.
pub(super) fn open_project(documents: &Path, loaded: &[String]) -> Result<Project> {
    let listed = listed_projects(documents)?;
    let mut candidates: Vec<Project> = listed
        .into_iter()
        .filter(|project| loaded.contains(&project.id))
        .collect();
    let needed: Vec<String> = candidates
        .iter()
        .flat_map(|project| project.dependencies.iter().cloned())
        .collect();
    candidates.retain(|project| !needed.contains(&project.guid));
    match candidates.len() {
        0 => Err(NoProject::NotListed {
            loaded: loaded.to_vec(),
        }
        .into()),
        1 => Ok(candidates.swap_remove(0)),
        _ => Err(NoProject::Many {
            folders: candidates
                .into_iter()
                .map(|project| project.folder)
                .collect(),
        }
        .into()),
    }
}

// region:    --- Support

/// Every project in the project lists of Workbench whose `.gproj` file can be read. A project
/// that was moved or deleted is skipped.
fn listed_projects(documents: &Path) -> Result<Vec<Project>> {
    let profile = documents.join("profile");
    let lists: Vec<PathBuf> = fs::read_dir(&profile)
        .into_iter()
        .flatten()
        .filter_map(|entry| Some(entry.ok()?.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| {
                    name.starts_with(".projectList")
                        && Path::new(name)
                            .extension()
                            .is_some_and(|ext| ext.eq_ignore_ascii_case("conf"))
                })
        })
        .collect();
    if lists.is_empty() {
        return Err(NoProject::NoList {
            folder: documents.to_owned(),
        }
        .into());
    }
    let mut projects = Vec::new();
    for list in lists {
        let Ok(text) = fs::read_to_string(&list) else {
            continue;
        };
        for gproj in quoted_values(&text, "FilePath") {
            let gproj = host_path(&gproj);
            let Ok(text) = fs::read_to_string(&gproj) else {
                continue;
            };
            let (Some(folder), Some(id)) = (gproj.parent(), quoted_values(&text, "ID").next())
            else {
                continue;
            };
            let project = Project {
                folder: folder.to_owned(),
                id,
                guid: quoted_values(&text, "GUID").next().unwrap_or_default(),
                dependencies: dependencies(&text),
            };
            if !projects.contains(&project) {
                projects.push(project);
            }
        }
    }
    Ok(projects)
}

/// The values of the lines `<key> "<value>"` in a Workbench config text.
fn quoted_values<'a>(text: &'a str, key: &'a str) -> impl Iterator<Item = String> + 'a {
    text.lines().filter_map(move |line| {
        let value = line
            .trim()
            .strip_prefix(key)?
            .strip_prefix(char::is_whitespace)?;
        Some(
            value
                .trim()
                .strip_prefix('"')?
                .strip_suffix('"')?
                .to_owned(),
        )
    })
}

/// The GUIDs in the `Dependencies { ... }` block of a `.gproj` text.
fn dependencies(text: &str) -> Vec<String> {
    let mut lines = text.lines().map(str::trim);
    if !lines.any(|line| line.starts_with("Dependencies") && line.ends_with('{')) {
        return Vec::new();
    }
    lines
        .take_while(|line| *line != "}")
        .filter_map(|line| Some(line.strip_prefix('"')?.strip_suffix('"')?.to_owned()))
        .collect()
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_the_project_list_that_workbench_wrote() {
        // -- Setup & Fixtures
        // The project list of Workbench 1.6 with one project.
        let list = "WBProjectList {\n Projects {\n  WBProjectListItem {\n   FilePath \"D:/ArmaReforgerWorkbench/BridgeTest/addon.gproj\"\n  }\n }\n}";

        // -- Exec & Check
        assert_eq!(
            quoted_values(list, "FilePath").collect::<Vec<_>>(),
            ["D:/ArmaReforgerWorkbench/BridgeTest/addon.gproj"]
        );
    }
}

// endregion: --- Tests
