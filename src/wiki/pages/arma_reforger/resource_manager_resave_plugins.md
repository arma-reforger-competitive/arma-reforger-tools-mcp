---
title: Arma Reforger:Resource Manager: Resave Plugins
url: https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Resave_Plugins
---

- **name**: Re-Save Tool<br>Re-Save Meta Tool
- **editor**: resource
- **descr**: Re-saves all files with provided extensions<br>Re-saves all meta files of provided extensions' files
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchCommon/ResaveTool.c`<br>`enfusion://ScriptEditor/scripts/WorkbenchCommon/ResaveMetaTool.c`
**Re-Save** tools allow to re-save resources or their meta files in a specified directory.

## Usage

Use it with `Plugins > Re-Save (Meta) Tool`; a window appears allowing to set wanted re-save extensions as well as a search parent directory (root path).

## Command Line

Both plugins can be used through command line usage:

```
ArmaReforgerWorkbenchSteam.exe -wbModule=ResourceManager -plugin=ResaveTool pluginArguments
ArmaReforgerWorkbenchSteam.exe -wbModule=ResourceManager -plugin=ResaveMetaTool pluginArguments
```

Both use the same arguments:
- `-addonPath` to specify the **absolute** directory path
- `-extension` to specify the file extension to process; only one extension can be used here
