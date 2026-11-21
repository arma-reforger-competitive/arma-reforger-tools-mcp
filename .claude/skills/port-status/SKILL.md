---
name: port-status
description: Show what is left to port from the old enfusion-mcp project, and record the owner's decisions in the Port ledger.
disable-model-invocation: true
---

The **Port ledger** (`docs/port-ledger.md`) lists every feature of the old project with one status: `ported`, `not needed`, or `not yet`. The owner decides `not needed` and `not yet`. An agent sets `ported` when its Port is done (check 5 in `CLAUDE.md`). A suggestion is only evidence for the owner.

`cargo xtask port-status` does all reading and writing of the ledger. Change the ledger only through it. `cargo xtask port-status help` shows the commands.

## Steps

1. **Report.** Run `cargo xtask port-status report`. It adds the features that the ledger does not list yet, writes new suggestions from the session history, and checks that every `ported` row links to a Tool or skill that exists. The step is done when the command ends with a verdict line.

2. **Show the open rows.** Give the owner, in plain words:
   - the history line, because a short history makes the suggestions weak;
   - every `ported` row that the report lists as broken;
   - the `not yet` rows, grouped by kind and by suggestion, with the usage numbers. Name groups the owner can decide at once, for example "the 12 `*_create` tools, never used".

3. **Ask for decisions.** For each row or group, the owner picks one:
   - **port it**: create a ticket (see below), then record it with `cargo xtask port-status set not-yet <selector> --ticket "[#<n>](<url>)"`;
   - **not needed**: `cargo xtask port-status set not-needed <selector>... --reason "<the owner's reason>"`;
   - **ported**: `cargo xtask port-status set ported <selector> --tool <tool name>` or `--skill <skill name>`;
   - **wait**: change nothing.

   One `set` takes many selectors and globs, so a decision for a group is one command. A `not needed` row goes back with `cargo xtask port-status set not-yet <selector>`. Write the owner's reason in their words. The step is done when the owner has answered every row or group you showed, or has said to stop.

4. **Verdict.** Run `cargo xtask port-status report` again. Give the owner its last block: "Port done: nothing left in the old repo." or "Not done: N features still open." with the list.

5. **Commit.** The ledger is data: commit the owner's decisions in `docs/port-ledger.md` straight to `main` with `docs(port-status): <what was decided>`. A `ported` row that a Port sets goes in the Port's PR, so it turns true when the PR merges.

## Port tickets

A port ticket is a GitHub issue in this repo, a sub-issue of the roadmap issue "Roadmap: rebuild enfusion-mcp as the reforger plugin" (#1), with the label `needs-triage`. Its body names the old feature, the owner's reason to port it, and the usage numbers from the report. `docs/agents/issue-tracker.md` in `~/www/home` has the `gh` commands.

Each claim in the body about how Workbench or the engine behaves names its source: a wiki page, a Game file path, or a test run with its result. Write "(unchecked)" after a claim that has no source. The agent that does the Port tests an unchecked claim before it builds on it.

## Usage evidence

The report reads Claude Code session history in `~/.claude/projects`:

- A Tool counts when Claude called `mcp__enfusion-mcp__<tool>`. An error is a call whose result has `is_error`.
- A prompt counts when the owner ran it as a slash command. A resource counts when Claude read its URI with `ReadMcpResourceTool`.
- Claude cannot call templates, recipes, patterns, knowledge pages, or the animation graph code. Their suggestion comes from the old Tools that use them.
- "Sessions" means sessions in which the old server appears at all. The history covers only the dates in the history line.
