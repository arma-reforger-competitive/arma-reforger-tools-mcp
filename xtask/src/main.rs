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

use std::io::Write as _;
use std::process::ExitCode;

use clap::Parser as _;

fn main() -> ExitCode {
    let result = match cmd::Cmd::parse().command {
        cmd::SubCmd::Import { export_list } => import::exec(export_list),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let _ = writeln!(std::io::stderr(), "error: {err}");
            ExitCode::FAILURE
        }
    }
}
