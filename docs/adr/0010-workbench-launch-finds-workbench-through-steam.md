# workbench_launch finds Workbench through Steam and adds -addonsDir

The Tool `workbench_launch` starts Workbench. It finds the Workbench program and the game folder through Steam, with no path setting. It starts the program itself, with the game folder as the work folder, and adds `-addonsDir` for the folder of the downloaded mods. The Workbench bridge still never starts Workbench (ADR 0009).

ADR 0009 named three costs of a start: a path setting or a guess for the program, a project to open, and the dependencies. Each one now has an answer that needs no guess:

- **The program and the game.** Steam knows both. The registry names the Steam folder, `libraryfolders.vdf` names the library of each app, and `appmanifest_<app>.acf` names its folder. When an app is missing, the Tool gives the `steam://install/<app>` link.
- **The project.** The project list of Workbench names the user's mods. The Tool takes the one mod, or the mod that Claude names, and never a packed project (the game or a Workshop mod).
- **Steam.** Workbench needs Steam with a logged-in user, and it does not start Steam itself: on Workbench 1.6 without Steam it logs `SteamAPI_Init failed. Is Steam running?` and closes. While a user is logged in, `ActiveUser` in `HKCU\Software\Valve\Steam\ActiveProcess` is not 0. When it is 0, the Tool starts `steam.exe -silent` from the Steam folder and waits for a user.
- **The dependencies.** A start with `-gproj` looks only in the `addons` folders of the game and of Documents. On Workbench 1.6, BridgeTest with a dependency in `D:\DownloadedMods\addons` did not load (`dependency '672B195EAD3036D4' can't be added`). With `-addonsDir D:\DownloadedMods\addons` it loaded. The folder comes from `REFORGER_WORKBENCH_PROFILE_PATH`, the setting that `game_find` already reads, so no links are needed.

## Considered

- **Start through Steam** (`steam.exe -applaunch 1874910 -gproj ...`). Steam adds the user's launch options, but it sets the Workbench folder as the work folder, and Workbench then does not find the game (`Game addon '58D0FB3206B6F859' not found`). The Steam option `-addonDownloadDir` also does not add the download folder to the search folders.
- **Read the launch options from Steam** (`userdata/<user>/config/localconfig.vdf`). This needs the active Steam user and a parser for one more Steam file, and it gives the same folder as the setting.

## Consequences

- A user who moved the folder of the downloaded mods sets `REFORGER_WORKBENCH_PROFILE_PATH`, or a mod with Workshop dependencies does not load. The Tool then gives the errors of the Workbench log and names the setting.
- The Tool checks after each start that the mod loaded (`GetLoadedProjects`).
