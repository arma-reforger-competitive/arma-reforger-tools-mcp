---
title: Arma Reforger:Object Import Tool
url: https://community.bistudio.com/wiki/Arma_Reforger:Object_Import_Tool
---

- **name**: CSV Object Import
- **editor**: world
- **descr**: A plugin to mass-import terrain objects
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/WorldEditor/SCR_ObjectImportPlugin.c`
The **CSV Object Import** [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) plugin allows users to parse a CSV file to mass-import objects in the currently-selected world layer.

## CSV File Format

CSV lines must be formatted the following way:

```
resourceHash posX posY posZ quatX quatY quatZ quatW scale
```

where:
- resourceHash: the ResourceName (or {GUID}) to be placed
- posX posY posZ: the relative or absolute (depending on `Relative Y` setting)
- quatX quatY quatZ quatW: a [quaternion](https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation) determining Prefab's angles
- scale: the wanted Prefab's scale

**Important:**
- Prefab resource name MUST be in double quotes. Single quotes or no quotes **will** cause a parsing error.
- As of Arma Reforger 1.6.0, only **spaces** are supported as data field separation.
- As of Arma Reforger 1.6.0, the parser expects **exactly** nine fields.
