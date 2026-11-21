---
name: workbench-test
description: Test a Workbench Tool on the real Workbench of the owner - start it with a test mod, send raw NET API calls, and close it. Use when a change to src/workbench/, or to another Tool that calls Workbench, needs a test on the real Workbench, or when a Workbench Tool gives an answer you must debug.
---

This skill holds only what a user of the plugin does not need. To start Workbench and to check its state, use the Tools of the plugin, the same way a user does.

## Steps

1. **Build.** Run `cargo build --release`. The Tools below run from `target/release/reforger`.

2. **Start Workbench with the test mod.** Ask the owner before you start Workbench. Use the test mod `BridgeTest` (`D:\ArmaReforgerWorkbench\BridgeTest`), not a real mod of the owner.

   ```sh
   cargo xtask call-tool workbench_launch mod=BridgeTest
   ```

   The step is done when the answer says the mod is loaded. On Workbench 1.6 this takes 8 to 15 seconds, and longer when the Tool starts Steam first. When Workbench runs already, the Tool starts nothing. Then check with `workbench_status` which mod is open.

3. **Test.** Call the Tool that you changed with `cargo xtask call-tool`, as in step 2. To see what Workbench itself answers, send a raw call (see "Raw NET API calls").

4. **Clean up.** Put back each file of `BridgeTest` that the test changed. When the mod did not load, the bridge could write into the game. Check that `D:\Games\steamapps\common\Arma Reforger\addons\data` has no `Scripts` folder.

5. **Close Workbench.** Close only a Workbench that you started in step 2. A Workbench that ran before you started can have unsaved work of the owner: ask the owner first.

   ```sh
   powershell.exe -NoProfile -Command "Get-Process | Where-Object { \$_.ProcessName -like 'ArmaReforgerWorkbench*' } | ForEach-Object { \$_.CloseMainWindow() | Out-Null }; Start-Sleep 20; Get-Process | Where-Object { \$_.ProcessName -like 'ArmaReforgerWorkbench*' } | Stop-Process -Force; 'closed'" | tr -d '\r'
   ```

   It asks Workbench to close, and stops it by force after 20 seconds. No Tool does this, because a forced stop loses unsaved work.

## Tool calls

`cargo xtask call-tool <tool> [name=value ...]` starts `target/release/reforger`, calls one Tool as an MCP client, and prints only the text of the answer. Each input is one `name=value`. It exits with 1 when the answer is an error.

- It passes every `REFORGER_*` setting of the `env` in `~/.claude/settings.json`, as Claude Code does. A `REFORGER_*` variable in the shell wins.
- It sends the value of a text input as it is. It reads the value of another input as JSON, for example a number or `true`, and sends the text when the value is not JSON.
- It lets one call run 300 seconds, which is longer than `workbench_launch` can take.

Test `reference_sync` with test lists, never with the owner's mod repos: put a `reference-mods.jsonl` in a subfolder of a temp folder, and set `REFORGER_MODS_PATH` to the temp folder.

```sh
mkdir -p /tmp/ref-test/TestRepo
echo '{"id":"6A09759015FFDA26","name":"S3C_CameraFix"}' > /tmp/ref-test/TestRepo/reference-mods.jsonl
REFORGER_MODS_PATH=/tmp/ref-test cargo xtask call-tool reference_sync
```

## Raw NET API calls

`cargo xtask net-api` sends one call to the Workbench NET API and prints the status and the JSON answer. It uses the same settings and the same bytes as the bridge: it builds `src/workbench/wire.rs` and `src/wsl.rs` of the server. It exits with 1 when the status is not `Ok`. No Tool does this, because a call to any function is too open for users.

```sh
cargo xtask net-api IsWorkbenchRunning
cargo xtask net-api GetLoadedProjects
cargo xtask net-api ReforgerBridge '{"action":"ping"}'
cargo xtask net-api ReloadScripts
```

`ReloadScripts` answers when the compile ends. Workbench does not load a new script file without it.

## Facts

- Under WSL, Workbench is on the Windows host: the gateway of the default route, for example `172.17.240.1`. `cargo xtask net-api` finds it by itself.
- A closed port times out after 3 seconds. Windows does not refuse the connection.
- The project list of Workbench is `profile/.projectList_*.conf` in `C:\Users\<you>\Documents\My Games\ArmaReforgerWorkbench`. It names the `.gproj` file of each mod.
- The Workbench log is in `logs/` of the same folder. When a mod does not load, the log names the reason, for example a missing dependency.
- `src/workbench/README.md` explains the bridge, the start, and the times.
