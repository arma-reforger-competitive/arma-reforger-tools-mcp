---
title: Arma Reforger:Resource Manager Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager_Plugin
---

This tutorial teaches how to create a [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)-specific plugin.

**Important:** Please read [Arma Reforger:Workbench Plugin](https://community.bistudio.com/wiki/Arma_Reforger:Workbench_Plugin) before following this tutorial.

## Setup

- Open [Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor)
- In an addon, create a new script in `WorkbenchGame/ResourceManager` - name it `TAG_TutorialPlugin.c` (must end with `Plugin` by convention)
- Double-click the file to open it
- Press [Ctrl+T] to use the [Script Template plugin](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Fill_From_Template_Plugin)
  - In its window, select "Class Type: WorkbenchPlugin", **set the parent class to `ResourceManagerPlugin`** and leave the other fields blank/default
  - A Workbench plugin skeleton is inserted.
- In the `Run()` method, write `Print("It works!");` and save the file
- Reload Workbench scripts via **Reload WB Scripts** option located in *Plugins→Settings* menu (default shortcut: [Ctrl+Shift+R])
- The `TAG_TutorialPlugin` plugin should appear in the Resource Manager's Plugins list, available in the top bar - click on the plugin entry
- "It works!" gets printed in the output console.

## Contextual Menu Option

The Resource **Browser**'s Contextual Menu can provide a Plugin option allowing to run code on the selected resource(s) of the defined type(s).

**Note:** The Plugin Contextual Menu is only available in Resource Manager, not any other Workbench module.

In the `WorkbenchPluginAttribute`, the `resourceTypes` parameter must be filled, e.g `resourceTypes: { "et", "c" }`.

The `OnResourceContextMenu` method must be overridden to work with said resources.

```enforce
[WorkbenchPluginAttribute(name: "Tutorial Plugin", wbModules: { "ResourceManager" }, resourceTypes: { "et", "c" })]
class TAG_TutorialPlugin : ResourceManagerPlugin
{
	//------------------------------------------------------------------------------------------------
	override void OnResourceContextMenu(notnull array<ResourceName> resources)
	{
		Print("Resource context menu action has been called! Here are the selected resources:");
		foreach (ResourceName resource : resources)
		{
			if (resource.EndsWith("c"))
				Print("- Script File: " + resource);
			else
				Print("- Prefab File: " + resource);
		}
	}
}
```

## ResourceManager Module API

**Note:** See the `ResourceManager` class.

The Resource Manager API allows to register resources, rebuild them, get the selected ones or get their meta file content.
In the below code:
- the `GetResourceBrowserSelection` method is used to temporarily store Resource Browser-selected resources in `m_aResourceNames` and their file path in `m_aFilePaths`
- the `GetMetaFile` method is used to get the current resource's `.meta` file as a `MetaFile` instance.

```enforce
[WorkbenchPluginAttribute(name: "Tutorial Plugin", wbModules: { "ResourceManager" }, awesomeFontCode: 0xF188)]
class TAG_TutorialPlugin : ResourceManagerPlugin
{
	protected ref array<ResourceName> m_aResourceNames;
	protected ref array<string> m_aFilePaths;

	//------------------------------------------------------------------------------------------------
	override void Run()
	{
		ResourceManager resourceManager = Workbench.GetModule(ResourceManager);
		
		m_aResourceNames = {};
		m_aFilePaths = {};
		resourceManager.GetResourceBrowserSelection(WorkbenchSearchResourcesCallbackMethod, false); // non-recursive search

		if (m_aFilePaths.IsEmpty())
		{
			Print("No Resources selected in Resource Browser");
			return;
		}

		MetaFile metaFile;
		foreach (int i, ResourceName filePath : m_aFilePaths)
		{
			metaFile = resourceManager.GetMetaFile(filePath);
			if (!metaFile)	// e.g directory
				continue;

			PrintFormat(
				"#%1: %3 (dir %2)",
				i,
				metaFile.GetSourceFilePath(),	// returns the directory tree, e.g $MyMod:Path/To/
				metaFile.GetResourceID());		// returns the file's ResourceName, equal to 'resName' below
		}
	}

	//------------------------------------------------------------------------------------------------
	// \param[in] resName
	// \param[in] filePath is in format $MyMod:Path/To/File.ext
	protected void WorkbenchSearchResourcesCallbackMethod(ResourceName resName, string filePath = "")
	{
		m_aResourceNames.Insert(resName);
		m_aFilePaths.Insert(filePath);
	}
}
```
