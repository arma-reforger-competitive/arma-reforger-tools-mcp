//! `cargo xtask net-api`: one raw call to the Workbench NET API, with the bytes of
//! `src/workbench/wire.rs` and the host search of `src/wsl.rs`.

// region:    --- Modules

mod call;
mod error;

pub use call::run;
pub use error::Error;
use error::Result;

// endregion: --- Modules
