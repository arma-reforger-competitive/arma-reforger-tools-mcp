# MCP Tools

## Names

A Tool name is `<word>_<verb>` in snake_case. The word is the first word of the Capability folder (ADR 0004): `wiki/` gives `wiki_read`, `game-files/` gives `game_find`.

## Descriptions

Write each description for a person who has never seen the Tool. Say what the Tool returns, and what it answers when nothing matches. Give every input a description with an example. A test in `tests/server.rs` fails when a Tool or an input has no description.

Example: `wiki_read` says that it returns the full text of one page, and that an unknown title gives the closest real titles.

## Answer size

Claude Code cuts a Tool answer at about 25,000 tokens. That is about 100,000 characters. The server cuts every answer at 100,000 characters before Claude Code gets it (`src/answer_size.rs`). It counts the text of all blocks together. A cut answer ends with "(Cut at 100,000 of N characters.)". Example: `wiki_read` of `Arma Reforger:Diag Menu` ends with "(Cut at 100,000 of 164,582 characters.)".

A Tool that returns a list keeps the list short with a limit, so that the server does not cut it. Example: `wiki_search` returns at most 10 pages.

## Wrong input

A wrong input gives a normal Tool answer with `isError: true` and a hint that helps Claude call again. It does not give an MCP protocol error, because Claude does not see the text of a protocol error as a hint.

Example from `wiki_read`:

- An unknown title lists the 3 closest real titles.
- An empty title gives an example of a real title.
