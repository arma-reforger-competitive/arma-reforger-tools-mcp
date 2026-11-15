//! Reads the API classes (`interface*.html`) of the two script API docs folders.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use scraper::{ElementRef, Html};
use serde::Serialize;

use super::markdown;
use super::{read_html, selector};
use crate::{Error, Result};

const DOCS_URL: &str = "https://community.bistudio.com/wikidata/external-data/arma-reforger/";

/// One folder of script API docs.
pub struct ApiFolder {
    /// `enfusion` for the engine API, `arma` for the game API.
    pub source: &'static str,
    pub html_dir: PathBuf,

    /// The folder name on the wiki server, for example `EnfusionScriptAPIPublic`.
    pub public_name: &'static str,
}

/// One class. Empty fields are left out of the JSON.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub name: String,
    pub source: &'static str,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub brief: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub group: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub source_file: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub methods: Vec<Method>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub protected_methods: Vec<Method>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub static_methods: Vec<Method>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub protected_static_methods: Vec<Method>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub properties: Vec<Property>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub protected_properties: Vec<Property>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub enums: Vec<Enum>,
    pub docs_url: String,
}

/// A method. Its parameters are in `signature`, so they have no field of their own.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Method {
    pub name: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub return_type: String,
    pub signature: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

#[derive(Serialize)]
pub struct Property {
    pub name: String,

    #[serde(rename = "type", skip_serializing_if = "String::is_empty")]
    pub kind: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

/// An enum declared in a class, with its values as Doxygen writes them: `Name { A, B = 1 }`.
#[derive(Serialize)]
pub struct Enum {
    pub name: String,
    pub definition: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub description: String,
}

/// Reads every class of `folder`, sorted by name.
/// Parents and children come from the class tree in `hierarchy.html`.
pub fn read(folder: &ApiFolder) -> Result<Vec<Class>> {
    let files = class_files(&folder.html_dir)?;
    let tree = class_tree(&folder.html_dir)?;
    let threads = thread::available_parallelism().map_or(4, usize::from);
    let chunk_size = files.len().div_ceil(threads).max(1);
    let mut classes = thread::scope(|scope| {
        let workers: Vec<_> = files
            .chunks(chunk_size)
            .map(|chunk| {
                scope.spawn(|| {
                    chunk
                        .iter()
                        .map(|file| read_class(folder, file))
                        .collect::<Result<Vec<_>>>()
                })
            })
            .collect();
        let mut classes = Vec::new();
        for worker in workers {
            classes.extend(
                worker
                    .join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic))?,
            );
        }
        Ok::<_, Error>(classes)
    })?;
    let names: HashMap<String, String> = classes
        .iter()
        .map(|(file, class)| (file.clone(), class.name.clone()))
        .collect();
    for (file, class) in &mut classes {
        let name_of = |file: &String| names.get(file).cloned();
        class.parents = tree
            .parent
            .get(file)
            .and_then(name_of)
            .into_iter()
            .collect();
        class.children = tree
            .children
            .get(file)
            .into_iter()
            .flatten()
            .filter_map(name_of)
            .collect();
    }
    let mut classes: Vec<Class> = classes.into_iter().map(|(_, class)| class).collect();
    classes.sort_by(|a, b| {
        a.name
            .cmp(&b.name)
            .then_with(|| a.docs_url.cmp(&b.docs_url))
    });
    Ok(classes)
}

// region:    --- Support

/// The class pages: `interface<Name>.html`, without the `-members.html` lists. Basic types such as
/// `string` and `array` have lowercase names and are classes too.
fn class_files(html_dir: &Path) -> Result<Vec<String>> {
    let entries = fs::read_dir(html_dir).map_err(|source| Error::Read {
        path: html_dir.to_owned(),
        source,
    })?;
    let mut files: Vec<String> = entries
        .filter_map(|entry| entry.ok()?.file_name().into_string().ok())
        .filter(|name| {
            name.strip_prefix("interface")
                .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_alphabetic()))
                && Path::new(name)
                    .extension()
                    .is_some_and(|ext| ext.eq_ignore_ascii_case("html"))
                && !name.ends_with("-members.html")
        })
        .collect();
    files.sort();
    Ok(files)
}

/// The class tree, by file name: each class's parent and children.
struct ClassTree {
    parent: HashMap<String, String>,
    children: BTreeMap<String, Vec<String>>,
}

fn class_tree(html_dir: &Path) -> Result<ClassTree> {
    let document = read_html(&html_dir.join("hierarchy.html"))?;
    let link = selector("td.entry a.el")?;
    let mut by_row: HashMap<String, String> = HashMap::new();
    let mut rows = Vec::new();
    for row in document.select(&selector("tr[id^=\"row_\"]")?) {
        let id = row.value().attr("id").unwrap_or_default().to_owned();
        let file = row
            .select(&link)
            .next()
            .and_then(|anchor| anchor.value().attr("href"))
            .map(|href| href.split('#').next().unwrap_or(href).to_owned());
        if let Some(file) = file {
            by_row.insert(id.clone(), file.clone());
            rows.push((id, file));
        }
    }
    let mut tree = ClassTree {
        parent: HashMap::new(),
        children: BTreeMap::new(),
    };
    for (id, file) in rows {
        let parent = id
            .strip_suffix('_')
            .and_then(|trimmed| trimmed.rsplit_once('_'))
            .map(|(parent, _)| format!("{parent}_"))
            .and_then(|parent| by_row.get(&parent));
        if let Some(parent) = parent {
            tree.parent.insert(file.clone(), parent.clone());
            let children = tree.children.entry(parent.clone()).or_default();
            if !children.contains(&file) {
                children.push(file);
            }
        }
    }
    Ok(tree)
}

