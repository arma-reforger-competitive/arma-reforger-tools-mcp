---
title: Arma Reforger:Data Modding Basics
url: https://community.bistudio.com/wiki/Arma_Reforger:Data_Modding_Basics
---

# Content Moddability Overview

## Can Be Replaced

This means that certain resource can be fully replaced. Usually such resources are created manually by creating same asset with same GUID

**Note:** **Example**:

Variant of *Model.xob* from ArmaReforger is created in Addon1 via ***Override in "Addon1" addon...***. This new file share same GUID of original file and can contain completely different mesh

## Can Be Modified

Certain elements of the resource can be modified while rest of them is kept

**Note:** **Example**:

Vehicle_Base.et is copied from ArmaReforger to Addon1 with exactly same GUID as original file via ***Override in "Addon1"*** action. Copy of Vehicle_Base.et, located in Addon1, contain only changes introduced in this addon and those changes are later applied to original prefab.

## Can Be Inherited From

A new resource can inherit from an existing file:

**Note:** **Example:**

Inherited version of ArsenalConfig_BLUEFOR.conf is created *via* ***Create Inherited File*** or ***Inherit in "Addon1"*** option. This new file has unique ID and inherits all data from its parent.

## Moddability table

| Extension | File Type | Can be replaced | Can be modified | Can be inherited from |
| --- | --- | --- | --- | --- |
| **.et** | Prefab | Warning | Yes | Yes |
| **.conf** | Config | Warning | Yes | Yes |
| **.xob** | Model | Yes | No | No |
| **.emat** | Material Definition | Warning | Yes | Yes |
| **.edds** | Enfusion DDS Texture | Yes | No | No |
| **.c** | Enforce Script Source | Warning | Warning | Yes |
| **.layout** | Layout Definition | Warning | Yes | Yes |
| **.ptc** | Particle System Definition | Warning | Yes | No |
| **.ent** | World Scene | Yes | No | No |
| **.layer** | World Layer | No | No | No |
| **.bt** | Behavior tree | Yes | Warning | No |
| **.anm** | Animation file | Yes | No | No |
| **.agf** | Animation graph file | Yes | No | No |
| **.agr** | Animation graph | Yes | Yes | No |
| **.aw** | Animation Workspace | Yes | No | No |
| **.asi** | Animation Set Template | Yes | No | No |
| **.pap** | Procedural Animation | Yes | No | No |
| **.siga** | Procedural Animation Signal | Yes | No | No |
| **.st** | String Table | Yes | No | No |
| **.acp** | Audio component | Yes | No | No |
| **.sig** | Signal (Audio) | Yes | No | No |
| **.wav** | Supported audio file format | Yes | No | No |

### Footnotes

Following assets are always overridden and cannot be fully replaced
Script files are fully replaced if script in other addon (*filesystem)* has same path as the one which is supposed to be replaced. For example **AIAutotest.c** located in *$ArmaReforger:scripts\Game\AI\* will be replaced by variant from **Addon1** if it is placed in same location and same name in **Addon1** file system - *$Addon1:scripts\Game\AI\ AIAutotest.c*
Usage of *modded* keyword is required + few other restrictions apply like method in class must not be private in order to allow overloading it
Behavior trees attempts to override each other, instead of replace

# Data manipulation

## Basics

