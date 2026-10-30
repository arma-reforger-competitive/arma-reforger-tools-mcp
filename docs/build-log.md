# Build log

## wiki_read (#2)

- Ported `wiki_read` and the wiki pages (258) from the old `src/tools/wiki-read.ts` and `data/wiki/pages.json`. The raw `export.xml` and the scrapers stay in the old repo.
- Pages now have their full wiki title, taken from the URL (`Arma Reforger:Mod Project Setup`, not `Mod Project Setup`), so the titles match the wiki.
- A wrong title now gives the 3 closest titles by letter-pair similarity, marked as an error (`isError`). The old code ranked pages by words found in their title and text, so a typo in a word found nothing for that word.
- New stack: MCP SDK 2 (`@modelcontextprotocol/server`), zod 4, and Node runs the TypeScript directly, so there is no build output to ship.
