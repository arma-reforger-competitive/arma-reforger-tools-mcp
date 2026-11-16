//! Developer commands of this repo. Run them with `cargo xtask <command>`.

// region:    --- Modules

mod cmd;
mod doxygen;
mod error;
mod import;
mod knowledge;
mod wiki_export;
mod wikitext;

pub use error::{Error, Result};

// endregion: --- Modules

use std::process::ExitCode;

use clap::Parser as _;

use cmd::{Cmd, ImportArgs, SubCmd};

fn main() -> ExitCode {
    let result = match Cmd::parse().command {
        SubCmd::Import(ImportArgs { export_list: true }) => import::print_export_steps(),
        SubCmd::Import(ImportArgs { export_list: false }) => import::run(),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::FAILURE
        }
    }
}