fn read_class(folder: &ApiFolder, file: &str) -> Result<(String, Class)> {
    let document = read_html(&folder.html_dir.join(file))?;
    // The title holds the group links too, so only its own text is the name.
    let name: String = document
        .select(&selector("div.headertitle div.title")?)
        .next()
        .map_or_default(|title| {
            title
                .children()
                .filter_map(|child| child.value().as_text().map(|text| text.to_string()))
                .collect()
        });
    let name = name
        .trim()
        .trim_end_matches(" Interface Reference")
        .trim_end_matches(" Class Reference")
        .to_owned();
    let brief = document
        .select(&selector("div.contents > p")?)
        .next()
        .map_or_default(markdown::inline)
        .trim_end_matches("More...")
        .trim()
        .to_owned();
    let description = document
        .select(&selector("div.contents > div.textblock")?)
        .next()
        .map_or_default(|block| markdown::blocks(block, 2));
    let description = if description == brief {
        String::new()
    } else {
        description
    };
    let group = document
        .select(&selector("div.ingroups a.el")?)
        .next()
        .map_or_default(markdown::text);
    let mut class = Class {
        name,
        source: folder.source,
        brief,
        description,
        parents: Vec::new(),
        children: Vec::new(),
        group,
        source_file: source_file(&document)?,
        methods: Vec::new(),
        protected_methods: Vec::new(),
        static_methods: Vec::new(),
        protected_static_methods: Vec::new(),
        properties: Vec::new(),
        protected_properties: Vec::new(),
        enums: Vec::new(),
        docs_url: format!("{DOCS_URL}{}/{file}", folder.public_name),
    };
    add_members(&document, &mut class)?;
    Ok((file.to_owned(), class))
}

/// Doxygen ends a class page with "generated from the following file:" and a list with the file.
fn source_file(document: &Html) -> Result<String> {
    Ok(document
        .select(&selector("div.contents > ul > li")?)
        .next_back()
        .map_or_default(markdown::text))
}

/// Reads the member tables at the top of the page. Rows that a class inherits are left out:
/// the parent class has them.
fn add_members(document: &Html, class: &mut Class) -> Result<()> {
    let docs = member_docs(document)?;
    let left = selector("td.memItemLeft")?;
    let right = selector("td.memItemRight")?;
    let link = selector("a.el")?;
    let mut section = String::new();
    for row in document.select(&selector("table.memberdecls tr")?) {
        let classes = row.value().attr("class").unwrap_or_default();
        if classes == "heading" {
            if let Some(anchor) = row.select(&selector("a[id]")?).next() {
                anchor
                    .value()
                    .attr("id")
                    .unwrap_or_default()
                    .clone_into(&mut section);
            }
            continue;
        }
        let Some(hash) = classes.strip_prefix("memitem:") else {
            continue;
        };
        if hash.contains(' ') {
            continue;
        }
        let (Some(left_cell), Some(right_cell)) =
            (row.select(&left).next(), row.select(&right).next())
        else {
            continue;
        };
        let left_text = markdown::text(left_cell);
        let right_text = markdown::text(right_cell);
        let name = right_cell
            .select(&link)
            .next()
            .map_or_else(|| right_text.clone(), markdown::text);
        let description = docs.get(hash).cloned().unwrap_or_default();
        let method = || Method {
            name: name.clone(),
            return_type: left_text.clone(),
            signature: format!("{left_text} {right_text}").trim().to_owned(),
            description: description.clone(),
        };
        let property = || Property {
            name: name.clone(),
            kind: left_text.clone(),
            description: description.clone(),
        };
        match section.as_str() {
            "pub-methods" => class.methods.push(method()),
            "pro-methods" => class.protected_methods.push(method()),
            "pub-static-methods" => class.static_methods.push(method()),
            "pro-static-methods" => class.protected_static_methods.push(method()),
            "pub-attribs" | "pub-static-attribs" => class.properties.push(property()),
            "pro-attribs" | "pro-static-attribs" => class.protected_properties.push(property()),
            "pub-types" | "pro-types" => class.enums.push(Enum {
                name,
                definition: right_text,
                description,
            }),
            _ => {}
        }
    }
    Ok(())
}

/// The documentation of each member, by the hash in its row class. The long text below the tables
/// wins over the one-line text in the table.
fn member_docs(document: &Html) -> Result<HashMap<String, String>> {
    let mut docs = HashMap::new();
    let short = selector("td.mdescRight")?;
    for row in document.select(&selector("table.memberdecls tr")?) {
        let classes = row.value().attr("class").unwrap_or_default();
        if let Some(hash) = classes.strip_prefix("memdesc:")
            && let Some(cell) = row.select(&short).next()
        {
            let text = markdown::inline(cell);
            if !text.is_empty() {
                docs.insert(hash.to_owned(), text);
            }
        }
    }
    let memdoc = selector("div.memdoc")?;
    for item in document.select(&selector("div.memitem")?) {
        let anchor = item
            .prev_siblings()
            .filter_map(ElementRef::wrap)
            .find(|sibling| sibling.value().name() == "a" && sibling.value().attr("id").is_some());
        let Some(id) = anchor.and_then(|anchor| anchor.value().attr("id")) else {
            continue;
        };
        let text = item
            .select(&memdoc)
            .next()
            .map_or_default(|doc| markdown::blocks(doc, 3));
        if !text.is_empty() {
            docs.insert(id.to_owned(), text);
        }
    }
    Ok(docs)
}

// endregion: --- Support
