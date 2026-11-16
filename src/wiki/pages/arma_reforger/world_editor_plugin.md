---
title: Arma Reforger:World Editor Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:World_Editor_Plugin
---

*Not to be confused with [Arma Reforger:World Editor Tool](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor_Tool).*

This tutorial teaches how to create a [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor)-specific plugin.

**Important:** Please read [Arma Reforger:Workbench Plugin](https://community.bistudio.com/wiki/Arma_Reforger:Workbench_Plugin) before following this tutorial.

## Setup

- Open Script Editor
- In an addon, create a new script in `WorkbenchGame/WorldEditor` - name it `TAG_TutorialPlugin.c` (must end with `Plugin` by convention)
- Double-click the file to open it
- Press [Ctrl+T] to use the [Script Template plugin](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Fill_From_Template_Plugin)
  - In its window, select "Class Type: WorkbenchPlugin", leave the other fields blank/default
  - A Workbench plugin skeleton is inserted.
- In the `WorkbenchPluginAttribute`, replace `wbModules: { "ResourceManager" }` by `wbModules: { "WorldEditor" }`
- Make the plugin inherit from `WorldEditorPlugin` instead of `WorkbenchPlugin`
- In the `Run()` method, write `Print("It works!");` and save the file
- Unlike other plugins, **Rebuild Scripts** using the [Shift+F7] default shortcut
- Reload Workbench scripts via **Reload Scripts** option located in **Plugins → Settings** (default shortcut: [Ctrl+Shift+R])
- The `TAG_TutorialPlugin` plugin should appear in the World Editor's Plugins list, available in the top bar - click on the plugin entry
- "It works!" gets printed in the output console.

## WorldEditorPlugin API

**Note:** See the `WorldEditorPlugin` class.

The WorldEditorPlugin class only exposes some events on which it is possible to subscribe (e.g `OnGameModeEnded()`).

## WorldEditor Module API

**Note:** See the `WorldEditor` class.

The WorldEditor module allows to do some minor operations (e.g get Resource Browser's selection, get current world's boundaries).

```enforce
vector min, max;
WorldEditor worldEditor = Workbench.GetModule(WorldEditor);
if (!worldEditor)
	return;

worldEditor.GetWorldBounds(min, max);
Print("Min = " + min);
Print("Max = " + max);
```

The meat of the matter is happening in `WorldEditorAPI`, available through the `WorldEditor.GetApi()` method - see below.

## WorldEditorAPI API

**Note:** See the `WorldEditorAPI` class as well as [Arma Reforger:WorldEditorAPI Usage](https://community.bistudio.com/wiki/Arma_Reforger:WorldEditorAPI_Usage).

## Example

In this example, we will use `WorldEditorAPI` methods to hide/show all selected entities (it will hide a shown entity and vice versa):

```enforce
#ifdef WORKBENCH
[WorkbenchPluginAttribute(name: "Tutorial Plugin", shortcut: "Ctrl+Shift+H", wbModules: { "WorldEditor" }, awesomeFontCode: 0xF188)]
class TAG_TutorialPlugin : WorldEditorPlugin
{
	override void Run()
	{
		WorldEditor worldEditor = Workbench.GetModule(WorldEditor);
		if (!worldEditor)
			return;

		WorldEditorAPI worldEditorAPI = worldEditor.GetApi();
		IEntitySource entitySource;

		worldEditorAPI.BeginEntityAction();
		for (int i = worldEditorAPI.GetSelectedEntitiesCount() - 1; i >= 0; --i)
		{
			entitySource = worldEditorAPI.GetSelectedEntity(i);
			if (!entitySource)
				continue;

			worldEditorAPI.SetEntityVisible(entitySource, !worldEditorAPI.IsEntityVisible(entitySource), false);
		}
		worldEditorAPI.EndEntityAction();
	}
}
#endif
```

**Note:** For a more advanced `WorldEditorAPI` usage, see [Arma Reforger:World Editor Tool](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor_Tool).
