# MCP Tools

## Names

A Tool name is `<word>_<verb>` in snake_case. The word is the first word of the Capability folder (ADR 0004): `wiki/` gives `wiki_read`, `game-files/` gives `game_find`.

## Descriptions

Write each description for a person who has never seen the Tool. Say what the Tool returns, and what it answers when nothing matches. Give every input a description with an example. A test in `tests/server.rs` fails when a Tool or an input has no description.

Example: `wiki_read` says that it returns the full text of one page, and that an unknown title gives the closest real titles.

## Tool-choice tests

A Tool description, an input description, and the Server guide decide which Tool Claude calls. A change to any of them is a change in behaviour. Before you open the PR, run `cargo xtask tool-choice --tool <name>` for each new or changed Tool: it asks only the tests that accept that Tool, so it costs little. Run `cargo xtask tool-choice` with no `--tool` before the merge, because a change in the Server guide can pull Claude away from other Tools. Put the pass rate of the last full run in the PR.

A new Tool gets at least 2 Tool-choice tests in `tests/tool-choice/cases.jsonl`. Write the question as a user asks it, not with words from the description. Accept every Tool that gives a good answer.

When a test fails, fix the Tool text and run again. An example in a description can push Claude to it: `wiki_read` gave "Arma Reforger:Mod Project Setup" as an example title, and Claude read that page instead of searching when a question was close to it. Use an example that no test asks about.

## Server guide

The Server guide (`SERVER_GUIDE` in `src/server.rs`) says in a few lines which Tool fits which job. Name each Tool in backticks. Claude Code loads the input schema of a Tool only when Claude asks for it, so Claude can call a Tool with a guessed input name. When Claude guesses wrong, name the input in backticks in the guide: "`wiki_read`: read one page by its `title`" stopped Claude from sending `page`. When you add, rename, or remove a Tool, change the guide: a test in `tests/server.rs` fails when the guide and the served Tools differ.

## Tool labels

Every Tool has a title and all four Tool labels: read-only, destructive, safe to call again (idempotent), and open-world (it uses the internet). A test in `tests/server.rs` fails when one is missing.

- A Tool that only reads files on this computer uses `server::read_only_labels()`. Example: `wiki_read`.
- A Tool that changes files is not read-only. It is destructive when it deletes or overwrites a file. A Tool that deletes is its own Tool, separate from the Tools that only read.
- A Tool that downloads from the Workshop or the wiki is open-world.

## Answer size

Claude Code cuts a Tool answer at about 25,000 tokens. That is about 100,000 characters. The server cuts every answer at 100,000 characters before Claude Code gets it (`src/answer_size.rs`). It counts the text of all blocks together. A cut answer ends with "(Cut at 100,000 of N characters.)". Example: `wiki_read` of `Arma Reforger:Diag Menu` ends with "(Cut at 100,000 of 164,582 characters.)".

A Tool that returns a list keeps the list short with a limit, so that the server does not cut it. Example: `wiki_search` returns at most 10 pages.

## Wrong input

A wrong input gives a normal Tool answer with `isError: true` and a hint that helps Claude call again. It does not give an MCP protocol error, because Claude does not see the text of a protocol error as a hint.

Example from `wiki_read`:

- An unknown title lists the 3 closest real titles.
- An empty title gives an example of a real title.
