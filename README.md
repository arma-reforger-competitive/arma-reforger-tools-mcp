# Arma Reforger Tools MCP

> **Work in progress.** The plugin installs and has one tool, `wiki_read`. Follow the [roadmap](https://github.com/arma-reforger-competitive/arma-reforger-tools-mcp/issues/1) to see what is done and what comes next.

`reforger` is a Claude Code plugin that helps you learn, build, and fix Arma Reforger mods. It gives Claude two kinds of help:

- **MCP tools** for jobs that need code: searching the Enfusion script API and the Reforger wiki, and finding and reading the game's own files.
- **Skills** with Reforger know-how: steps, rules, examples, and templates for modding tasks.

This project rebuilds the older [enfusion-mcp](https://github.com/arma-reforger-competitive/arma-enfusion-mcp) step by step, one part at a time.

## Install

You need Claude Code and Rust. Install Rust with [rustup](https://rustup.rs/). This repo's `rust-toolchain.toml` names the Rust version, and rustup installs it on the first `cargo` command.

```sh
git clone https://github.com/arma-reforger-competitive/arma-reforger-tools-mcp.git
cd arma-reforger-tools-mcp
cargo build --release
claude plugin marketplace add ./
claude plugin install reforger@reforger-local
```

`cargo build --release` makes the server program `target/release/reforger`. Claude Code runs it from this folder, so keep the folder after the install. After a `git pull`, run `cargo build --release` again. To check the install, run `claude mcp list`. The line `plugin:reforger:reforger` must show `Connected`.

## Tools

| Tool        | What it does                                      |
| ----------- | ------------------------------------------------- |
| `wiki_read` | Returns one Arma Reforger wiki page by its title. |

## Development

```sh
cargo fmt                                    # fix the format
cargo clippy --all-targets -- -D warnings    # find code problems
cargo test                                   # run all tests
```

CI runs the same checks, with `cargo fmt --check`, on every push and pull request. Each Capability has its own folder in `src/`. The rules for Rust code are in [`docs/agents/rust.md`](docs/agents/rust.md), and the rules for Tools are in [`docs/agents/mcp.md`](docs/agents/mcp.md).

## Learn more

- [`GLOSSARY.md`](GLOSSARY.md): the words this project uses
- [`docs/adr/`](docs/adr/): the design decisions and why we made them
- [`docs/build-log.md`](docs/build-log.md): what each Port changed

## Disclaimer

Unofficial. Not affiliated with Bohemia Interactive. Arma Reforger is a trademark of Bohemia Interactive a.s.
