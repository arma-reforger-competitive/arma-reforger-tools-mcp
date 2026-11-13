# MCP Tools

## Names

A Tool name is `<word>_<verb>` in snake_case. The word is the first word of the Capability folder (ADR 0004): `wiki/` gives `wiki_read`, `game-files/` gives `game_find`.

## Wrong input

A wrong input gives a normal Tool answer with `isError: true` and a hint that helps Claude call again. It does not give an MCP protocol error, because Claude does not see the text of a protocol error as a hint.

Example from `wiki_read`:

- An unknown title lists the 3 closest real titles.
- An empty title gives an example of a real title.
