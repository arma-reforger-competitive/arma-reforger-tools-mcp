---
title: Arma Reforger:Resource Manager
url: https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager
---

Welcome to the **Enfusion Workbench - Resource Manager!**

The Resource Manager is an integral part of the main "hub" of the **Enfusion Workbench**. The Resource Manager is its asset manager, which allows for **importing, managing and editing** assets and their related files. It’s also the place from which navigation to other tools or areas of Workbench can be done, as well as accessing project and Workbench **options**, **build data**, **package**, **build project** and more.

### Interface

By default, **Resource Manager** welcomes you with 4 main sections:

1. **Resource Browser** - as its name suggests, this window can be used to browse through available resources from base game and loaded mods
1. **Main Window** - if there are not tabs opened, then **Welcome Tab** will be visible. When resource is opened, it will appear in this main window area in a tab.
1. **Log Console** - window containing live updated logs from current session.
1. **Window Menu** - in top toolbar it is possible to access **all available Editors** , options, plugins and other things. More details can be found below

Beside that, the Workbench version is available in the bottom right corner along to which backend (Production = Stable branch, Submission = Experimental branch) it is connected.

## Window Menu

| Title | Hierarchy |
| --- | --- |
| **Workbench** | - Save<br>- Save All<br>- Options<br>- Publish Project<br>- Login/Logout<br>- Exit |
| **Editors** | - [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor)<br>- [Arma Reforger:Particle Editor](https://community.bistudio.com/wiki/Arma_Reforger:Particle_Editor)<br>- [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor)<br>- [Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor)<br>- [Arma Reforger:Audio Editor](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor)<br>- [Arma Reforger:Behavior Editor](https://community.bistudio.com/wiki/Arma_Reforger:Behavior_Editor)<br>- [Arma Reforger:Procedural Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Procedural_Animation_Editor)<br>- [Arma Reforger:String Editor](https://community.bistudio.com/wiki/Arma_Reforger:String_Editor) |
| **View** | - Normal view<br>- Text view<br>- Hex view<br>- 8 bytes<br>- 16 bytes<br>- 32 bytes |
| **Edit** | - Undo<br>- Redo<br>- Reimport resource<br>- Locate file in resource browser |
| **Window** | - Next Tab<br>- Prev Tab<br>- Cycle Tabs<br>- Close Current Tab<br>- Undo Close Tab<br>- Log Console<br>- Resource Browser<br>- Resource Browser 2 |
| **Utilities** | - Generate GUID<br>- File system sources<br>- Qt introspection tool |
| **Plugins** | - Settings<br>- Reload WB Scripts<br>- Clear All Settings<br>- List world resources<br>- Texture Import<br>- Model Import<br>- Material Import<br>- Terrain Import<br>- Edit Selected Prefab(s)<br>- SVN Log<br>- Create/Update Selected Editable Prefabs<br>- Image Set Generator<br>- Batch resource processor<br>- Batch texture processor<br>- Re-Save Meta Tool<br>- Re-Save Tool<br>- In-game Editor<br>- Register Placeable Entities...<br>- Update All Editable Prefabs<br>- Create/Update Selected Editable Prefabs<br>- SVN<br>- SVN Log<br>- User Settings<br>- Edit Engine Settings<br>- Edit Game Settings |
| **Help** | - About |

### Workbench

#### Save

Save the file from the currently selected tab.

#### Save All

Save the file from all opened tabs.

#### Options

See [Resource Manager: Options](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Options)

#### Login/Logout

Log in or out of the Bohemia account. Logging in allows for publishing online to the [Bohemia Workshop](https://community.bistudio.com/wiki/Arma_Reforger:Workshop).

#### Publish Project

[Publish the mod](https://community.bistudio.com/wiki/Arma_Reforger:Mod_Publishing_Process) in its current state.

#### Exit

Close the Workbench and all other open editors, if any.

### Editors

This menu allows for navigating through all available editors.

**Note:** Greyed out editors are either already opened or unopenable as per the script base status (non-compilable code).

### View

This menu allows the switching of the current tab's view mode between:

- **Normal** view
- **Text** view
- **Hex** view (8, 16 or 32B)

**Note:** **Text** and **Hex** views are **read-only**.

### Edit

#### Undo

Undo the last applied action to the world. Shortcut: [Ctrl+Z].

#### Redo

Reapply the last undone action. Shortcut: [Ctrl+Y].

#### Reimport resource

Reload the current file from storage. Wanted changes must have been saved first! Shortcut: [F5].

#### Locate file in resource browser

Show the selected file in the Resource Browser tree. Shortcut: [Ctrl+B].

#### Edit prefab

Open the World Editor to edit the currently-opened prefab. Shortcut: [Ctrl+E].

### Window

The first section groups Tab operations.

The second section groups Window operations.

#### Next Tab

Shortcut: [Ctrl+PageDown].

#### Previous Tab

Shortcut: [Ctrl+PageUp].

#### Cycle Tabs

Shortcut: [Ctrl+Tab].

#### Close Current Tab

Shortcut: [Ctrl+W].

#### Undo Close Tab

Shortcut: [Ctrl+Shift+T].

#### Log Console

Hide/show the Log Console and its history. Shortcut: [F1].

#### Resource Browser

Hide/show the "project tree" view. Shortcut: [F3].

#### Resource Browser 2

Hide/show a secondary Resource Browser. This one has no keyboard shortcuts.

### Utilities

#### Generate GUID

Generate a new Globally Unique IDentifier.

#### File system dump

Show which directories are used as source, in the form of

1. **<Game>** game data directory
1. **core** ./addons/core/
1. **profile** game user profile directory

#### Qt introspection tool

Show debug information about Qt, the Workbench Graphical User Interface (GUI).

### Plugins

This menu lists Workbench plugins. Each plugin has its own usage and user manual.

#### Settings

##### Reload WB Scripts

Reload Workbench plugins. Shortcut: [Ctrl+Shift+R]

##### Clear All Settings

Resets all parameters that have been changed, Workbench parameters included.

##### Texture Import

##### Model Import

##### Material import

##### Terrain Import

##### Edit Selected Prefab(s)

##### Create/Update Selected Editable Prefabs

##### Bookmark #

##### SVN Log

#### Batch resource processor

#### Batch texture processor

#### Re-Save Meta Tool

Re-save the `.meta` files for files having the provided extensions. Useful if a format has changed and files should be re-saved.

#### Re-Save Tool

Re-save the files having the provided extensions. Useful if a format has changed and files should be re-saved.

Extensions are to be provided **without** the dot (e.g `layout` and not `.layout`)

#### Image Set Generator

#### In-game Editor

#### User Settings

##### Edit Engine Settings

##### Edit Game Settings

##### Register Placeable Entities...

##### Update All Editable Prefabs

##### Create/Update Selected Editable Prefabs

#### Bookmarks

#### SVN

##### SVN Log

#### Validate Behavior Trees

#### Search Tool (XOBs)

#### Edit Selected Prefab(s)

Shortcut: [Ctrl+Shift+E]

#### Generate Class from Layout

#### Localize Selected Files

Shortcut: [Ctrl+Alt+L]

#### Find Linked Resources

#### Validate Prefabs

#### Generate Controls Scheme

## Resource Browser

Resource Browser is the main tool for **navigating** and **managing** game data, it can work with "local" data, packed data and both at the same time.

Default layout provides navigation through game data in two windows - one displays **directories** in tree layout and the other displays content (**sub-directories** and **files**) of the selected one.

There is a **Search bar** at the top left which has various filters and options available.

And lastly, over viewport, there are **tabs** with opened resources.
**Note:** This section also applies to "Resource Browser 2" that is a simple copycat of this window.

### Contextual Menus

In both windows, using [rmb] on files and folders opens a **context menu**, dependent on the clicked item's type:

| Context | Options |
| --- | --- |
| Tree view | - Show in Explorer<br>- Register<br>- Edit Properties<br>- Delete<br>- Rename<br>- Copy Resource Name(s)<br>- Copy Resource GUID(s)<br>- SVN<br>- Update<br>- Commit<br>- Add<br>- Remove<br>- Revert |
| File view | - Open File in New Tab<br>- Show in Explorer<br>- Navigate to Folder<br>- Register<br>Registers the resource to be used (creates .meta files)<br>- Register and Import<br>**Register** and (re)**import**<br>- Reimport<br>Converts file from source file (tiff, fbx, etc.) to Enfusion file (xob/txo, edds, etc.)<br>- Delete<br>- Rename<br>- [Navigate to Original](https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics#Navigate_to_Original)<br>- [Navigate to Ancestor](https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics#Navigate_to_Ancestor)<br>- [Navigate to Override](https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics#Navigate_to_Override)<br>- [Override in...](https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics#Using_.22Override_in....22_function)<br>- [Duplicate to...](https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics#Using_.22Duplicate_to....22_function)<br>- [Inherit in...](https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics#Using_.22Inherit_in....22_function)<br>- [Transfer to...](https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics#Using_.22Transfer_to....22_function)<br>- Copy Resource Name(s)<br>Copies the Enfusion path to this resource (format `{GUID}path/to/file.ext`) to the clipboard<br>- Copy Resource GUID(s)<br>Copies the resource GUID to the clipboard<br>- Copy File Path(s)<br>Copies the absolute path (format `C:\path\to\file.ext`) to the clipboard<br>- Copy Link<br>Copies an [`Enfusion://` link](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Options#Register_.22enfusion:.2F.2F.22_protocol) pointing to this resource to the clipboard<br>- SVN<br>- Update<br>- Commit<br>- Add<br>- Remove<br>- Revert<br>- Find References<br>**expensive operation** looks through all project files to find a reference to this element. *not present for script files*<br>- Refresh Thumbnail(s)<br>- [Edit Prefab(s)](https://community.bistudio.com/wiki/Arma_Reforger:Prefabs_Basics#Prefab_edit_mode)<br>[Opens the prefab](https://community.bistudio.com/wiki/Arma_Reforger:Prefabs_Basics#Opening_prefab_in_Prefab_Edit_Mode) in [World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) in Prefab Edit Mode to see and edit it<br>- Save changes<br>- Revert changes |

### Usage

#### Search bar

The **Search bar** allows the typing of words/sequences, separated by a space, that should be found in the file name - including keywords.

It is also possible to search by GUID, i.e. `A1F2619C082CFDDF` will find `{A1F2619C082CFDDF}assets/models/weapons/Ammo/Bullets/545x39/Data/5_45x39_US_NMO.edds`

e.g. looking for `village 1i03 .xob` will find and pinpoint `House_Village_E_1I03.xob`.

##### Special characters

By using those character it is possible to modify the search result

****

| Character | Function | Example |
| --- | --- | --- |
| 1. | Search by tag | `#myCustomTag` - search by tag |
| ! | Exclude from search | `chair  old !dst` - this will search for all assets called **chair old** but it will exclude from the list files having **dst** in the file name |

#### Filters

- Show Runtime files only
- Show unregistered sources only

- **Filter by Category**<br>Filter files by type (See [File Types](https://community.bistudio.com/wiki/Arma_Reforger:File_Types))

#### Options

- Expand All on Search

- Horizontal Layout
- Vertical Layout

- Grid View
- List View

- On with filters
- Always On
- Always Off

- Icons Scale

- Sort by Name
- Sort by Size
- Sort by Type
- Sort by Date

- Ascending sorting order
- Descending sorting order

## Viewport

### Tabs

| Right-click | - Close Tab<br>- Close All Tabs<br>- Close All But This<br>- Reopen Closed Tabs<br>- Navigate to File<br>- Show in Explorer |
| --- | --- |

A **double-click** on the tab will **Locate** the file **in Resource Browser**, when a middle mouse button click will close it.

### Viewport Types

See Arma Reforger.

## Log Console

Log console displays various messages, they can be either **informative**, **warnings** or **errors**, and can be sorted by **type** or **category**.

## Details / Import Settings

**Details** and **Import Settings** tabs are closely related, even dependent on the Viewport type. See Resource Manager: Viewport for more respective details.

### Import Settings

This tab is about import options. Each type of resource has different options available that are detailed in Resource Manager: Viewport.

If any changes are done in this category, it’s required to click on the "**Reimport resource**" button at the top to take effect.

**Note:** Each platform has its own settings, but all of them inherit from PC.

#### Edit also selected files

It is possible to edit multiple files at once. To do that, you need to check the "Edit also selected files" checkbox and select files in Resource Browser.

#### Parameters

Different per resource type, each described in Arma Reforger.

## General Shortcuts

| Shortcut | Function |
| --- | --- |
| [Ctrl+S] | Saves resource in active tab |
| [Ctrl+Shift+S] | Save all |
| [F5] | Reimports active resource |
| [Ctrl+B] | Locates active resource in Resource Browser |
| [F1] | Opens/closes log console |
| [F3] | Opens/closes Resource Browser window |
| [Ctrl+Page Up] | Next tab |
| [Ctrl+Page Down] | Previous tab |
| [Ctrl+Tab] | Cycles between opened tab |
| [Ctrl+W] | Closes active tab |
