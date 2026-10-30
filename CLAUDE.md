# CLAUDE.md

A Claude Code plugin for Arma Reforger modding: an MCP server for Capabilities, skills for Know-how. See `GLOSSARY.md` for the words and `docs/adr/` for the decisions.

## Porting

This repo is built by porting Capabilities from the old enfusion-mcp project (ADR-0001). The old project is at `/home/yevhenii/www/arma-enfusion-mcp` (GitHub: `arma-reforger-competitive/arma-enfusion-mcp`). Treat it as read-only reference.

Before you port a Capability, read its old code there. Reuse what is sound, rewrite what is not, and bring over only what the ticket needs. Copied code keeps the MIT notice from ADR-0001.

A Port is done when all four checks pass:

1. An automated test proves the real behaviour.
2. The tools give the expected answers when called by hand in MCP Inspector.
3. One real question in Claude Code uses the tools and gets a correct answer.
4. `docs/build-log.md` has a 3-5 line entry: what was ported, what changed from the old code, and why.

## Agent skills

### Issue tracker

Issues live as GitHub issues; use the `gh` CLI for all operations. See `docs/agents/issue-tracker.md`.

### Triage labels

Five canonical labels: `needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `GLOSSARY.md` + `docs/adr/` at the repo root. See `docs/agents/domain.md`.