In principle, any file with meta file and unique **[GUID](https://community.bistudio.com/wiki/Arma_Reforger:Workbench_Metadata#GUID) -** *global unique identifier -* can be be overridden or replaced . This means however, that files without meta file have to be modified in different way (scripts ) or cannot be replaced at all.

That means that for instance **models, textures, layout, prefabs, configs or animations can be replaced using this method.**

**Note:** When replacing assets, engine cares **only about [GUID](https://community.bistudio.com/wiki/Arma_Reforger:Workbench_Metadata#GUID) -** that means neither **file name** or **location** doesn't have to be same as source file.

Creating overrides, duplicates or inherited files can be performed from context menu which is available in **Resource Browser**. This menu is visible after clicking with **Right Mouse Button** on selected asset. Availability of the actions depends on the type of the assets, so for instance models (*XOBs)* have only *Navigate to ...* options.

|  | **Override** | **Replacement** | **Duplicate** | **Inherit** |
| --- | --- | --- | --- | --- |
| **Description** | Override creates a new file which shares same [GUID](https://community.bistudio.com/wiki/Arma_Reforger:Workbench_Metadata#GUID) as original asset. This action allows to selectively replaces or add new elements to selected asset.<br>For instance it is possible to change value<br>**It is not possible to remove elements from the overridden asset this way.**<br>This action can be performed by selecting **"Override in '...'"** action from the **Addons** menu. | Replacement, as name suggest, completely replace original file with the new file.<br>In **Particle Editor**, it is possible to use "**Create variant for '...' addon..."** action but beside that, in all other cases it user has to create duplicates on his own by ensuring that replacing file has same GUID as original one.<br>This can be done by either copy pasting meta file (if we have access to it) or changing GUID of asset in text editor, so it matches the original file GUID.<br>**Prefabs**, **configs** and **layouts cannot be replaced** - they will be always in override mode. | Duplication creates a new file, with new, unique GUID. Duplicated file contains all the data from the original file (including inheritance) but doesn't modify in any way original file.<br>This action can be performed by selecting **"Duplicate in '...'"** action from the **Addons** menu. | Inherit action creates a new child file which inherits all the data from the parent.<br>This action can be performed by selecting **"Inherit in '...'"** action from the **Addons** menu.<br>Inheritance of **prefabs** can be only performed in **Resource Browser** attached to **World Editor** |
| **Where it can be used** | Prefabs, configs, materials and layouts | Sounds, models, textures, animations, particle effects, world files or behavior trees | Prefabs, configs, materials or layouts | Prefabs*****, configs or layouts |

**Note:** When using [CLI parameter](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#addons) to load addons, **working addon** is equal to the last addon listed in addons list.

*Example:*

```enforce
-addons addon2, addon1, NewAddon
```

*In above example **NewAddon** is current work addon*

## Overriding assets

### Using "Override in..." function

"**Override to..."** function can be used to modify (override) existing assets like configs, layouyts or prefabs.

**Note:** **Override to "..."** option is available only for prefabs, layouts or configs. If there is already override in **working addon**, this option will be missing from the context menu.

After this option is clicked, **Resource Browser** will create new overridden file in **working addon** (retaining original folder structure) and then navigate to **overridden instance** of selected asset.

### Using "Navigate to..." function

Once you have file in addon which is overriding some data, you can quickly navigate

#### Navigate to Original

**Note:** ***Navigate to Original*** button is available in file context menu only if selected file overridden or replaced in some addon. If selected asset is not overridden in any currently loaded addon then this option will appear greyed out.

After this option is clicked, **Resource Browser** will navigate to **original instance** of selected asset.

#### Navigate to Override

**Note:** ***Navigate to Override*** button is available in file context menu only if selected file overridden or replaced in some addon. If selected asset is not overridden in any currently loaded addon then this option will appear greyed out.

After clicking that option, **Resource Browser** will navigate to the first **overridden instance** of selected asset.

In scenario where multiple addons are present and they are overriding same file, **Navigate to Override** can be used multiple times until last override is reached.

#### Navigate to Ancestor

**Note:** ***Navigate to Ancestor*** button is available in file context menu only if selected file overridden or replaced in some addon. If selected asset is not overridden in any currently loaded addon then this option will appear greyed out.

After clicking that option, **Resource Browser** will navigate to the first **ancestor of overridden instance** of selected asset.

In the scenario where multiple addons are present and they are overriding the same file, **Navigate to Override** can be used multiple times until the last override is reached.

### Scripts

In Enfusion it's possible to modify already existing scripts by using some of special keyword

- **[modded](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Advanced_Usage#Modding) -** keyword used to modify existing scripting class
- **[override](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics#override) -** keyword to override methods in modded classes
- **[super](https://community.bistudio.com/wiki/Arma_Reforger:Object_Oriented_Programming_Basics#super) -** allows you to invoke content of overridden method

#### Modded keyword

```enforce
modded class SCR_PlayerScoreInfo
{

}
```

#### Override keyword

```enforce
modded class SCR_ScoringSystemComponent : SCR_BaseGameModeComponent
{
 override void AddSuicide(int playerID) {
 {

 }
}
```

#### Super keyword

```enforce
modded class SCR_ScoringSystemComponent : SCR_BaseGameModeComponent
{
 override void AddSuicide(int playerID)
 {
	super.AddSuicide(playerID);
	// Your example code below
	AudioSystem.PlaySound("{9BB653FF9E065943}Sounds/Animals/Bos_Taurus/Samples/bawl/bawl_0.wav");
 }
}
```

### Using "Duplicate to..." function

In **Resource Browser** you can quickly create new, duplicated file in project you are currently working on. This file is exact copy of original file and will be used from that point when any part of the game will try use asset with this **GUID**. Any changes that you are going to do to that file are going to be applied applied to that copy are not going to be propagated to it.

**Note:** **Duplicate to "..."** option is available only for prefabs, layouts or configs.

After this option is clicked, **Resource Browser** will create new duplicated file - with name selected by the user - in **working addon** (retaining original folder structure) and then navigate to that duplicate.

## Inheriting

### Using "Inherit in..." function

Inheriting from the prefabs, config files or layouts can be performed via "**Inherit in...**" function. After clicking with **Right Mouse Button** on appropriate file and then selecting Inherit in function, a new pop up window will appear asking for a new file name. Once name is confirmed, a new file will be created in **working addon** with same folder structure as original file. As expected, all of the attributes will be **inherited from parent file**.

### Using "Transfer to..." function

Transfer to function is a functionality for moving data & merging partially overridden files (like prefabs or configs) from current working addon to parent addon. Parent addon need to be unpacked in order to have this functionality working.

## Replacing assets

Replacing assets means that i.e. AK-74 model can be with a simple gray box and all prefabs which are referencing that AK-74 are going to use that new gray box from now. Such replacement can be performed with sounds, models, textures, animations, particle effects, world files or behavior trees.

```
In Particle Editor replacement can be performed quite easily by using "Create variant for "..." addon..." function. This action creates new ptc file with same GUID and content as original file.
```

**Important:** Extreme caution is advised when doing such replacement - tinkering with Workbench generated files in text editor might break those files if you don't know what you are doing

In all other cases, replacing files is slightly more complicated and requires operations outside of Workbench.

#### Getting file GUID

First step towards creating such replacement would be obtaining GUID - this can be done via **Copy Resource GUID(s)** action, available in **context menu** after clicking on file with **Right Mouse Button.**

Once **GUID** is obtained, it can be manually typed into meta file of some newly created model. After that action is completed, restart of the **Workbench** might be necessary.

## Replacing script

Scripts can be replace each other simply when they are in same relative location with same name.

For instance, to replace [enfusion://ResourceManager/~ArmaReforger:scripts/Game/UserActions/PickupAction.c PickupAction.c] located in base **ArmaReforger** data, you would need to create a new script in your addon in same location and with exact name.
```c#
MyAddon\scripts\Game\UserActions\PickupAction.c
```

If multiple addons are trying to replace same file, then the last one in the loading chain will be used.
