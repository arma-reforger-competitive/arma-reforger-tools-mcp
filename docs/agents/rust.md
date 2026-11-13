# Rust

This repo follows the shared Rust practices in `~/www/home/docs/agents/rust/`. Read its `README.md` first, then the topic files that your task needs. This file adds the rules for this repo. Where the two differ, this file wins.

`cargo fmt` and `cargo clippy` check most rules. The lints are in `[lints]` in `Cargo.toml`.

## rmcp examples

The `rmcp` API changes between versions. Copy from the working code here before you read the `rmcp` source:

- A server with Tools: `src/server.rs`.
- A Capability that gives its Tools, and a test with an in-memory client: `src/wiki/read.rs`.
- A test that starts the real program over stdio: `tests/server.rs`.
- An in-memory server and client for the tests of every Capability: `src/test_support.rs`.

## Errors

- A wrong input from Claude is not a Rust error. Answer it as a Tool error with a hint (see `mcp.md`).
- A failure inside a Tool, for example a missing file or a setting that is not set, is a case of the Capability's `Error` enum (shared `errors.md`). The Tool answers it with `CallToolResult::error`, and the error's `Display` text is the hint for Claude.
- Use `#[expect]` only for code that a macro writes, and say which macro in the reason.

## Async

- The server runs on one thread. Do blocking work, for example reading `.pak` files or a long search, in `tokio::task::spawn_blocking`, so that the other Tools still answer.

## Tests

- Test a Capability through its public `tools()` function: serve it with `test_support::connect`, call the Tool with `test_support::call`, and check the text and `is_error`. Example: the tests in `src/wiki/read.rs`.
- `tests/server.rs` starts the real program and calls one Tool from each Capability.
- A Port keeps the expected answers of the old TypeScript tests. Take other expected values from the wiki text.
