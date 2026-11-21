//! Developer commands of this repo. Run them with `cargo xtask <command>`.

// region:    --- Modules

mod call_tool;
mod ci;
mod cmd;
mod doxygen;
mod error;
mod import;
mod knowledge;
mod net_api;
mod tool_choice;
mod wiki_export;
mod wikitext;
// The server's own files, so that `net-api` sends the same bytes and finds the same host as the
// Workbench bridge.
#[path = "../../src/workbench/wire.rs"]
mod wire;
#[path = "../../src/wsl.rs"]
mod wsl;

pub use error::{Error, Result};

// endregion: --- Modules

use std::process::ExitCode;

use clap::Parser as _;

use cmd::{Cmd, ImportArgs, SubCmd};

fn main() -> ExitCode {
    let result = match Cmd::parse().command {
        SubCmd::Ci => ci::run(),
        SubCmd::Import(ImportArgs { export_list: true }) => import::print_export_steps(),
        SubCmd::Import(ImportArgs { export_list: false }) => import::run(),
        SubCmd::ToolChoice(args) => tool_choice::run(&args),
        SubCmd::NetApi(args) => net_api::run(&args).map_err(Error::from),
        SubCmd::CallTool(args) => call_tool::run(&args).map_err(Error::from),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
