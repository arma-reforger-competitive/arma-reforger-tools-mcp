---
title: Arma Reforger:String Editor Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:String_Editor_Plugin
---

This tutorial teaches how to create a [Arma Reforger:String Editor](https://community.bistudio.com/wiki/Arma_Reforger:String_Editor)-specific plugin.

**Important:** Please read [Arma Reforger:Workbench Plugin](https://community.bistudio.com/wiki/Arma_Reforger:Workbench_Plugin) before following this tutorial.

## Setup

- Open [Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor)
- In an addon, create a new script in `WorkbenchGame/LocalizationEditor` - name it `TAG_TutorialPlugin.c` (must end with `Plugin` by convention)
- Double-click the file to open it
- Press [Ctrl+T] to use the [Script Template plugin](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Fill_From_Template_Plugin)
  - In its window, select "Class Type: WorkbenchPlugin", **set the parent class to `LocalizationEditorPlugin`** and leave the other fields blank/default
  - A Workbench plugin skeleton is inserted.
- In the `WorkbenchPluginAttribute`, replace `wbModules: { "ResourceManager" }` by `wbModules: { "LocalizationEditor" }`
- Make the plugin inherit from `LocalizationEditorPlugin` instead of `WorkbenchPlugin`
- In the `Run()` method, write `Workbench.Dialog("Does it?", "It works!");` (the String Editor not having a log console) and save the file
- Reload Workbench scripts via **Reload WB Scripts** option located in **Plugins → Settings** (default shortcut: [Ctrl+Shift+R])
- The `TAG_TutorialPlugin` plugin should appear in the String Editor's Plugins list, available in the top bar - click on the plugin entry
- The "It works!" modal appears on screen.

## LocalizationEditorPlugin API

**Note:** See the `LocalizationEditorPlugin` class.

## LocalizationEditor Module API

**Note:** See the `LocalizationEditor` class.

The String Editor API allows
In the below code:
- the `GetTable` method is used to grab the table as a `BaseContainerList`
- the `OnSelectionChanged` method is used to Print the currently selected row IDs.

```enforce
[WorkbenchPluginAttribute(name: "Tutorial Plugin", shortcut: "Ctrl+Shift+H", wbModules: { "LocalizationEditor" }, awesomeFontCode: 0xF188)]
class TAG_TutorialPlugin : LocalizationEditorPlugin
{
	[Attribute(defvalue: "", desc: "If no username is provided, the Windows username will be used")]
	protected string m_sAuthorName;

	protected ref array<ResourceName> m_aResourceNames;
	protected ref array<string> m_aFilePaths;

	//------------------------------------------------------------------------------------------------
	override void Run()
	{
		LocalizationEditor stringEditor = Workbench.GetModule(LocalizationEditor);
		array<int> rows = {};
		stringEditor.GetSelectedRows(rows);
		if (rows.IsEmpty())
		{
			Workbench.Dialog("Input Error", "Please enter a valid author name");
			return;
		}

		BaseContainer stringTable = stringEditor.GetTable();
		if (!stringTable)
		{
			Print("No string table opened", LogLevel.WARNING);
			return;
		}

		BaseContainerList items = stringTable.GetObjectArray("Items"); // entries - cannot be empty since rows are selected

		if (Workbench.ScriptDialog("Author", "Please input the wanted author's name for these entries.", this) == 0)
			return; // on Cancel

		m_sAuthorName.Trim();
		if (m_sAuthorName.IsEmpty())
		{
			Workbench.GetUserName(m_sAuthorName);
			if (m_sAuthorName.IsEmpty())
			{
				Workbench.Dialog("Input Error", "Please enter a valid author name");
				return;
			}
		}

		BaseContainer item;
		string itemId, itemAuthor;

		stringEditor.BeginModify("RowEdit");	// prepare the Undo operation

		foreach (int row : rows)
		{
			item = items.Get(row);
			item.Get("Id", itemId);
			item.Get("Author", itemAuthor);

			Print("Changing #" + itemId + "'s Author from '" + itemAuthor + "' to '" + m_sAuthorName + "'", LogLevel.NORMAL); // log for manual Undo if needed
			stringEditor.ModifyProperty(item, item.GetVarIndex("Author"), m_sAuthorName);
		}

		stringEditor.EndModify();				// wrap the Undo operation

		// stringEditor.Save();					// we allow the user to save, let's not override data ourselves
	}

	//------------------------------------------------------------------------------------------------
	override void OnSelectionChanged()
	{
		LocalizationEditor stringEditor = Workbench.GetModule(LocalizationEditor);
		array<int> rows = {};
		stringEditor.GetSelectedRows(rows);

		Print("Selected rows: " + rows, LogLevel.NORMAL);
	}

	//------------------------------------------------------------------------------------------------
	[ButtonAttribute("OK", true)]
	bool ButtonOK()
	{
		return true;
	}

	//------------------------------------------------------------------------------------------------
	[ButtonAttribute("Cancel")]
	bool ButtonCancel()
	{
		return false;
	}
}
```
