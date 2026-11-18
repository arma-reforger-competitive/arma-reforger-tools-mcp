# workbench

The Workbench bridge: it calls the Handler script of this plugin in a Workbench that already runs, through the Workbench NET API. When the Handler script is missing or old, it copies the new one into the open mod and asks Workbench to compile the scripts again. It never starts or stops Workbench, never polls, and never changes a `.gproj` file (ADR 0009). Only the Tool `workbench_launch` starts Workbench (ADR 0010).

## Tools

- `workbench_status`: checks the connection and gets Workbench ready for the Workbench Tools. It has no inputs. It answers one of four states:
  - Workbench is not running: nothing answers on the host and port. The answer says how to start it and names `REFORGER_WORKBENCH_HOST` and `REFORGER_WORKBENCH_PORT`.
  - The Handler script was not loaded. The Tool copied it into the open mod and reloaded the scripts, and now it is loaded.
  - Workbench had an old version of the Handler script. The Tool copied the new one and reloaded the scripts, and now it is loaded.
  - The Handler script is loaded. The answer gives its version and the open mod project, for example `BridgeTest in /mnt/d/ArmaReforgerWorkbench/BridgeTest`.

  The last three answers end with the next step: delete the folder of the Handler script before you pack or publish the mod. "Not running" is an error for the other Workbench Tools, but for `workbench_status` it is a normal answer: this Tool reports the state.

  Other failures are errors, each with the next step: the scripts of the mod do not compile, the reload did not load the Handler script, no answer in 10 seconds, a connection that breaks before the answer, an answer that is not a NET API answer, or no open mod in the project list. The Tool writes a file, so it is not read-only, and it is destructive: it overwrites an old Handler script.

- `workbench_launch`: starts Workbench with one mod open and waits until the scripts are compiled (at most 180 seconds). Its input `mod` is the ID or the folder name of a mod in the project list, in any case. It answers:
  - Workbench runs already: it starts nothing and gives the answer of `workbench_status`.
  - It started Workbench with the mod, and the mod is loaded: the answer of `workbench_status`, after "Workbench started with the mod ...".
  - The project list has no mod: it starts the Workbench launcher, so the user can create or add a mod.

  When no user is logged in to Steam, it starts Steam first and waits for a login (at most 120 seconds), because Workbench closes without Steam. The answer then starts with "No user was logged in to Steam, so `workbench_launch` started Steam."

  Errors, each with the next step: Steam, Arma Reforger Tools, or the game is not installed (with the `steam://install/<app>` link); PowerShell did not start Steam, or no user logged in to Steam in time; `mod` is not in the project list, or it is empty and the list has more than one mod (the answer lists the mods); PowerShell did not start Workbench; the NET API did not answer in time; the scripts did not compile in time; or Workbench did not load the mod. In the last case the answer gives the errors of the Workbench log, for example `Addon 'BridgeTest' dependency '672B195EAD3036D4' can't be added`.

## Settings

Set them in the `env` of Claude Code's `settings.json`. A path works in Windows form (`D:\...`) and in WSL form (`/mnt/d/...`). Restart Claude Code after a change.

- `REFORGER_WORKBENCH_HOST`: the computer that runs Workbench. Without it, the bridge uses `127.0.0.1` on Windows. Under WSL, it uses the Windows host, the gateway of the default route in `/proc/net/route`, for example `172.17.240.1`. Workbench listens on all addresses, so no port-forward rule is needed.
- `REFORGER_WORKBENCH_PORT`: the Net API port in Workbench (File > Options > General). Without it, the bridge uses 5775. The server checks the value when it starts, and every Workbench Tool answers a wrong value with an error.
- `REFORGER_WORKBENCH_DOCUMENTS_PATH`: the Workbench folder in Documents, the folder that holds `profile` and `logs`, for example `C:\Users\<you>\Documents\My Games\ArmaReforgerWorkbench`. Without it, the bridge uses that folder of the Windows user. Under WSL, it looks in every user folder in `/mnt/c/Users` and uses the one folder that has it. This is not `REFORGER_WORKBENCH_PROFILE_PATH` of `game-files`: that setting names the folder of the mods that Workbench downloaded, and a user can move that folder out of Documents. When the user did not move it, both settings name the same folder.

- `REFORGER_WORKBENCH_PROFILE_PATH`: the folder of the Workshop mods that Workbench downloaded, the folder that holds `addons`, for example `D:\DownloadedMods`. `game-files` reads the same setting. `workbench_launch` adds `-addonsDir <folder>\addons` to the start, so Workbench finds the Workshop dependencies of the mod. Without it, Workbench looks only in the `addons` folders of the game and of the Workbench folder in Documents.

