# CLAUDE.md

A Claude Code plugin for Arma Reforger modding: an MCP server for Capabilities, skills for Know-how. See `GLOSSARY.md` for the words and `docs/adr/` for the decisions.

## Porting

This repo is built by porting Capabilities from the old enfusion-mcp project (ADR-0001). The old project is at `/home/yevhenii/www/arma-enfusion-mcp` (GitHub: `arma-reforger-competitive/arma-enfusion-mcp`). Treat it as read-only reference.

Before you port a Capability, read its old TypeScript code there. Write the same logic in Rust (ADR-0008), fix what is not sound, and keep the old expected answers as tests. Bring over only what the ticket needs. Translated code keeps the MIT notice from ADR-0001.

A Port is done when all four checks pass:

1. An automated test proves the real behaviour.
2. Each new Tool gives the expected answers in MCP Inspector: `npx -y @modelcontextprotocol/inspector@2.15.0 --cli target/release/reforger --method tools/call --tool-name <tool> --tool-arg <name>=<value>`. Run `cargo build --release` first.
3. One real question uses the new Tools and gets a correct answer: `claude -p "<question>" --allowedTools "mcp__plugin_reforger_reforger__<tool>"`.
4. `docs/build-log.md` has a 3-5 line entry: what was ported, what changed from the old code, and why.

## Code

Read `docs/agents/rust.md` before you change Rust code, and `docs/agents/mcp.md` before you add or change a Tool.

## Commits

Commit messages follow Conventional Commits with this project's types and scopes. Read `docs/agents/commits.md` before every commit.
