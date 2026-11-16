//! Reads the pages of the `MediaWiki` export `src/wiki/source/export.xml`.

use std::fs;
use std::path::Path;

use quick_xml::Reader;
use quick_xml::escape::unescape;
use quick_xml::events::Event;

use crate::{Error, Result};

/// The wiki namespace of the Arma Reforger pages. Templates and other games' pages are in other namespaces.
const REFORGER_NAMESPACE: &str = "5050";

pub struct WikiPage {
    pub title: String,
    pub wikitext: String,
}

/// Gives the Arma Reforger pages of the export, in the order of the export.
pub fn reforger_pages(path: &Path) -> Result<Vec<WikiPage>> {
    let pages: Vec<WikiPage> = read(path)?
        .into_iter()
        .filter(|(namespace, _)| namespace == REFORGER_NAMESPACE)
        .map(|(_, page)| page)
        .collect();
    if pages.is_empty() {
        return Err(Error::NoWikiPages {
            path: path.to_owned(),
        });
    }
    Ok(pages)
}

/// Gives the titles of all Arma Reforger pages of the export, for the wiki's Special:Export page.
pub fn reforger_titles(path: &Path) -> Result<Vec<String>> {
    Ok(read(path)?
        .into_iter()
        .filter(|(namespace, _)| namespace == REFORGER_NAMESPACE)
        .map(|(_, page)| page.title)
        .collect())
}

// region:    --- Support

/// Reads each `<page>` as its namespace and its page.
fn read(path: &Path) -> Result<Vec<(String, WikiPage)>> {
    let xml = fs::read_to_string(path).map_err(|source| Error::Read {
        path: path.to_owned(),
        source,
    })?;
    let xml_error = |source| Error::Xml {
        path: path.to_owned(),
        source,
    };
    let mut reader = Reader::from_str(&xml);
    let mut pages = Vec::new();
    let (mut title, mut namespace, mut wikitext) = (String::new(), String::new(), String::new());
    loop {
        match reader.read_event().map_err(xml_error)? {
            Event::Start(tag) => {
                let field = match tag.local_name().as_ref() {
                    "title" => &mut title,
                    "ns" => &mut namespace,
                    "text" => &mut wikitext,
                    _ => continue,
                };
                let raw = reader.read_text(tag.name()).map_err(xml_error)?;
                *field = unescape(&raw)
                    .map_err(|source| Error::XmlEntity {
                        path: path.to_owned(),
                        source,
                    })?
                    .into_owned();
            }
            Event::End(tag) if tag.local_name().as_ref() == "page" => {
                let page = WikiPage {
                    title: std::mem::take(&mut title),
                    wikitext: std::mem::take(&mut wikitext),
                };
                pages.push((std::mem::take(&mut namespace), page));
            }
            Event::Eof => break,
            _ => {}
        }
    }
    Ok(pages)
}

// endregion: --- Support
