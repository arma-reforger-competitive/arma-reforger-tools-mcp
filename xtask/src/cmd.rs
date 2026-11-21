//! The command-line arguments.

use clap::{Args, Parser, Subcommand, ValueEnum};

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
    /// Send one call to the Workbench NET API, and print the status and the JSON answer.
    /// Reads `REFORGER_WORKBENCH_HOST` and `REFORGER_WORKBENCH_PORT`, the same as the Workbench bridge.
    NetApi(NetApiArgs),
    /// Call one Tool of target/release/reforger, and print only the text of the answer.
    /// Passes the `REFORGER_*` settings of ~/.claude/settings.json; a shell variable wins.
    /// Exits with 1 when the answer is an error. Run `cargo build --release` first.
    CallTool(CallToolArgs),
    /// Keep the Port ledger (docs/port-ledger.md) up to date.
    /// Reads `PORT_STATUS_OLD_REPO` (the old project, default ~/www/arma-enfusion-mcp) and
    /// `PORT_STATUS_HISTORY` (Claude Code session history, default ~/.claude/projects).
    #[command(subcommand)]
    PortStatus(PortStatusCmd),
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
    /// Ask only the tests that accept this Tool, for example `reference_sync`. Use it before the PR
    /// of a new or changed Tool; run every test before the merge.
    #[arg(long)]
    pub tool: Option<String>,
}

#[derive(Args, Debug)]
pub struct NetApiArgs {
    /// The NET API function (`APIFunc`), for example `IsWorkbenchRunning` or `ReforgerBridge`.
    pub function: String,
    /// The parameters as one JSON object, for example '{"action":"ping"}'. Default: {}.
    pub params: Option<String>,
}

#[derive(Args, Debug)]
pub struct CallToolArgs {
    /// The Tool, for example `game_find`.
    pub tool: String,
    /// The inputs of the Tool, each as name=value, for example query=S3C.
    pub inputs: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum PortStatusCmd {
    /// Add missing features to docs/port-ledger.md, write new suggestions, check the "ported"
    /// rows, and print the open rows and the verdict.
    Report,
    /// Give every row that a selector names the same status.
    /// not-needed needs --reason. ported needs --tool or --skill. not-yet takes an optional --ticket.
    Set(PortStatusSetArgs),
}

#[derive(Args, Debug)]
pub struct PortStatusSetArgs {
    /// The new status of the rows.
    pub status: NewStatus,
    /// A name, a glob such as "*_create", or "<kind>:<name or glob>".
    #[arg(required = true)]
    pub selectors: Vec<String>,
    /// The reason for not-needed.
    #[arg(long)]
    pub reason: Option<String>,
    /// The Tool of this repo for ported.
    #[arg(long, conflicts_with = "skill")]
    pub tool: Option<String>,
    /// The skill of this repo for ported.
    #[arg(long)]
    pub skill: Option<String>,
    /// The port ticket for not-yet, for example "[#12](https://github.com/...)".
    #[arg(long)]
    pub ticket: Option<String>,
}

#[derive(ValueEnum, Clone, Copy, Debug)]
pub enum NewStatus {
    Ported,
    NotNeeded,
    NotYet,
}
