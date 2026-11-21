//! `cargo xtask port-status`: keeps the Port ledger (`docs/port-ledger.md`) up to date. The
//! skill `.claude/skills/port-status/SKILL.md` says how to use it.

// region:    --- Modules

mod collate;
mod commands;
mod error;
mod files;
mod ledger;
mod old_project;

pub use commands::run;
pub use error::Error;
use error::Result;

// endregion: --- Modules
