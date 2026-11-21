---
name: create-mod
description: Start a new Arma Reforger mod from an empty folder - the Workbench project file (.gproj) with a new GUID, the folder layout, the class name Tag, and the first open in Workbench. Use when the user wants to create, start, or set up a new Reforger mod or Workbench project.
---

A new mod is a folder with one project file, `addon.gproj`. Workbench makes every other file itself when it opens the mod. This skill writes the project file and the files for git, then opens the mod in Workbench.

The Tools come from the `reforger` MCP server of this plugin: `guid_generate`, `game_find`, `game_read`, `wiki_search`, `wiki_read`, and `workbench_launch`.

## Steps

1. **Check the server.** The GUID of the mod must come from `guid_generate`. When `guid_generate` is not in your Tools, stop before you write a file, and tell the user:

   > The `reforger` server is not connected, so I cannot make a GUID for the mod. Run `claude mcp list` in a terminal. The line `plugin:reforger:reforger` must show `Connected`. If it does not, run `cargo build --release` in the plugin folder and restart Claude Code.

   Make a GUID only with `guid_generate`, and wait until the user connects the server. A GUID from any other place can be the same as the GUID of another mod. The step is done when `guid_generate` is in your Tools.

2. **Ask the user for the four values.** Ask in one message, and give a default for each:
   - **Name**: the title of the mod. Letters, numbers, spaces, `-`, `_`, and `.` only.
   - **ID**: the Name without spaces, for example `Lanka Core` gives `LankaCore`. This is what Workbench does too. Other tools and the `-addons` start parameter use the ID.
   - **Tag**: 2 to 5 capital letters and `_`, for example `LNK_`. Every class name and script file name of the mod starts with it, so two mods do not use the same class name (wiki page "Arma Reforger:Scripting: Conventions", Section "Tag"). `SCR_` is the Tag of Bohemia Interactive.
   - **Folder**: the default is `%userProfile%\Documents\My Games\ArmaReforgerWorkbench\addons\<Name>`. Use a folder that the user can write to, not in OneDrive or Google Drive, and not in the folder of downloaded Workshop mods: Workbench opens a mod in those folders as read-only. Under WSL, `D:\Mods\X` is `/mnt/d/Mods/X`.

   The step is done when the user has accepted or changed each of the four values.

3. **Check the Tag.** Call `game_find` with the Tag, for example `LNK_`. When the game or a downloaded mod has files that start with the Tag, tell the user which ones, and ask for another Tag. The step is done when `game_find` finds no file for the Tag.

4. **Check the folder.** The folder must be empty or missing. When it has files, show them to the user and ask what to do. Make the folder when it is missing.

5. **Make the GUID.** Call `guid_generate` one time. Its answer is the GUID: 16 hex characters in upper case. Use this answer as it is.

6. **Write the files.** Copy each template from the `templates/` folder of this skill into the mod folder, and put in the values:
   - `templates/addon.gproj` becomes `addon.gproj`. Replace `{{ID}}`, `{{GUID}}`, and `{{NAME}}`. The one dependency, `58D0FB3206B6F859`, is the GUID of Arma Reforger. Every mod needs it.
   - `templates/gitignore` becomes `.gitignore`. It keeps the Handler script of this plugin out of git (see `src/workbench/README.md` of the plugin).
   - Make an empty file `reference-mods.jsonl`. It is the list of Workshop mods that the user wants Claude to study. `reference_sync` downloads them. One line is one JSON object, for example `{"id":"6AA81EE549F50CB7","name":"tvtLobby"}`.

   The step is done when the folder has these three files and `addon.gproj` has no `{{`.

7. **Add the mod to the Workbench project list.** `workbench_launch` opens only a mod that is in this list. Ask the user to do this in the Workbench launcher, and wait until they say it is done:

   > Start Arma Reforger Tools from Steam. In the Workbench launcher, click **Add Existing** and select `<Folder>\addon.gproj`. Then close the launcher.

8. **Open the mod in Workbench.** Call `workbench_launch` with `mod` set to the ID. The step is done when the answer says that the mod is loaded. When the answer gives errors from the Workbench log, fix the cause and call again. For example, `dependency '<GUID>' can't be added` means that a dependency is not in the project list, or that `Dependencies` has an empty string `""`.

9. **Tell the user.** Give the folder, the ID, the GUID, and the Tag, and the rules below that they need for their first script.

## Where things go

- **Scripts** go in `Scripts/Game/`. Workbench reads scripts only from the folders of its script modules: `Scripts/Core`, `Scripts/GameLib`, `Scripts/Game`, `Scripts/GameCode`, `Scripts/Workbench`, and `Scripts/WorkbenchGame`. It ignores a script in any other folder, with no error (wiki page "Arma Reforger:Scripting Example", Section "Create the Script").
- **Names**: a script file and its class both start with the Tag, for example `Scripts/Game/LNK_PlayerSlot.c` with `class LNK_PlayerSlot`. When the mod changes a game class with `modded class`, the new methods and variables of that class start with the Tag too.
- **A Workshop mod as a dependency**: add its GUID to `Dependencies` in `addon.gproj`. The folder of a downloaded Workshop mod is named `<Name>_<GUID>`. The mod must also be in the project list, and so must its own dependencies, or Workbench opens the game without the mod.

For other questions about the project, read the wiki page "Arma Reforger:Mod Project Setup" with `wiki_read`. To see how other mods do a thing, find their files with `game_find`, and read them with `game_read`.
