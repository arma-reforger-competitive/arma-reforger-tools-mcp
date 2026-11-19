# Rust

This repo follows the shared Rust practices in `~/www/home/docs/agents/rust/`. Read its `README.md` first, then the topic files that your task needs. This file adds the rules for this repo. Where the two differ, this file wins.

`cargo fmt` and `cargo clippy` check most rules. The lints are in `[lints]` in `Cargo.toml`.

`xtask` has its own lint table in `xtask/Cargo.toml`, without `print_stdout` and `print_stderr`. Those two lints protect the stdout of the MCP server, but `xtask` is a command-line tool that prints for a person, so it uses `println!`.

## rmcp examples

The `rmcp` API changes between versions. Copy from the working code here before you read the `rmcp` source:

- A server with Tools: `src/server.rs`.
- A Capability that gives its Tools: `src/wiki/tools.rs`.
- A test with an in-memory client: the tests in `src/wiki/read.rs`.
- A test that starts the real program over stdio: `tests/server.rs`.
- An in-memory server and client for the tests of every Capability: `src/test_support.rs`.

## Public items

- A function for a later ticket becomes `pub` in the PR that adds its first caller. This crate is a program with no library part, so `cargo clippy` fails on a `pub` function that nothing calls (`dead_code`).

## Errors

- A wrong input from Claude is not a Rust error. Answer it as a Tool error with a hint (see `mcp.md`).
- A failure inside a Tool, for example a missing file or a setting that is not set, is a case of the Capability's `Error` enum (shared `errors.md`). The Tool answers it with `CallToolResult::error`, and the error's `Display` text is the hint for Claude.
- A Tool whose job is to report a state answers each state as a normal answer, also when another Tool gives that state as an error. Example: `workbench_status` answers "Workbench is not running" with `CallToolResult::success`.
- A Capability's `mod.rs` brings in `Error` and `Result` with `use error::{Error, Result};`, not `pub use`: ADR 0004 lets a Capability make only its `tools()` function public (and `Bridge` in `workbench`). This rule wins over "Its parent re-exports both" in the shared `errors.md`.
- Use `#[expect]` only for code that a macro writes, and say which macro in the reason.

## Async

- The server runs on one thread. Do blocking work, for example reading `.pak` files or a long search, in `tokio::task::spawn_blocking`, so that the other Tools still answer.
- When Claude cancels a call, `src/server.rs` stops the Tool at its next `await`. Work in `spawn_blocking` does not stop that way. Give the Tool a `CancellationToken` argument (rmcp fills it in), move a clone into the blocking work, and check `is_cancelled()` in each step of its loop.
- A Tool that writes a file first writes a temp file in the same folder, then renames it. A rename is all or nothing, so a stopped call never leaves half a file.

## Logs

- The server writes logs to stderr at the INFO level (`src/main.rs`). rmcp logs the start, each notification, each cancel, and the stop. Claude Code saves the logs in `~/.cache/claude-cli-nodejs/<project>/mcp-logs-plugin-reforger-reforger/`.
- To log from a Tool, add the `tracing` crate and use `tracing::info!` or `tracing::warn!`. Never print to stdout: it carries the MCP messages. A test in `tests/server.rs` fails when a log line goes to stdout.

## Tests

- Test a Capability through its public `tools()` function: serve it with `test_support::connect`, call the Tool with `test_support::call`, and check the text and `is_error`. Example: the tests in `src/wiki/read.rs`.
- `tests/server.rs` starts the real program. It calls one Tool from each Capability, and it tests the work of the server for all Tools, for example the cut of long answers.
- A Port keeps the expected answers of the old TypeScript tests. Take other expected values from the wiki text.
