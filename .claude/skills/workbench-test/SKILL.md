---
name: workbench-test
description: Test a Workbench Tool on the real Workbench of the owner - start it with a test mod, send raw NET API calls, and close it. Use when a change to src/workbench/ needs a test on the real Workbench, or when a Workbench Tool gives an answer you must debug.
---

This skill holds only what a user of the plugin does not need. To start Workbench and to check its state, use the Tools of the plugin, the same way a user does.

## Steps

1. **Build.** Run `cargo build --release`. The Tools below run from `target/release/reforger`.

2. **Start Workbench with the test mod.** Ask the owner before you start Workbench. Use the test mod `BridgeTest` (`D:\ArmaReforgerWorkbench\BridgeTest`), not a real mod of the owner.

   ```sh
   npx -y @modelcontextprotocol/inspector@2.15.0 --cli -e 'REFORGER_WORKBENCH_PROFILE_PATH=D:\DownloadedMods' \
     target/release/reforger --method tools/call --tool-name workbench_launch --tool-arg mod=BridgeTest
   ```

   The step is done when the answer says the mod is loaded. On Workbench 1.6 this takes 8 to 15 seconds. When Workbench runs already, the Tool starts nothing. Then check with `workbench_status` which mod is open.

3. **Test.** Call the Tool that you changed with MCP Inspector, as in step 2. To see what Workbench itself answers, send a raw call (see "Raw NET API calls").

4. **Clean up.** Put back each file of `BridgeTest` that the test changed. When the mod did not load, the bridge could write into the game. Check that `D:\Games\steamapps\common\Arma Reforger\addons\data` has no `Scripts` folder.

5. **Close Workbench.** Close only a Workbench that you started in step 2. A Workbench that ran before you started can have unsaved work of the owner: ask the owner first.

   ```sh
   powershell.exe -NoProfile -Command "Get-Process | Where-Object { \$_.ProcessName -like 'ArmaReforgerWorkbench*' } | ForEach-Object { \$_.CloseMainWindow() | Out-Null }; Start-Sleep 20; Get-Process | Where-Object { \$_.ProcessName -like 'ArmaReforgerWorkbench*' } | Stop-Process -Force; 'closed'" | tr -d '\r'
   ```

   It asks Workbench to close, and stops it by force after 20 seconds. No Tool does this, because a forced stop loses unsaved work.

## Raw NET API calls

`net-api.ts` in this folder sends one call to the Workbench NET API and prints the status and the JSON answer. It uses the same settings and the same bytes as the bridge (`src/workbench/wire.rs`). No Tool does this, because a call to any function is too open for users.

```sh
node .claude/skills/workbench-test/net-api.ts IsWorkbenchRunning
node .claude/skills/workbench-test/net-api.ts GetLoadedProjects
node .claude/skills/workbench-test/net-api.ts ReforgerBridge '{"action":"ping"}'
node .claude/skills/workbench-test/net-api.ts ReloadScripts
```

`ReloadScripts` answers when the compile ends. Workbench does not load a new script file without it.

## Facts

- Under WSL, Workbench is on the Windows host: the gateway of the default route, for example `172.17.240.1`. The script finds it by itself.
- A closed port times out after 3 seconds. Windows does not refuse the connection.
- The project list of Workbench is `profile/.projectList_*.conf` in `C:\Users\<you>\Documents\My Games\ArmaReforgerWorkbench`. It names the `.gproj` file of each mod.
- The Workbench log is in `logs/` of the same folder. When a mod does not load, the log names the reason, for example a missing dependency.
- `src/workbench/README.md` explains the bridge, the start, and the times.
