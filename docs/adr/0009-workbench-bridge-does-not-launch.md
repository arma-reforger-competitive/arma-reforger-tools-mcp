# The Workbench bridge does not start Workbench, poll, or link mods

The Workbench bridge only calls a Workbench that already runs. When the Handler script is missing or old, it copies the file into the open mod, calls the built-in `ReloadScripts` once, and calls the Handler script again. It does not start or stop Workbench, it does not poll for a recompile, and it makes no dependency links.

The old bridge did all three, and each one cost more than it gave:

- **Start.** Starting Workbench needs the path of the Workbench program, a project to open, the game folder as the work folder, and a wait of up to 90 seconds. Each of these was a setting or a guess, and a wrong one gave a 90-second wait and then an error. The user starts Workbench in a few seconds and knows which mod to open.
- **Poll for a recompile.** The old bridge pinged every 2 seconds for up to 30 seconds and hoped that Workbench loaded the copied script by itself. Workbench 1.6 does not: with the BridgeTest mod, a copied script still gave "Undefined API func" 8 seconds later. One call to `ReloadScripts` loads it: the call answers when the compile ends, and the next ping answers. This worked for a new script, for a changed script, and for a new `Scripts` folder, with no restart.
- **Dependency links.** A Workbench that is started with `-gproj` does not find dependencies outside its search folders, so the old bridge made links to them in the profile folder. The user's own start of Workbench finds them, so the links are not needed when the bridge does not start Workbench.

## Consequences

- The NET API does not say which project is open or where it is: `GetLoadedProjects` gives only IDs, and Workbench 1.6 has no `EvaluateScript`. The bridge reads the project list of Workbench in Documents to find the folder of the open project.
- `ReloadScripts` compiles all scripts of the open mod. When they have errors, the Handler script does not load, and the answer says to fix them.
- The Tool `workbench_launch` starts Workbench, apart from the bridge, and uses the bridge for the calls (ADR 0010).
