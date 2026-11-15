---
title: Arma Reforger:Script Editor: Create New Script Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Create_New_Script_Plugin
---

- **name**: Create New Script
- **editor**: script
- **descr**: A plugin to help create a new file
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/ScriptEditor/SCR_NewScriptPlugin.c`
- **shortcut**: [Ctrl+N]
**Create New Script** is a plugin that fulfills the "Create New Script" feature for the [Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor).

## Parameters

- **Addon** - in which addon the file will be created
- **Destination Directory** - in which *directory tree* the file will be created:<br>It is possible to select an "ArmaReforger" directory to create the file and the directory tree in the selected addon, e.g
```bat
Addon: MyAddon
Selected directory: $ArmaReforger:scripts/Game/Building
End destination: $MyAddon:scripts/Game/Building
```

- **Parent Class Name** - the parent class name, if any - it may be overridden by config's parent class name
- **Type** - the script type (same as [Fill From Template's Type](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Fill_From_Template_Plugin#Parameters))
- **Template Config File** - the template config to use, storing the different types's templates

## See Also

- [Arma Reforger:Script Editor: Fill From Template Plugin](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Fill_From_Template_Plugin)
- [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor)
