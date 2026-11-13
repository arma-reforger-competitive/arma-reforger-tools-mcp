# Rust

`cargo fmt` and `cargo clippy` check most rules. The lints are in `[lints]` in `Cargo.toml`. This file holds only the rules that no tool checks.

## rmcp examples

The `rmcp` API changes between versions. Copy from the working code here before you read the `rmcp` source:

- A server with Tools: `src/main.rs`.
- A Capability that gives its Tools, and a test with an in-memory client: `src/wiki/mod.rs`.
- A test that starts the real program over stdio: `tests/server.rs`.

## Errors

- Check data when the server starts, not on the first Tool call. Example: `wiki::tools()` parses `pages.json` and returns the error.
- A wrong input from Claude is not a Rust error. Answer it as a Tool error with a hint (see `mcp.md`).
- Use `#[expect]` only for code that a macro writes, and say which macro in the reason.

## Async

- The server runs on one thread. Do blocking work, for example reading `.pak` files or a long search, in `tokio::task::spawn_blocking`, so that the other Tools still answer.

## Tests

- Test a Capability through its public `tools()` function: serve it on a test server, call the Tool with an MCP client, and check the text and `isError`. Example: the tests in `src/wiki/mod.rs`.
- `tests/server.rs` starts the real program and calls one Tool from each Capability.
- Name a test by the behaviour it proves: `suggests_the_closest_real_titles_when_the_title_is_unknown`.
- A Port keeps the expected answers of the old TypeScript tests.
- Take expected values from the old answers or from the wiki text, never from the code under test.
