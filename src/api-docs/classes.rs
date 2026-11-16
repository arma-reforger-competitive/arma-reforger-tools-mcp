//! The script API classes of `classes.jsonl`, built into the program.

use std::collections::HashMap;

use serde::Deserialize;

use super::{Error, Result};

/// One class per line, written by `cargo xtask import`.
const CLASSES_JSONL: &str = include_str!("classes.jsonl");

// region:    --- Types

pub(super) struct Classes {
    classes: Vec<Class>,

    /// For each class name in lowercase: the positions of the classes with that name. The engine
    /// and the game docs both have a few classes, for example `RplComponent`.
    by_name: HashMap<String, Vec<usize>>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Class {
    pub(super) name: String,
    pub(super) source: Source,

    #[serde(default)]
    pub(super) brief: String,

    #[serde(default)]
    pub(super) description: String,

    /// The class it inherits from. The docs give at most one.
    #[serde(default)]
    pub(super) parents: Vec<String>,

    #[serde(default)]
    pub(super) children: Vec<String>,

    #[serde(default)]
    pub(super) group: String,
    pub(super) source_file: String,

    #[serde(default)]
    pub(super) methods: Vec<Method>,

    #[serde(default)]
    pub(super) protected_methods: Vec<Method>,

    #[serde(default)]
    pub(super) static_methods: Vec<Method>,

    #[serde(default)]
    pub(super) protected_static_methods: Vec<Method>,

    #[serde(default)]
    pub(super) properties: Vec<Property>,

    #[serde(default)]
    pub(super) protected_properties: Vec<Property>,
    pub(super) docs_url: String,
}

/// Engine classes come from the Enfusion script API docs, game classes from the Arma Reforger docs.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(super) enum Source {
    Enfusion,
    Arma,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Method {
    pub(super) name: String,
    pub(super) signature: String,

    #[serde(default)]
    pub(super) description: String,
}

#[derive(Deserialize)]
pub(super) struct Property {
    pub(super) name: String,

    #[serde(rename = "type", default)]
    pub(super) kind: String,

    #[serde(default)]
    pub(super) description: String,
}

// endregion: --- Types

impl Classes {
    /// Reads the classes that are built into the program. It fails when a line is not a valid class.
    pub(super) fn load() -> Result<Self> {
        let classes = CLASSES_JSONL
            .lines()
            .enumerate()
            .map(|(index, line)| {
                serde_json::from_str(line).map_err(|error| Error::BadClassLine {
                    line: index + 1,
                    error,
                })
            })
            .collect::<Result<Vec<Class>>>()?;
        let mut by_name: HashMap<String, Vec<usize>> = HashMap::new();
        for (index, class) in classes.iter().enumerate() {
            by_name
                .entry(class.name.to_lowercase())
                .or_default()
                .push(index);
        }
        Ok(Self { classes, by_name })
    }

    pub(super) fn all(&self) -> &[Class] {
        &self.classes
    }

    /// Gives the first class with this name. Capital letters do not matter.
    pub(super) fn find(&self, name: &str) -> Option<&Class> {
        let index = self.by_name.get(&name.to_lowercase())?.first()?;
        self.classes.get(*index)
    }

    /// Gives the names of the classes that `class` inherits from, the parent first. A parent that
    /// the docs do not have ends the chain.
    pub(super) fn ancestors<'a>(&'a self, class: &'a Class) -> Vec<&'a str> {
        let mut names: Vec<&str> = Vec::new();
        let mut current = class;
        while let Some(parent) = current.parents.first() {
            if parent.eq_ignore_ascii_case(&class.name)
                || names.iter().any(|name| name.eq_ignore_ascii_case(parent))
            {
                break;
            }
            names.push(parent);
            let Some(next) = self.find(parent) else {
                break;
            };
            current = next;
        }
        names
    }
}

/// Accessors
impl Class {
    /// The methods in groups, each with its heading in a class page.
    pub(super) fn method_groups(&self) -> [(&'static str, &[Method]); 4] {
        [
            ("Methods", &self.methods),
            ("Protected methods", &self.protected_methods),
            ("Static methods", &self.static_methods),
            ("Protected static methods", &self.protected_static_methods),
        ]
    }

    /// The properties in groups, each with its heading in a class page.
    pub(super) fn property_groups(&self) -> [(&'static str, &[Property]); 2] {
        [
            ("Properties", &self.properties),
            ("Protected properties", &self.protected_properties),
        ]
    }
}

/// Accessors
impl Source {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Enfusion => "Enfusion engine",
            Self::Arma => "Arma Reforger",
        }
    }
}
