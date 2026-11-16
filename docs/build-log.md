# Build log

## wiki_read (#2)

- Ported `wiki_read` and the wiki pages (258) from the old `src/tools/wiki-read.ts` and `data/wiki/pages.json`. The raw `export.xml` and the scrapers stay in the old repo.
- Pages now have their full wiki title, taken from the URL (`Arma Reforger:Mod Project Setup`, not `Mod Project Setup`), so the titles match the wiki.
- A wrong title now gives the 3 closest titles by letter-pair similarity, marked as an error (`isError`). The old code ranked pages by words found in their title and text, so a typo in a word found nothing for that word.
- New stack: MCP SDK 2 (`@modelcontextprotocol/server`), zod 4, and Node runs the TypeScript directly, so there is no build output to ship.

## Rust server (#21)

- Translated the `wiki_read` Tool and the page search from TypeScript into Rust with the MCP SDK `rmcp` (ADR 0008).
- Same answers as before: a script sent 1,813 titles (every page title, with case, underscore, namespace and typo changes) to both servers, and all answers matched.
- Changed: an empty title now gives an error with an example title. Before, the input check gave "Too small", and that did not help Claude.
- The pages are built into the program with `include_str!`. The server checks them when it starts.

## docs import (#25)

- Replaced the old scrapers (`scripts/parse-wiki-export.ts`, `src/scraper/`) with one Rust command, `cargo xtask import`. It rebuilds the wiki pages, the engine pages, and the API classes. `pages.json` is gone: each page is now a Markdown file.
- The wikitext converter expands templates inner first, as the wiki does. The old regular expressions lost code names (`fastValidation`), broke headings, and left `}}`. An unknown template now stops the import.
- 252 wiki pages, not 251: the old converter kept "Known Issues" and dropped "Roadmap", two pages of the same kind. The new rule keeps every page with text.
- 24 engine pages, not 7, with headings, lists, tables, and code kept. 9,119 API classes, not 8,693: the basic types such as `string` and `array` are classes too. Method names no longer have their parameter types glued on (old: `SetParentRplNodebool`).

## wiki_search (#4)

- Ported `wiki_search` from the old `src/tools/wiki-search.ts` and `searchWiki` in `src/index/search-engine.ts`. The old tests stay: a known word finds pages, `limit` caps the results, a nonsense word finds nothing.
- Not ported: the old ranking. It found text inside other words ("mod" matched "model") and gave a common word the same weight as a rare one. The new BM25 ranking scores whole words in each Section and the page title once per page, with no search library (ADR 0005).
- Changed: each result shows the path of its best Section and a short text, not the first 2,000 characters of the page, and not the source and the URL: `wiki_read` gives them. `wiki_read` takes a `section`, so Claude reads only that part.
- A test set of 11 searches that Claude sent to the old server finds the expected page in the top 3.

## api_search (#5)

- Ported `api_search` from the old `src/tools/api-search.ts` and `searchClasses` and `searchMethods` in `src/index/search-engine.ts`. The old tests stay: a full class name comes first, and the typos "ScriptCompnent" and "GetPositon" still find `ScriptComponent` and `GetPosition`. The data is the `classes.jsonl` from #25, not a trimmed copy of the old JSON.
- Same ranking of names as before: full name, then start of name, then part of name. A full class name now gives the class page with the whole inheritance chain, and the other matches come as a short list with a note that says how to narrow it. Each word of the query is one name, so "PlayerLimit MaxPlayers" finds both. The old code searched the whole text as one name and found nothing.
- Not ported: enum and property search, the tree format, inherited members, related classes, and matches in class descriptions. The ticket asks only for class and method names. The typo fallback keeps the edit distance of 2 but drops the trigram match, which listed names that only look a little alike.
- A test set of 12 searches that Claude sent to the old server finds the expected class or method in the first 10 results.
