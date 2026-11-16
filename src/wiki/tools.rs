//! The Tools of the wiki Capability.

use std::sync::Arc;

use rmcp::handler::server::{
    common::schema_for_type, router::tool::ToolRouter, wrapper::Parameters,
};
use rmcp::model::Tool;

use super::Result;
use super::pages::Pages;
use super::read::{ReadInput, read};
use super::search::{Index, SearchInput, search};
use crate::server::read_only_labels;

/// Gives the wiki Tools to the server. It fails when a built-in page file is not valid.
pub fn tools<S: Send + Sync + 'static>() -> Result<ToolRouter<S>> {
    let pages = Arc::new(Pages::load()?);
    let index = Arc::new(Index::new(&pages));
    let wiki_read = Tool::new(
        "wiki_read",
        "Read one Arma Reforger wiki page or Enfusion engine page by its full title in `title`, for \
         example \"Arma Reforger:Server Config\", or only one Section of it with `section`. If \
         no page has that title, the answer lists the closest real titles. If no Section has that \
         name, the answer lists the Sections of the page.",
        schema_for_type::<ReadInput>(),
    )
    .with_title("Read a wiki page")
    .annotate(read_only_labels());
    let wiki_search = Tool::new(
        "wiki_search",
        "Find Arma Reforger wiki pages and Enfusion engine pages by words. Each result gives the page \
         title, the path of its best Section, and a short text from it: read it with wiki_read. \
         If no page matches, the answer says what to try next.",
        schema_for_type::<SearchInput>(),
    )
    .with_title("Search the wiki")
    .annotate(read_only_labels());
    let read_pages = Arc::clone(&pages);
    Ok(ToolRouter::new()
        .with_route((
            wiki_read,
            move |Parameters(input): Parameters<ReadInput>| read(&read_pages, &input),
        ))
        .with_route((
            wiki_search,
            move |Parameters(input): Parameters<SearchInput>| search(&pages, &index, &input),
        )))
}
