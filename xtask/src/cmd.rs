//! The command-line arguments.

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(about = "Developer commands of the reforger plugin repo")]
pub struct Cmd {
    #[command(subcommand)]
    pub command: SubCmd,
}

#[derive(Subcommand, Debug)]
pub enum SubCmd {
    /// Run every check of CI, in the same order as CI. Stops at the first failure.
    Ci,
    /// Rebuild the Knowledge base from the Workbench docs (`REFORGER_WORKBENCH_DOCS`) and src/wiki/source/export.xml.
    Import(ImportArgs),
}

#[derive(Args, Debug)]
pub struct ImportArgs {
    /// Only print the steps and the page list to save a new export.xml from the wiki.
    #[arg(long)]
    pub export_list: bool,
}
