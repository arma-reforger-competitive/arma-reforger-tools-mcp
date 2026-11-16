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

## game_find (#6)

- Ported the .pak reader from the old `src/pak/reader.ts` as `src/game-files/pak.rs`, and added the new `game_find` Tool. Its ranking comes from the old `asset_search`: full file name, then start of name, then part of name, then part of path.
- Fixed: a file `offset` is a position from the start of the .pak file, not from the DATA chunk. The old reader checked every chunk size, but not the sizes, offsets, and child counts in the FILE table. Now each one is checked, and a property test gives the reader damaged files.
- Changed: `game_find` also searches the Workshop mods that Workbench downloaded, and names the mod of each path. A class name with `SCR_` also finds the file without the prefix. A damaged .pak file is skipped and named in the answer. The old code skipped it in silence.
- The settings are `REFORGER_GAME_PATH` and `REFORGER_WORKBENCH_PROFILE_PATH`, in Windows or WSL form. The old `ENFUSION_GAME_PATH` is not read.

## workbench_status (#40)

- Ported the NET API wire format and the TCP call from the old `src/workbench/protocol.ts` and `client.ts`, and the Windows host of WSL from `detectWslHostIp` in `src/utils/wsl-path.ts`. It reads `/proc/net/route` and runs no `ip` command. One new Tool, `workbench_status`, and one handler script, `ReforgerBridge.c`, in place of the old 20 handler files.
- Not ported: the start of Workbench, the 30-second poll for a recompile, and the dependency links (ADR 0009). After a copy, the bridge calls the built-in `ReloadScripts` once. A test on Workbench 1.6 showed that it loads a new or changed handler script with no restart.
- Changed: the bridge finds the open mod from `GetLoadedProjects` and the project list of Workbench in Documents. The old code guessed it from a project folder setting. Every answer of the handler script has its version, so an old copy is replaced.
- Pack test: Workbench packs the handler script into the mod (`data.pak` of BridgeTest held `Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c`), and has no setting to leave a folder out. The README says to delete the folder before a pack and gives the `.gitignore` line.
