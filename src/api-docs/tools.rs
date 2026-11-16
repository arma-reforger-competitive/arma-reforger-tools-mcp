//! The Tools of the api-docs Capability.

use std::sync::Arc;

use rmcp::handler::server::{
    common::schema_for_type, router::tool::ToolRouter, wrapper::Parameters,
};
use rmcp::model::Tool;

use super::Result;
use super::classes::Classes;
use super::search::{Index, SearchInput, search};

/// Gives the api-docs Tools to the server. It fails when a line of `classes.jsonl` is not valid.
pub fn tools<S: Send + Sync + 'static>() -> Result<ToolRouter<S>> {
    let classes = Arc::new(Classes::load()?);
    let index = Arc::new(Index::new(&classes));
    let api_search = Tool::new(
        "api_search",
        "Search the Enfusion engine and Arma Reforger script API by the full name or a part of the \
         name of a class or a method, for example \"SCR_BaseGameMode\", \"GetFocusFOVScale\", or \
         \"CharacterCamera\". A full class name gives that class with the classes it inherits from \
         and all its methods and properties. Other matches come as a list: each class with its \
         parent, and each method with its class and signature. If no name matches, the answer \
         lists the names with almost the same spelling.",
        schema_for_type::<SearchInput>(),
    );
    Ok(ToolRouter::new().with_route((
        api_search,
        move |Parameters(input): Parameters<SearchInput>| search(&classes, &index, &input),
    )))
}
