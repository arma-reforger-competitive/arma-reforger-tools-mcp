# Arma Reforger Tools MCP

> **Work in progress.** The plugin installs and has the six tools below. Follow the [roadmap](https://github.com/lanka-community/arma-reforger-tools-mcp/issues/1) to see what is done and what comes next.

`reforger` is a Claude Code plugin that helps you learn, build, and fix Arma Reforger mods. It gives Claude two kinds of help:

- **MCP tools** for jobs that need code: searching the Enfusion script API and the Reforger wiki, and finding and reading the game's own files.
- **Skills** with Reforger know-how: steps, rules, examples, and templates for modding tasks.

This project rebuilds the older [enfusion-mcp](https://github.com/lanka-community/arma-enfusion-mcp) step by step, one part at a time.

## Install

You need Claude Code and Rust. Install Rust with [rustup](https://rustup.rs/). This repo's `rust-toolchain.toml` names the Rust version, and rustup installs it on the first `cargo` command.

```sh
git clone https://github.com/lanka-community/arma-reforger-tools-mcp.git
cd arma-reforger-tools-mcp
cargo build --release
claude plugin marketplace add ./
claude plugin install reforger@reforger-local
```

`cargo build --release` makes the server program `target/release/reforger`. Claude Code runs it from this folder, so keep the folder after the install. After a `git pull`, run `cargo build --release` again. To check the install, run `claude mcp list`. The line `plugin:reforger:reforger` must show `Connected`.

## Tools

| Tool               | What it does                                                                                                   |
| ------------------ | -------------------------------------------------------------------------------------------------------------- |
| `api_search`       | Finds classes and methods of the Enfusion and Arma Reforger script API, with signatures and parent classes.    |
| `wiki_search`      | Finds wiki pages and engine pages by words, with the best Section of each.                                     |
| `wiki_read`        | Returns one Arma Reforger wiki page or Enfusion engine page by its title, or one Section of it.                |
| `game_find`        | Finds the scripts, Prefabs, and configs inside the `.pak` files of the game and of downloaded Workshop mods.   |
| `workbench_status` | Checks the connection to a running Workbench, copies the Handler script into the open mod, and names that mod. |
| `workbench_launch` | Starts Workbench with one of your mods open, finds it through Steam, and waits until the scripts are compiled. |
| `guid_generate`    | Makes a new random GUID for the `.gproj` file of a new mod: 16 hex characters in upper case.                   |

`game_find` and the Workbench tools need settings. Their READMEs list them: [`src/game-files/README.md`](src/game-files/README.md) and [`src/workbench/README.md`](src/workbench/README.md).

## Development

```sh
cargo fmt             # fix the format
cargo xtask ci        # run every check of CI, in the same order
cargo xtask import    # rebuild the Knowledge base after a game update
cargo xtask tool-choice  # check that Claude calls the right Tool (needs a Claude Code login)
```

`cargo xtask ci` runs these checks in this order and stops at the first failure. CI runs `cargo xtask ci` on every push and pull request. When you add a check, add it here and to `STEPS` in [`xtask/src/ci.rs`](xtask/src/ci.rs).

```sh
cargo fmt --check
ast-grep test -c lint/ast-grep/sgconfig.yml --skip-snapshot-tests
ast-grep scan -c lint/ast-grep/sgconfig.yml src
cargo clippy --workspace --all-targets -- -D warnings
cargo nextest run --workspace --profile ci
node --test .claude/skills/port-status/port-status.test.ts
cargo deny --workspace check
```

The checks need `ast-grep`, `cargo-deny`, `cargo-nextest`, and Node 24. Install the first three with `cargo install --locked ast-grep cargo-deny cargo-nextest`. `cargo-nextest` runs the tests. Its settings are in [`.config/nextest.toml`](.config/nextest.toml). `ast-grep` stops a `String` used as an error. Its rule is in [`lint/ast-grep/`](lint/ast-grep/).

`cargo xtask tool-choice` asks Claude Code (Haiku) each question in [`tests/tool-choice/cases.jsonl`](tests/tool-choice/cases.jsonl) 3 times, blocks the Tool call, and checks that the first call uses the expected Tool and inputs. At least 90 % must pass. CI does not run it, because it needs a Claude Code login and costs model use: about $0.02 for one run.

`cargo xtask import` needs the Workbench docs of Arma Reforger Tools. [`src/wiki/README.md`](src/wiki/README.md) has the steps.

Each Capability has its own folder in `src/`. The rules for Rust code are in [`docs/agents/rust.md`](docs/agents/rust.md), and the rules for Tools are in [`docs/agents/mcp.md`](docs/agents/mcp.md).

## Learn more

- [`GLOSSARY.md`](GLOSSARY.md): the words this project uses
- [`docs/adr/`](docs/adr/): the design decisions and why we made them
- [`docs/build-log.md`](docs/build-log.md): what each Port changed

## Disclaimer

Unofficial. Not affiliated with Bohemia Interactive. Arma Reforger is a trademark of Bohemia Interactive a.s.
