---
title: Arma Reforger:Entity Catalog
url: https://community.bistudio.com/wiki/Arma_Reforger:Entity_Catalog
---

An Entity Catalog is a list of faction-related or not prefabs in order to have one cohesive list of all entities that is used by various systems,
as well as unifying the way information can be obtained from said entities without having to spawn the prefab first.

- Faction-related entity catalog can be found in said Faction's config (e.g US.conf (`enfusion://ResourceManager/~ArmaReforger:Configs/Factions/US.conf`))
  - Each faction has their own list with the entities associated with that faction.
  - Each entry within the list is a config. Do not edit the lists directly in the faction config but use the Catalog config instead.
- Factionless entity catalog can be found on the game mode (in `SCR_EntityCatalogManagerComponent` on GameMode_Base.et (`enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Modes/GameMode_Base.et`))
  - Lists prefabs that do not belong to a specific faction.

## Catalog

A catalog holds a collection of entities of a specific type. The Catalog has a **Catalog Type**, so prefabs listed in the catalog should correspond with the type (e.g Vehicles, Characters, Groups, etc).

There are two Catalog classes:

| Class | Description |
| --- | --- |
| `SCR_EntityCatalog` | This has one list for all entities of that catalog.<br>**Note:** See the code documentation as well. |
| `SCR_EntityCatalogMultiList` | Similar to `SCR_EntityCatalog` in that it holds an entity list, but different as it can also host sub-lists.<br>As an example the Inventory Items are in the ITEM catalog but WEAPONS are in a sub-list.<br>Each list has an identifier that should be descriptive of its content (code does not use this identifier so feel free to name it as wanted).<br>Lists can be created, moved, renamed, deleted as they are merged with the Full entity list on Init. Also note that the General EntityList can still be used as it is not cleared before the merge. |

## Entity Entry

Within the Catalog are the Entity Entries. These entries hold the actual prefab data and are the main entries with which to be working.

### Entry Info

There are a view pieces of info (almost) all entries will share. Different entry classes have different ways to obtain the information, so check the following table to know more about that.

| Prefab | The ResourceName of the prefab. |
| --- | --- |
| **Enabled** | - The entry will be removed from the list on init and never checked by the autotest if this is set to false.<br>- Use this for prefabs that are not yet ready for the game but you want to add them to the lists or this can be used for modders to remove some entries |
| **Labels** | - `EEditableEntityLabel` labels. The Catalog system supports labels and can quickly get all entries within a catalog that have (or have not) a specific label.<br>- By default this information is obtained from the `SCR_EditableEntityComponent` of the prefab but can also be set manually if the entity is not an EditableEntity. |
| **UI Info** | - UI info such as Name, Description and Icon<br>- By default this information is obtained from the `SCR_EditableEntityComponent` of the prefab but can also be set manually if the entity is not an EditableEntity. |
| **Entity Data List** | See Below for more information about this but these are essentially "components" with data that is attached to the Entry |

### Entry Classes

There are different entity entry classes which each are created for a different type of entry. Note these are some examples of how to use them but there are most likely more.

| Class | Description |
| --- | --- |
| `SCR_EntityCatalogEntry` | Editable Entity Prefab. The system will get the UIinfo and the Labels on init from the `SCR_EditableEntityComponentClass` on the prefab and you only need to worry about assigning the prefab and Data.<br>This is way you can still get the UIInfo and Labels but never have to set them.<br>**Important:** This prefab **must** be an editable entity. Non-Editable entities are not supported for this entry. |
| `SCR_EntityCatalogEntryNonEditable` | Base for inheriting to use Non-Editable entry prefabs. **Do not use directly!**<br>The system will show a warning if this class is used. |
| `SCR_EntityCatalogEntryCustomInfo` | Use this entry for non-Editable entities. It allows the user to set a custom UIInfo and Labels.<br>The labels work the same as the `SCR_EntityCatalogEntry` and you can get all entities with specific (or lacking specific) labels.<br>Custom UiInfo is required to be added else it will fail the autotest (Unless entry is disabled)<br>Technically you can also use Editable entities and use this class to overwrite the UIInfo and labels but it is not advised to do. Add a new Entity Data if you want to overwrite the UIInfo or get somespecific data. |
| `SCR_EntityCatalogInventoryItem` | This is used for inventory items.<br>At this moment it is not possible to get the UIInfo (that is assigned in the InventoryItemComponent) for inventory items so take that into account. |

## Entry Data

This is the real star of the Catalog and allows other devs to customise entries without interfering with other systems that might use the same entries.

Each Entry has an Entity Data list. Data are essentially components of the Entry which contains data specific to systems.
Let's say you have an entity spawner and you need to know the Supply cost, you will add a `SCR_EntityCatalogSpawnerData` and put the info in there.

More over the catalog has the `GetEntityListWithData()` method which allows you to quickly get all entries within the catalog filtered to have the specific Data type. This is a powerful system that makes it so that you do not have to worry about creating your own list of entities, maintaining the list or even having to worry about getting all entries with your data type. Also note there are more `GetEntityList*` methods to filter on data, labels and so on.

### Examples

Note this is not a full list but simply some examples and might not be up to date

| Data | Description |
| --- | --- |
| `SCR_ArsenalItem` | A rework of the arsenal item. Holds the Item type and item mode. Allowing the item to be spawned within the arsenal. |
| `SCR_EntityCatalogSpawnerData` | Entity spawner data. Allowing the system to get the Supply cost of an entity as well as which Slots it can spawn in, among other things. |
