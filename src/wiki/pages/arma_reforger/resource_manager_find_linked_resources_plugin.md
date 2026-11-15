---
title: Arma Reforger:Resource Manager: Find Linked Resources Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Find_Linked_Resources_Plugin
---

- **name**: Find Linked Resources
- **editor**: resource
- **descr**: Find resources linked by the selected resource(s)
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/ResourceManager/SCR_FindResourcesPlugin.c`
**Find Linked Resources** is a plugin that reads and outputs resources used by Resource Browser-selected ones.

**Important:** It is not recommended to select a huge amount of resources, the processing time may increase drastically.

## Usage

Use it with `Plugins > Find Linked Resources`; a window appears allowing to set a wanted extensions filter, separated by commas - e.g `et,edds,xob`.

## Output

The log console can output something along the lines of (here `enfusion://ResourceManager/~ArmaReforger:Prefabs/Vehicles/Wheeled/UAZ469/UAZ469.et` with `et,edds` filter):

```
Linked resources of type(s) et,edds (9):
@"Prefabs/Items/Core/Inventory_Virtual_Entity.et"
@"Prefabs/Vehicles/Core/Lights/VehicleLight_Brake_Base.et"
@"Prefabs/Vehicles/Core/Lights/VehicleLight_Hazard_Base.et"
@"Prefabs/Vehicles/Core/Lights/VehicleLight_Head_Base.et"
@"Prefabs/Vehicles/Core/Lights/VehicleLight_HiBeam_Base.et"
@"Prefabs/Vehicles/Core/Lights/VehicleLight_Rear_Base.et"
@"UI/Textures/Editor/EditableEntities/Vehicles/EditableEntity_Vehicle_Offroad.edds"
@"UI/Textures/Icons/InventoryHints/InventoryHint_SuppliesMove.edds"
@"UI/Textures/Icons/InventoryHints/InventoryHint_SuppliesStored.edds"
```
