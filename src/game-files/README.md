# game-files

Finds and opens Game files: the scripts, prefabs, configs, and other files inside the .pak archives of the installed game and of the Workshop mods that Workbench downloaded.

## Tools

- `game_find`: finds Game files by a part of their name or path, or by a class name, for example `CharacterCameraHandler` or `GameMode/SCR_`. Capital letters do not matter. It returns the real paths, for example `scripts/Game/Character/CharacterCameraHandlerComponent.c`, the file that defines `SCR_CharacterCameraHandlerComponent`.
  - Order: the full file name (with or without the extension), then names that start with the query, then names that have it inside, then folder paths that have it inside. Game files come before mod files, and shorter paths before longer ones.
  - A class name that starts with `SCR_` also finds the file without the prefix: `SCR_CharacterCameraHandlerComponent` finds `CharacterCameraHandlerComponent.c` first.
  - Each path says where it comes from: `(game)` or `(mod tvtLobby)`.
  - `limit` sets how many paths the list shows, from 1 to 100 (default 20). A longer list is cut, and the first line says how many match.
  - When no file matches, the answer says how many files it searched, in the game and in which mods.
  - An empty query gives an error with an example.
- `game_read`: opens one Game file by its `path` and returns its full text. The first line gives the real path, the source, and the size in bytes, for example `scripts/Game/Character/CharacterCameraHandlerComponent.c (game, 43980 bytes)`.
  - Capital letters do not matter: `Scripts/Game/Character/CharacterCameraHandlerComponent.c` opens the file and gives the real path, `scripts/...`.
  - A path that no file has gives the 5 closest real paths. The file name counts most, and a `SCR_` prefix that the file name does not have is ignored. Example: `scripts/Game/Characters/SCR_CharacterControllerComponentt.c` gives `scripts/Game/Character/SCR_CharacterControllerComponent.c` first.
  - A path copied from `game_find` with its source, for example `Configs/Shared.conf (mod tvtLobby)`, works. So do back slashes, and a Prefab path with its `{GUID}` in front.
  - When the same path is in the game and in mods, `source` says which one to open: `game` or a mod name, for example `tvtLobby`. Without it, the game opens first, and the answer names the other sources.
  - The game has 34 paths in two .pak files, for example `Prefabs/AI/SCR_AIWorld.et` in `data/data007.pak` and `rally/data.pak`. The first .pak file in folder order opens, and the answer names both .pak files.
  - A file with a zero byte in its first 8,000 bytes is binary, for example a texture. It gives an error, not its bytes.
  - The server cuts an answer at 100,000 characters, and the end says so: "(Cut at 100,000 of N characters.)". A file with a full size above 64 MB gives an error before it is unpacked.

## Settings

Set them in the `env` of Claude Code's `settings.json`. A path works in Windows form (`D:\Games\...`) and in WSL form (`/mnt/d/Games/...`). Restart Claude Code after a change.

- `REFORGER_GAME_PATH`: the game folder, the folder that holds `ArmaReforger.exe` and `addons`. Without it, or when it has no `addons` folder with .pak files, `game_find` and `game_read` give an error that names the setting.
- `REFORGER_WORKBENCH_DOCUMENTS_PATH`: the Workbench folder in Documents, for example `C:\Users\<you>\Documents\My Games\ArmaReforgerWorkbench`. Workbench downloads Workshop mods to its `addons` folder. Without it, `game-files` finds the folder the same way as `workbench` (`src/workbench_documents.rs`).
- `REFORGER_WORKBENCH_PROFILE_PATH`: a second folder that holds `addons` with Workshop mods, for example `D:\DownloadedMods`, the folder where the game downloads mods. Without it and without a Documents folder, `game_find` and `game_read` use the game only, and `game_find` says so.

## Data

The file lists come from the FILE table of each .pak archive:

- The game: every `.pak` file in `<game>/addons` and one folder down, for example `addons/data/data007.pak`. The game has 19 archives with about 251,000 files.
- The mods: every `.pak` file in `<Documents folder>/addons/<Name>_<ID>/` and in `<profile>/addons/<Name>_<ID>/`. The mod name is the folder name without the 16-digit ID. When the same mod ID is in both folders, the mod is listed once, from the Documents folder.

The first `game_find` or `game_read` call reads all file lists, outside the server thread, in about a second. Before each later call, `game-files` lists the mod folders in both `addons` folders. When they differ from the folders at the last build, it reads all file lists again, so a mod that Workbench downloads shows with no restart. A damaged .pak file is skipped, and the answer names it.

`pak.rs` reads the FILE table and the content of one file. It is translated from `src/pak/reader.ts` and `src/pak/vfs.ts` of enfusion-mcp, with three changes:

- The `offset` of a file is a position from the start of the .pak file, not from the DATA chunk.
- A compressed file is zlib with a header, so it is unpacked with zlib `inflate`, not raw deflate. The old raw deflate failed with "invalid block type" on most scripts.
- Every size, offset, child count, folder depth, and path length is checked against the file length or a limit before the reader uses it. A damaged file gives an error, never a crash.
