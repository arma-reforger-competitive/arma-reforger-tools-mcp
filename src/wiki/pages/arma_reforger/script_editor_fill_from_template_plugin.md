---
title: Arma Reforger:Script Editor: Fill From Template Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Fill_From_Template_Plugin
---

- **name**: Fill from Template
- **editor**: script
- **descr**: A plugin to help create a script skeleton based on templates
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/ScriptEditor/SCR_ScriptTemplatePlugin.c`
- **shortcut**: [Ctrl+T]
**Fill from Template** is a plugin that allows adding content to a file (empty or not) based on the selected template.
This is useful to gain time by automatically preparing a class to be e.g a Workbench plugin and respect standards (e.g ending with "Component" for a component).

## Parameters

- **Class Type** - can be one of:
  - `None` - a normal class
  - `Entity` - a [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) entity
  - `Component` - a scripted component
  - `WidgetComponent` - a widget component
  - `ScriptInvoker` - a [script invoker](https://community.bistudio.com/wiki/Arma_Reforger:ScriptInvoker_Usage)
  - `ScriptedUserAction` - a user action script
  - `UIMenu` - a UI menu script
  - `ConfigRoot` - a config
  - `WorkbenchPlugin` - a Workbench plugin (any editor)
  - `WorldEditorTool` - a [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) tool

**Note:** This type is defined by the `SCR_EScriptTemplateType` enum.

- **Class Name** - if left empty, it will use the file name
- **Parent Name** - if left empty, the class does not inherit from anything unless the template mentions an inheritance

**Note:** See the Template config (`enfusion://ResourceManager/~ArmaReforger:Configs/Workbench/ScriptTemplatePlugin/ScriptTemplateConfig.conf`, an array of `SCR_ScriptTemplateConfigEntry`) to know more about Template configuration.

## See Also

- [Arma Reforger:Script Editor: Create New Script Plugin](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Create_New_Script_Plugin)
- [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor)
