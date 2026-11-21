# CLAUDE.md

A Claude Code plugin for Arma Reforger modding: an MCP server for Capabilities, skills for Know-how. See `GLOSSARY.md` for the words and `docs/adr/` for the decisions.

## Where things are

- `src/`: the MCP server. `server.rs` lists the Tools of every Capability. `answer_size.rs` cuts long answers. `host_path.rs` turns a path setting into the form of the computer the server runs on, Windows or WSL, for every Capability (ADR 0006). `workbench_documents.rs` finds the Workbench folder in Documents for every Capability. `wsl.rs` sees WSL and finds its Windows host. `mod_folder.rs` parses a mod folder name `<Name>_<ID>` for every Capability. `test_support/` holds the in-memory server and the fake Workbench for the tests of every Capability. Read `server.rs` before you add a Capability.
- `src/<capability>/` (`wiki/`, `api-docs/`, `game-files/`, `workbench/`, `reference-mods/`, `guid/`): one Capability each: its Tools, logic, tests, and a `README.md`. Read the folder's `README.md` before you change a Tool (ADR-0004).
- `src/wiki/pages/` and `src/api-docs/classes.jsonl`: the Knowledge base. `cargo xtask import` writes them from the Workbench docs and `src/wiki/source/export.xml`, which the owner saves from the wiki (ADR-0005). To change them, run the import.
- `tests/`: tests that start the real program (`server.rs`) or check the Knowledge base files (`knowledge_base.rs`). `tests/tool-choice/cases.jsonl`: the Tool-choice tests.
- `xtask/`: developer commands: `cargo xtask ci` runs every check, `cargo xtask import` rebuilds the Knowledge base, `cargo xtask tool-choice` runs the Tool-choice tests, `cargo xtask net-api` sends one raw call to the Workbench NET API, `cargo xtask call-tool` calls one Tool of the release build and prints only its answer. `wire.rs` and `wsl.rs` of the server are built into `net-api` too, so they use only `std`. Read it before you change a check or the import.
- `lint/ast-grep/`: the check that stops a `String` used as an error. Read it when `cargo xtask ci` fails at an `ast-grep` step.
- `docs/adr/`: the decisions and why. `docs/agents/`: the rules for Rust code, Tools, and commits. `docs/port-ledger.md` and `docs/build-log.md`: what is ported and what changed. `docs/sources/`: read-only research copies.
- `.claude/skills/`: Developer skills of this repo: `port-status` (the Port ledger) and `workbench-test` (start, call, and close the real Workbench in a test). Read `workbench-test` before you test a change to `src/workbench/` on the real Workbench. `.claude-plugin/`: the plugin manifest.
- `.githooks/`: the pre-commit and commit-msg hooks. `.github/workflows/`: CI, which runs `cargo xtask ci`. `.config/nextest.toml`: the settings of the test runner.

## Porting

This repo is built by porting Capabilities from the old enfusion-mcp project (ADR-0001). The old project is at `/home/yevhenii/www/arma-enfusion-mcp` (GitHub: `lanka-community/arma-enfusion-mcp`). Treat it as read-only reference.

Before you port a Capability, read its old TypeScript code there. Write the same logic in Rust (ADR-0008), fix what is not sound, and keep the old expected answers as tests. Bring over only what the ticket needs. The MIT notice from ADR-0001 is in `LICENSE`. Code comments say what the code does now. The build log says what came from which old file.

A Port is done when all five checks pass:

1. An automated test proves the real behaviour.
2. Each new Tool gives the expected answers when you call it: `cargo xtask call-tool <tool> <name>=<value>`. It passes your `REFORGER_*` settings and prints only the answer. Run `cargo build --release` first.
3. Each new Tool has at least 2 Tool-choice tests in `tests/tool-choice/cases.jsonl`. `cargo xtask tool-choice --tool <tool>` passes before the PR, and `cargo xtask tool-choice` passes before the merge.
4. `docs/build-log.md` has a 3-5 line entry: what was ported, what changed from the old code, and why.
5. The Port's rows in the Port ledger are marked "ported": `node .claude/skills/port-status/port-status.ts set ported <old feature> --tool <new tool>`. The agent sets this itself, in the Port's PR, when checks 1-4 pass and the ticket has no work left.

## Code

Read `docs/agents/rust.md` before you change Rust code, and `docs/agents/mcp.md` before you add or change a Tool.

## Commits

Commit messages follow Conventional Commits with this project's types and scopes. Read `docs/agents/commits.md` before every commit.

The git hooks are in `.githooks/`. After a clone, turn them on: `git config core.hooksPath .githooks`. The pre-commit hook runs `node ~/www/home/tools/home.ts check-size`, which stops a staged file over 20,000 bytes and says how to split it. Then it runs `cargo fmt --check` and `cargo check --workspace` on the staged files, so every commit is formatted and builds. `.gitattributes` marks the Knowledge base `linguist-generated`, so check-size skips it. The commit-msg hook checks the message.
