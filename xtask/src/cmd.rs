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
    /// Ask Claude every Tool-choice test in tests/tool-choice/cases.jsonl and check its first call.
    /// Needs a Claude Code login, and costs model use.
    ToolChoice(ToolChoiceArgs),
}

#[derive(Args, Debug)]
pub struct ImportArgs {
    /// Only print the steps and the page list to save a new export.xml from the wiki.
    #[arg(long)]
    pub export_list: bool,
}

#[derive(Args, Debug)]
pub struct ToolChoiceArgs {
    /// The Claude model. A small model shows a weak Tool description first.
    #[arg(long, default_value = "haiku")]
    pub model: String,
    /// How many times to ask each question. Answers differ between runs.
    #[arg(long, default_value_t = 3)]
    pub runs: usize,
}
