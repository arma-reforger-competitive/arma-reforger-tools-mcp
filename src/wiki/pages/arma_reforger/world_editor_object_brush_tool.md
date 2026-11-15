---
title: Arma Reforger:World Editor: Object Brush Tool
url: https://community.bistudio.com/wiki/Arma_Reforger:World_Editor:_Object_Brush_Tool
---

**Object Brush** is a scripted [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) Tool.
It allows for quick terrain "painting" with entities, based on the set parameters.

To access it, click the paintbrush icon in World Editor.

## Usage

- [LMB] to generate Prefabs in the area
- [LMB] and drag to generate Prefabs along mouse movement
- [Alt+LMB] then to generate Prefabs along a straight line; [Esc] to cancel
- Hold [Space] to delete Object Brush-created entities (works with all control combinations above)

## Settings

### Object Brush

| Name | Description |
| --- | --- |
| **Radius** | Brush's radius |
| **Strength** | Brush's strength, value being in entity per hectare (100×100m) |
| **Scale Fall Off Curve** | Define the scale's Fall Off curve - curve goes from centre to border |
| **Density Fall Off Enabled** | Enable Density Fall Off settings below<br>Density Fall Off only works well for wid brush single click placements |
| **Density Fall Off Curve** | Define the density's Fall Off curve - curve goes from centre to border |
| **Density Fall Off Subarea Count** | Define the amount of sub-areas (internal sub-division for performance) |
| **Override Brush** | Remove previously Brush-placed entities |
| **Objects Config** | This is where objects to be created are defined - see the section below. An `SCR_ObjectBrushArrayConfig` config can be drag & dropped here.<br>**Note:** Config examples can be found in `Configs/Workbench/ObjectBrush`. |

### Objects Config

| Name | Description |
| --- | --- |
| **Min Scale** | Maximum scale the object can have - requires Override Randomization enabled |
| **Max Scale** | Maximum scale the object can have - requires Override Randomization enabled |
| **Prefab** | The Prefab to use |
| **Bot Distance** | *unused* |
| **Random Pitch Angle** | Random X angle ("looking down/up") |
| **Random Roll Angle** | Random Z angle ("barrel roll") |
| **Vertical Offset** | *unused* |
| **Random Yaw Angle** | Random Y angle ("direction") |
| **Prefab Offset Y** | Static value that will be added to the Prefab's Y position |
| **Min Random Vertical Offset** | -value..0 negative range of Y randomness (cumulative with Prefab Offset Y) |
| **Max Random Vertical Offset** | 0..+value positive range of Y randomness (cumulative with Prefab Offset Y) |
| **Override Randomization** | If checked, Prefab will use:<br>- Min Scale / Max Scale<br>- Random Pitch, Random Roll, Random Yaw values<br>Otherwise, Prefab's Library random values will be used |
| **Weight** | The selection probability weight; if set to zero, the Prefab is ignored |
| **Align To Normal** | Align the generated Prefab to terrain's normal (makes it perpendicular to the terrain) |
| **Scale Falloff** | Whether or not using Scale Fall Off parameters |
| **Lowest Scale Falloff Value** | Minimum scale value the Scale Fall Off can set |

### Obstacles

| Name | Description |
| --- | --- |
| **Avoid Objects** | Avoid creating Prefabs where other entities are placed |
| **Avoid Objects Detection Radius** | Objects detection radius - can be zero |
| **Avoid Objects Detection Height** | Objects detection's max height (from the ground) |
| **Avoid Roads** | Avoid roads, using their `RoadGeneratorEntity` **Road Clearance** value |
| **Avoid Rivers** | Avoid rivers, using their `RiverEntity` **Clearance** value |
| **Avoid Power Lines** | Avoid powerlines, using their `SCR_PowerlineGeneratorEntity` **Clearance** value |
| **Avoid Land** | Avoid anything above ocean's level |
| **Avoid Ocean** | Avoid anything below ocean's level |

### Obstacles - Area

| Name | Description |
| --- | --- |
| **Avoid Forests** | Avoid forests (areas with a `ForestGeneratorEntity`) |
| **Avoid Lakes** | Avoid lakes (areas with a `LakeGeneratorEntity` **and** with "Is Lake" checked) |
| **Area Detection Radius** | Area (forest, lake) detection radius *from the first click*<br>Set to zero to detect all areas of the world, takes more time the first time (and more RAM) |
