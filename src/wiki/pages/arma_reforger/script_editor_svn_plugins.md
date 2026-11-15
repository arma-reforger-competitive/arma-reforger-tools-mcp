---
title: Arma Reforger:Script Editor: SVN Plugins
url: https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_SVN_Plugins
---

- **name**: SVN Plugins
- **editor**: resource script
- **descr**: VCS-related shortcut commands
- **shortcut**:

| SVN Blame | [Alt+Shift+B]<br>*[Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor) only* |
| --- | --- |
| **SVN Diff** | [Alt+Shift+I]<br>*[Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor) only* |
| **SVN Log** | [Alt+Shift+L] |

- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/ScriptEditor/SCR_SVNPlugin.c`

SVN plugins are keyboard shortcuts to SVN commands - more precisely [TortoiseSVN](https://tortoisesvn.net/) commands by default.
They are defined in `enfusion://ScriptEditor/scripts/WorkbenchGame/ScriptEditor/SCR_SVNPlugin.c`.

**Note:** The plugins are named [SVN](https://subversion.apache.org/) but can absolutely be used with another VCS software like [Git](https://git-scm.com/) (e.g [Git Extensions](https://gitextensions.github.io/)), provided commands are replaced in the plugins's options.<br>
They will be renamed to **VCS** plugins in Arma Reforger 1.3.0.

## Commands

Commands can be anything and have two possible parameters:
- `$path`: replaced by the absolute file path between double quotes (`"`), e.g `"C:\Users\John Bob\MyFile.c"`.
- `$line`: replaced by the current cursor position's line number.

| Name | Shortcut | Description | Command |
| --- | --- | --- | --- |
| *SVN* Blame | [Alt+Shift+B] | List the last author of each line, to find the culprit | TortoiseProc /command:blame /path:$path /startrev:1 /endrev:-1 /ignoreeol /ignoreallspaces /line:$line<br>```<br>gitex blame $path<br>``` |
| *SVN* Diff | [Alt+Shift+I] | List the changes between the repository and the local file | TortoiseProc /command:diff /path:$path<br>```<br>gitex difftool $path<br>``` |
| *SVN* Log | [Alt+Shift+L] | Show the file's changes commit history | TortoiseProc /command:log /path:$path<br>```<br>gitex filehistory $path<br>``` |

## See Also

- [SVN](https://subversion.apache.org/)
  - [TortoiseSVN](https://tortoisesvn.net/)
- [Git](https://git-scm.com/)
  - [Git Extensions](https://gitextensions.github.io/)
  - [TortoiseGit](https://tortoisegit.org/)
