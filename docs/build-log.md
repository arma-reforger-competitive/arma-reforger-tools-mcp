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

- Ported the NET API wire format and the TCP call from the old `src/workbench/protocol.ts` and `client.ts`, and the Windows host of WSL from `detectWslHostIp` in `src/utils/wsl-path.ts`. It reads `/proc/net/route` and runs no `ip` command. One new Tool, `workbench_status`, and one Handler script, `ReforgerBridge.c`, in place of the old 20 handler files.
- Not ported: the start of Workbench, the 30-second poll for a recompile, and the dependency links (ADR 0009). After a copy, the bridge calls the built-in `ReloadScripts` once. A test on Workbench 1.6 showed that it loads a new or changed Handler script with no restart.
- Changed: the bridge finds the open mod from `GetLoadedProjects` and the project list of Workbench in Documents. The old code guessed it from a project folder setting. Every answer of the Handler script has its version, so an old copy is replaced.
- Pack test: Workbench packs the Handler script into the mod (`data.pak` of BridgeTest held `Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c`), and has no setting to leave a folder out. The README says to delete the folder before a pack and gives the `.gitignore` line.

## workbench_launch (#61)

- Ported the start of Workbench from `launchWorkbench` in the old `src/workbench/client.ts` as a new Tool, `workbench_launch`. The bridge still never starts Workbench (ADR 0009).
- Changed: it finds Workbench and the game through Steam (the registry, `libraryfolders.vdf`, `appmanifest_<app>.acf`), with no path setting (ADR 0010). A missing app gives its `steam://install/<app>` link. It takes the mod from the project list of Workbench, by ID or folder name, and never a packed project: the game or a Workshop mod.
- Changed: in place of the old dependency links, the start adds `-addonsDir` from `REFORGER_WORKBENCH_PROFILE_PATH`. A test on Workbench 1.6 showed that a `-gproj` start without it does not load a mod whose Workshop dependency is in a moved download folder. The Tool now checks that the mod loaded and gives the errors of the Workbench log when it did not.
- Real test, BridgeTest from WSL: loaded and compiled in 8 to 15 seconds; with a Workshop dependency, loaded only with `-addonsDir`.

## workbench_launch starts Steam

- Workbench needs Steam with a logged-in user, and it does not start Steam itself. On Workbench 1.6 without Steam, it logged `SteamAPI_Init failed. Is Steam running?` and closed after 9 seconds, so `workbench_launch` waited 180 seconds for nothing.
- Now, when `ActiveUser` in `HKCU\Software\Valve\Steam\ActiveProcess` is 0, the Tool starts `steam.exe -silent` and waits for a user (at most 120 seconds), then starts Workbench. A Steam folder with no `steam.exe` gives the "install Steam" error.
- Real test, BridgeTest from WSL with Steam closed: Steam had a logged-in user after 8 seconds, and the whole call took 13 seconds.

## game_read (#7)

- Ported `game_read` from the old `src/tools/game-read.ts` and the file read of `src/pak/vfs.ts`. It reads the .pak files of the game and of the downloaded mods, with the index that `game_find` builds.
- Fixed: a compressed file is unpacked with zlib, not raw deflate (the old "invalid block type"), and the full size is checked against a 64 MB limit before memory is reserved. A real test opens every 20th script of the game.
- Changed: capital letters in the path do not matter, and a wrong path gives the 5 closest real paths. In the old project, 27 of 42 reads failed on guessed paths. A binary file is found by a zero byte, not by a list of extensions. `source` picks the game or a mod when both have the path.
- Not ported: the read of loose files outside the .pak files. The installed game has only .pak files.

## reference_sync without Workbench (#74)

- New Capability `reference-mods` with the Tool `reference_sync`. It is not a Port. It reads `reference-mods.jsonl` in each folder of `REFORGER_MODS_PATH` and keeps its own record in `<Workbench Documents folder>/addons/.reference-mods.jsonl`.
- It deletes only a mod that is in the record and on no list. Projects, links, `saves`, and mods downloaded by hand stay. A bad setting or a bad list line stops the call before any delete.
- A pure function makes the plan (delete, download, present). In this ticket a missing mod is a failed line with the message of the Workbench bridge: the download comes in a later ticket.
- The `workbench` Capability now gives `Bridge` to other Capabilities, so the bridge stays the only way to call Workbench.

## reference_sync downloads (#77)

- The Handler script got the actions `reference_download` and `reference_progress`, with the steps that #19 proved: `RequestPage` with `"search": "<ID>"`, keep the item with that exact ID, `AskDetail`, then `Download` of `GetLatestRevision()`. The license name comes from `License()`.
- The state of each download stays in Workbench memory between calls. A second start of the same ID starts nothing, unless its last try failed. One search runs at a time, because the Workshop keeps only the items of the last page.
- The Handler version and the version in `Cargo.toml` went up to 0.2.0. A test checks that the script has both actions and every field that `reference_sync` reads. The compile and the real download need the real Workbench.

## reference_sync review fixes (#72)

- The record keeps a `started` line for each download that Workbench accepted, so a download that outlives the server is recorded and later deleted. The record is the only state. The Handler script answers `unknown` for a download that it does not know, and it starts a complete or failed download again. The search keeps only the item with the exact ID.
- The wait ends at 30 seconds, progress questions included. `reference_sync` checks a cancel in its file work.
- `game-files` builds its file list again also when a `.pak` file in a mod folder changes, so a mod whose `.pak` came after its folder shows with no restart. Its notes name the right setting.
- One parse of the folder name `<Name>_<ID>` in `src/mod_folder.rs` for both Capabilities. ADR 0004 now lets `workbench` make `Bridge` public.

## workbench_launch checks that Steam runs (#77)

- The real Workbench check of #77 found the bug: Steam was closed, but `ActiveUser` in the registry still named the user. `workbench_launch` did not start Steam, and Workbench closed with `SteamAPI_Init failed. Is Steam running?`.
- Now a user counts as logged in only when `steam.exe` runs (`tasklist.exe`) and `ActiveUser` is not 0. The check reads only the CSV row of `steam.exe`, because `tasklist.exe` writes its "no match" message in the language of Windows.
- After the fix, the Tool started Steam and then Workbench. The four checks of #77 passed with `S3C_CameraFix`: downloaded, present, found by `game_find`, and deleted after its line went.

## One input schema for Tools with no inputs

- The input schema of a Tool with no inputs is now one function, `server::no_inputs()`, for `workbench` and `reference-mods`. Before, each Capability had its own copy.