`workbench_launch` needs no path setting for Workbench: it finds Workbench and the game through Steam.

## How it works

- **A call.** Each call opens a new TCP connection, sends one request, and reads one answer (the wiki page "Workbench NET API" has the format). `wire.rs` writes and reads the bytes, and `net.rs` makes the call with a time limit: 3 seconds to connect, 10 seconds for the answer. Under WSL, the Windows firewall can drop a connection to a closed port, so a connect that times out also means "not running". A connection that breaks after the connect is not "not running": a port-forward rule can take the connection for a Workbench that does not run.
- **The Handler script.** `ReforgerBridge.c` is one Enforce Script class, `ReforgerBridge`, that implements `NetApiHandler`. The bridge calls it with `"APIFunc": "ReforgerBridge"` and an `action`. Each answer has the `version` of the script. `ping` is the only action. Later Workbench Tools add their actions to this file. `VERSION` in the file must be the version in `Cargo.toml`: a test checks it.
- **The open project.** The built-in `GetLoadedProjects` gives only the IDs of the loaded projects, for example `["ArmaReforger", "BridgeTest"]`, with the game and the dependencies. The bridge reads the project list of Workbench, `profile/.projectList_*.conf` in the Documents folder, and the `ID`, `GUID`, and `Dependencies` of each `.gproj` in it. The open project is the listed project whose ID is loaded, that is not packed, and that no other loaded listed project needs. A packed project has `.pak` files in its folder: the game and the mods from the Workshop. The bridge never writes into them. A test on Workbench 1.6 showed why: when a mod does not load, Workbench loads only the game, and the game is in the project list. A mod that is not in the project list is not found: the answer says to add it with Add Existing in the Workbench launcher.
- **The install.** The bridge copies the Handler script to `Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c` in the open project. It writes only in that folder: first a temp file, then a rename. Then it calls the built-in `ReloadScripts`, which answers when the compile ends (at most 120 seconds), and calls the Handler script again. Workbench does not load a new script file by itself.

- **The start.** `steam.rs` reads the Steam folder from the Windows registry (`HKCU\Software\Valve\Steam`, value `SteamPath`). Then `steamapps/libraryfolders.vdf` names the library of Arma Reforger Tools (app 1874910) and of the game (app 1874880), and `steamapps/appmanifest_<app>.acf` names the folder of each. When `ActiveUser` in `HKCU\Software\Valve\Steam\ActiveProcess` is 0, no user is logged in: `launch.rs` starts `<Steam folder>\steam.exe -silent` and checks every 2 seconds until it is not 0. Then it copies the Handler script into the mod, so the first compile loads it. Then it runs `Start-Process` of the PowerShell of Windows: the program `Workbench\ArmaReforgerWorkbenchSteamDiag.exe`, the game folder as the work folder, and `-gproj <the .gproj file>` (and `-addonsDir`, see Settings). Workbench keeps running when the server stops. The Tool asks `IsWorkbenchRunning` every 2 seconds until it answers `"ScriptsCompiled": true`, then asks `GetLoadedProjects` whether the mod is loaded.
- **Why not through Steam.** `steam.exe -applaunch 1874910` adds the launch options of the user, but its work folder is the Workbench folder, and Workbench then does not find the game (`Game addon '58D0FB3206B6F859' not found`). The Steam launch option `-addonDownloadDir` does not add the folder of the downloaded mods to the search folders either. So the Tool starts the program itself and adds `-addonsDir`.
- **Times on Workbench 1.6, from WSL.** From the call to a loaded mod with compiled scripts: 8 to 15 seconds. A call when Workbench runs: about 1 second.

## For other Workbench Tools

`Bridge::call(action, params)` in `bridge.rs` is the one function that a Workbench Tool uses. It sends an action to the Handler script and returns its JSON answer and what it did to the Handler script first. When the Handler script is missing or old, it copies and reloads it, then calls again. A failure is an `Error` with the same text as in `workbench_status`.

## Keep the handler out of your mod

The Handler script goes into your mod when Workbench packs or publishes it: a pack of the test mod `BridgeTest` had `Scripts/WorkbenchGame/ReforgerBridge/ReforgerBridge.c` in its `data.pak`. Workbench has no setting to leave a folder out of a pack.

- Before you pack or publish, delete the folder `Scripts/WorkbenchGame/ReforgerBridge/` from your mod.
- Add this line to the `.gitignore` of your mod, so the Handler script does not go into git:

  ```gitignore
  Scripts/WorkbenchGame/ReforgerBridge/
  ```
