//! Reads the API classes (`interface*.html`) of the two script API docs folders.

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;

use scraper::{ElementRef, Html, Selector};
use serde::Serialize;

use super::markdown;
use super::{DOCS_URL, parent_row, read_html, selector};
use crate::{Error, Result};

/// One folder of script API docs.
pub struct ApiFolder {
    pub source: ApiSource,
    pub html_dir: PathBuf,

    /// The folder name on the wiki server, for example `EnfusionScriptAPIPublic`.
    pub public_name: &'static str,
}

/// Which script API a class belongs to. It is the `source` field in the JSON.
#[derive(Clone, Copy, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiSource {
    /// The engine API, in `EnfusionScriptAPI`.
    Enfusion,

    /// The game API, in `ArmaReforgerScriptAPIPublic`.
    Arma,
}

/// One class. Empty fields are left out of the JSON.
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Class {
    pub name: String,
    pub source: ApiSource,

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
/// Parents and children come from the class tree in `hierarchy.html`. A parent from the other
/// docs folder has no page here, so the tree gives only its name.
pub fn read(folder: &ApiFolder) -> Result<Vec<Class>> {
    let files = class_files(&folder.html_dir)?;
    let tree = class_tree(&folder.html_dir)?;
    let selectors = Selectors::new()?;
    let threads = thread::available_parallelism().map_or(4, usize::from);
    let chunk_size = files.len().div_ceil(threads).max(1);
    let mut classes = thread::scope(|scope| {
        let workers: Vec<_> = files
            .chunks(chunk_size)
            .map(|chunk| {
                scope.spawn(|| {
                    chunk
                        .iter()
                        .map(|file| read_class(folder, &selectors, file))
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
            .and_then(|parent| match parent {
                TreeClass::File(file) => name_of(file),
                TreeClass::Name(name) => Some(name.clone()),
            })
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

impl ApiFolder {
    /// The engine API folder under the Workbench `docs` folder.
    pub fn enfusion(docs: &Path) -> Self {
        Self {
            source: ApiSource::Enfusion,
            html_dir: docs.join("EnfusionScriptAPI/html"),
            public_name: "EnfusionScriptAPIPublic",
        }
    }

    /// The game API folder under the Workbench `docs` folder.
    pub fn arma(docs: &Path) -> Self {
        Self {
            source: ApiSource::Arma,
            html_dir: docs.join("ArmaReforgerScriptAPIPublic/html"),
            public_name: "ArmaReforgerScriptAPIPublic",
        }
    }

    /// The public URL of a file in this folder.
    pub fn url(&self, file: &str) -> String {
        format!("{DOCS_URL}{}/{file}", self.public_name)
    }
}

// region:    --- Support

/// The class pages: `interface<Name>.html`, without the `-members.html` lists. Basic types such as
/// `string` and `array` have lowercase names and are classes too.
fn class_files(html_dir: &Path) -> Result<Vec<String>> {
    let entries = fs::read_dir(html_dir).map_err(|source| Error::Read {
        path: html_dir.to_owned(),
        source,
    })?;
    let mut files = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| Error::Read {
            path: html_dir.to_owned(),
            source,
        })?;
        if let Ok(name) = entry.file_name().into_string()
            && name
                .strip_prefix("interface")
                .is_some_and(|rest| rest.starts_with(|c: char| c.is_ascii_alphabetic()))
            && Path::new(&name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("html"))
            && !name.ends_with("-members.html")
        {
            files.push(name);
        }
    }
    files.sort();
    Ok(files)
}

/// The class tree: each class's parent and children. Classes with a page are named by file name.
struct ClassTree {
    parent: HashMap<String, TreeClass>,
    children: BTreeMap<String, Vec<String>>,
}

/// A class in `hierarchy.html`. A class with a page in this folder is a link to its file. A class
/// from the other docs folder, for example `ScriptComponent` in the game docs, is only a bold name.
#[derive(Clone)]
enum TreeClass {
    File(String),
    Name(String),
}

fn class_tree(html_dir: &Path) -> Result<ClassTree> {
    let document = read_html(&html_dir.join("hierarchy.html"))?;
    let link = selector("td.entry a.el")?;
    let bold = selector("td.entry b")?;
    let mut by_row: HashMap<String, TreeClass> = HashMap::new();
    let mut files = Vec::new();
    for row in document.select(&selector("tr[id^=\"row_\"]")?) {
        let id = row.value().attr("id").unwrap_or_default().to_owned();
        let file = row
            .select(&link)
            .next()
            .and_then(|anchor| anchor.value().attr("href"))
            .map(|href| href.split('#').next().unwrap_or(href).to_owned());
        if let Some(file) = file {
            by_row.insert(id.clone(), TreeClass::File(file.clone()));
            files.push((id, file));
        } else if let Some(name) = row.select(&bold).next().map(markdown::text) {
            by_row.insert(id, TreeClass::Name(name));
        }
    }
    let mut tree = ClassTree {
        parent: HashMap::new(),
        children: BTreeMap::new(),
    };
    for (id, file) in files {
        let Some(parent) = parent_row(&id).and_then(|parent| by_row.get(&parent)) else {
            continue;
        };
        if let TreeClass::File(parent) = parent {
            let children = tree.children.entry(parent.clone()).or_default();
            if !children.contains(&file) {
                children.push(file.clone());
            }
        }
        tree.parent.insert(file, parent.clone());
    }
    Ok(tree)
}

fn read_class(folder: &ApiFolder, selectors: &Selectors, file: &str) -> Result<(String, Class)> {
    let document = read_html(&folder.html_dir.join(file))?;
    // The title holds the group links too, so only its own text is the name.
    let name: String = document
        .select(&selectors.title)
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
        .select(&selectors.brief)
        .next()
        .map_or_default(markdown::inline)
        .trim_end_matches("More...")
        .trim()
        .to_owned();
    let description = document
        .select(&selectors.description)
        .next()
        .map_or_default(|block| markdown::blocks(block, 2));
    // The description often starts with the brief again.
    let description = match description.strip_prefix(&brief) {
        Some(rest) if !brief.is_empty() && (rest.is_empty() || rest.starts_with('\n')) => {
            rest.trim_start().to_owned()
        }
        _ => description,
    };
    let group = document
        .select(&selectors.group)
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
        source_file: source_file(&document, selectors),
        methods: Vec::new(),
        protected_methods: Vec::new(),
        static_methods: Vec::new(),
        protected_static_methods: Vec::new(),
        properties: Vec::new(),
        protected_properties: Vec::new(),
        enums: Vec::new(),
        docs_url: folder.url(file),
    };
    add_members(&document, selectors, &mut class);
    Ok((file.to_owned(), class))
}

/// The last item of the list at the end of the page: the file that the class comes from.
fn source_file(document: &Html, selectors: &Selectors) -> String {
    document
        .select(&selectors.source_file)
        .next_back()
        .map_or_default(markdown::text)
}

/// Reads the member tables at the top of the page. Rows that a class inherits are left out:
/// the parent class has them.
fn add_members(document: &Html, selectors: &Selectors, class: &mut Class) {
    let docs = member_docs(document, selectors);
    let mut section = String::new();
    for row in document.select(&selectors.member_row) {
        let classes = row.value().attr("class").unwrap_or_default();
        if classes == "heading" {
            if let Some(anchor) = row.select(&selectors.anchor).next() {
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
        let (Some(left_cell), Some(right_cell)) = (
            row.select(&selectors.item_left).next(),
            row.select(&selectors.item_right).next(),
        ) else {
            continue;
        };
        let left_text = markdown::text(left_cell);
        let right_text = markdown::text(right_cell);
        let name = right_cell
            .select(&selectors.link)
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
}

/// The documentation of each member, by the hash in its row class. The long text below the tables
/// wins over the one-line text in the table.
fn member_docs(document: &Html, selectors: &Selectors) -> HashMap<String, String> {
    let mut docs = HashMap::new();
    for row in document.select(&selectors.member_row) {
        let classes = row.value().attr("class").unwrap_or_default();
        if let Some(hash) = classes.strip_prefix("memdesc:")
            && let Some(cell) = row.select(&selectors.item_description).next()
        {
            let text = markdown::inline(cell);
            if !text.is_empty() {
                docs.insert(hash.to_owned(), text);
            }
        }
    }
    for item in document.select(&selectors.member_item) {
        let anchor = item
            .prev_siblings()
            .filter_map(ElementRef::wrap)
            .find(|sibling| sibling.value().name() == "a" && sibling.value().attr("id").is_some());
        let Some(id) = anchor.and_then(|anchor| anchor.value().attr("id")) else {
            continue;
        };
        let text = item
            .select(&selectors.member_doc)
            .next()
            .map_or_default(|doc| markdown::blocks(doc, 3));
        if !text.is_empty() {
            docs.insert(id.to_owned(), text);
        }
    }
    docs
}

/// The CSS selectors of a class page, made once for all pages.
struct Selectors {
    title: Selector,
    brief: Selector,
    description: Selector,
    group: Selector,

    /// Doxygen ends a class page with "generated from the following file:" and a list with the file.
    source_file: Selector,
    member_row: Selector,
    anchor: Selector,
    item_left: Selector,
    item_right: Selector,
    item_description: Selector,
    link: Selector,
    member_item: Selector,
    member_doc: Selector,
}

impl Selectors {
    fn new() -> Result<Self> {
        Ok(Self {
            title: selector("div.headertitle div.title")?,
            brief: selector("div.contents > p")?,
            description: selector("div.contents > div.textblock")?,
            group: selector("div.ingroups a.el")?,
            source_file: selector("div.contents > ul > li")?,
            member_row: selector("table.memberdecls tr")?,
            anchor: selector("a[id]")?,
            item_left: selector("td.memItemLeft")?,
            item_right: selector("td.memItemRight")?,
            item_description: selector("td.mdescRight")?,
            link: selector("a.el")?,
            member_item: selector("div.memitem")?,
            member_doc: selector("div.memdoc")?,
        })
    }
}

// endregion: --- Support

// region:    --- Tests

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;

    #[test]
    fn names_a_parent_from_the_other_docs_folder_and_drops_a_repeated_brief() {
        // -- Setup & Fixtures
        let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "tests/.out/names_a_parent_from_the_other_docs_folder_and_drops_a_repeated_brief",
        );
        assert!(dir.to_string_lossy().contains("tests/.out"));
        let _ = fs::remove_dir_all(&dir);
        let folder = ApiFolder::arma(&dir);
        fs::create_dir_all(&folder.html_dir).unwrap();
        // As in the game docs: `ScriptComponent` is an engine class, so it is bold, not a link.
        fs::write(
            folder.html_dir.join("hierarchy.html"),
            "<table>\
             <tr id=\"row_0_\"><td class=\"entry\"><b>ScriptComponent</b></td></tr>\
             <tr id=\"row_0_0_\"><td class=\"entry\"><a class=\"el\" href=\"interfaceSCR_A.html\">SCR_A</a></td></tr>\
             <tr id=\"row_0_0_0_\"><td class=\"entry\"><a class=\"el\" href=\"interfaceSCR_B.html\">SCR_B</a></td></tr>\
             </table>",
        )
        .unwrap();
        for (name, brief, description) in [
            (
                "SCR_A",
                "Gives items.",
                "<p>Gives items.</p><p>Only on the server.</p>",
            ),
            ("SCR_B", "Shows items.", "<p>Shows items.</p>"),
        ] {
            fs::write(
                folder.html_dir.join(format!("interface{name}.html")),
                format!(
                    "<div class=\"headertitle\"><div class=\"title\">{name} Interface Reference</div></div>\
                     <div class=\"contents\"><p>{brief} <a href=\"#details\">More...</a></p>\
                     <div class=\"textblock\">{description}</div>\
                     <ul><li>Game/{name}.c</li></ul></div>"
                ),
            )
            .unwrap();
        }

        // -- Exec
        let classes = read(&folder).unwrap();

        // -- Check
        let classes: Vec<Value> = classes
            .iter()
            .map(|class| serde_json::to_value(class).unwrap())
            .collect();
        let url = "https://community.bistudio.com/wikidata/external-data/arma-reforger/ArmaReforgerScriptAPIPublic/";
        assert_eq!(
            classes,
            [
                json!({
                    "name": "SCR_A",
                    "source": "arma",
                    "brief": "Gives items.",
                    "description": "Only on the server.",
                    "parents": ["ScriptComponent"],
                    "children": ["SCR_B"],
                    "sourceFile": "Game/SCR_A.c",
                    "docsUrl": format!("{url}interfaceSCR_A.html"),
                }),
                json!({
                    "name": "SCR_B",
                    "source": "arma",
                    "brief": "Shows items.",
                    "parents": ["SCR_A"],
                    "sourceFile": "Game/SCR_B.c",
                    "docsUrl": format!("{url}interfaceSCR_B.html"),
                }),
            ]
        );
    }
}

// endregion: --- Tests
