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
    /// Rebuild the Knowledge base from the Workbench docs (`REFORGER_WORKBENCH_DOCS`) and src/wiki/source/export.xml.
    Import(ImportArgs),
}

#[derive(Args, Debug)]
pub struct ImportArgs {
    /// Only print the steps and the page list to save a new export.xml from the wiki.
    #[arg(long)]
    pub export_list: bool,
}
