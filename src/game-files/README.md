# game-files

Finds Game files: the scripts, prefabs, configs, and other files inside the .pak archives of the installed game and of the Workshop mods that Workbench downloaded.

## Tools

- `game_find`: finds Game files by a part of their name or path, or by a class name, for example `CharacterCameraHandler` or `GameMode/SCR_`. Capital letters do not matter. It returns the real paths, for example `scripts/Game/Character/CharacterCameraHandlerComponent.c`, the file that defines `SCR_CharacterCameraHandlerComponent`.
  - Order: the full file name (with or without the extension), then names that start with the query, then names that have it inside, then folder paths that have it inside. Game files come before mod files, and shorter paths before longer ones.
  - A class name that starts with `SCR_` also finds the file without the prefix: `SCR_CharacterCameraHandlerComponent` finds `CharacterCameraHandlerComponent.c` first.
  - Each path says where it comes from: `(game)` or `(mod tvtLobby)`.
  - `limit` sets how many paths the list shows, from 1 to 100 (default 20). A longer list is cut, and the first line says how many match.
  - When no file matches, the answer says how many files it searched, in the game and in which mods.
  - An empty query gives an error with an example.

## Settings

Set them in the `env` of Claude Code's `settings.json`. A path works in Windows form (`D:\Games\...`) and in WSL form (`/mnt/d/Games/...`). Restart Claude Code after a change.

- `REFORGER_GAME_PATH`: the game folder, the folder that holds `ArmaReforger.exe` and `addons`. Without it, or when it has no `addons` folder with .pak files, `game_find` gives an error that names the setting.
- `REFORGER_WORKBENCH_PROFILE_PATH`: the Workbench profile folder, for example `C:\Users\<you>\Documents\My Games\ArmaReforgerWorkbench`. Without it, `game_find` searches the game only and says so. This is not `REFORGER_WORKBENCH_DOCUMENTS_PATH` of `workbench`: this setting names the folder that holds `addons`, the mods that Workbench downloaded, and a user can move it out of Documents.

## Data

The file lists come from the FILE table of each .pak archive:

- The game: every `.pak` file in `<game>/addons` and one folder down, for example `addons/data/data007.pak`. The game has 19 archives with about 251,000 files.
- The mods: every `.pak` file in `<profile>/addons/<Name>_<ID>/`. The mod name is the folder name without the 16-digit ID.

The first `game_find` call reads all file lists, outside the server thread, in about a second. The server keeps them until it stops, so a mod that you download later shows after a restart. A damaged .pak file is skipped, and the answer names it.

`pak.rs` reads the FILE table. It is translated from `src/pak/reader.ts` of enfusion-mcp, with two changes:

- The `offset` of a file is a position from the start of the .pak file, not from the DATA chunk.
- Every size, offset, child count, folder depth, and path length is checked against the file length or a limit before the reader uses it. A damaged file gives an error, never a crash.
