//! `cargo xtask call-tool`: one Tool call to `target/release/reforger` with the `REFORGER_*`
//! settings of Claude Code, which prints only the text of the answer.

// region:    --- Modules

mod call;
mod error;

pub use call::run;
pub use error::Error;
use error::Result;

// endregion: --- Modules
