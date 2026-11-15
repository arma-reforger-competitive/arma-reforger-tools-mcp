---
title: Arma Reforger:Respawn Setup
url: https://community.bistudio.com/wiki/Arma_Reforger:Respawn_Setup
---

## Game Mode Setup

1. Make sure the game mode's `SCR_RespawnSystemComponent` component has Spawn Logic set (under Respawn System).<br>Spawn Logic is an object responsible for defining respawn logic.
1. You can override `SCR_SpawnLogic` to define your custom respawn behaviour.<br>Right now, there are two Spawn Logic objects defined:
   - `SCR_AutoSpawnLogic` - spawns the player automatically after assigning random faction, loadout and spawn point. You can also define Forced Faction and Forced Loadout, which will assign the player with a specific faction and loadout.
   - `SCR_MenuSpawnLogic` - spawning is done via respawn menu. This object notifies the player when they're ready for spawn and opens respawn menu.
1. You can notice that SCR_RespawnSystemComponent has multiple `SCR_SpawnHandlerComponent` components as its children. This signifies what types of respawn are possible with the game mode to which it is attached.
   - `SCR_FreeSpawnHandlerComponent` - allows for spawning player at a certain position.
   - `SCR_PossessSpawnHandlerComponent` - allows for possessing a character by passing its `RplId`.
   - `SCR_SpawnPointSpawnHandlerComponent` - allows for spawning player at a spawn point.
1. The Deploy menu got a overhaul too. Instead of using a `SCR_SuperMenuBase` and `SCR_SubMenuBase`, the deploy menu now consists of 3 separate menus, all inheriting from `SCR_DeployMenuBase`.<br>Opening/closing logic of respawn menu screens is handled in `SCR_PlayerDeployMenuHandlerComponent`, which is attached to the player controller.
   - `SCR_WelcomeScreenMenu` - serves as an overview of the current game mode, objectives and rules, similar to what briefing submenu used to be. Opens only once when player joins the game.
   - `SCR_RoleSelectionMenu` - here you can select your faction, group and a loadout. Opens only for the initial setup when the player does not yet have an assigned faction.
   - `SCR_DeployMenuMain` - main deploy menu with map for selecting spawning points, but you are able to change your loadout and group as well. Opens after player's death.

## Requesting Respawn

If you need to request respawn, you can do easily by creating a new `SCR_SpawnData` class and passing it to the `SCR_RespawnComponent.RequestSpawn(SCR_SpawnData data)` method.

Here are examples for requesting respawn for aforementioned spawn handlers:

**SCR_FreeSpawnHandlerComponent**

```enforce
void RequestFreeSpawn()
{
	ResourceName prefab = "{84B40583F4D1B7A3}Prefabs/Characters/Factions/INDFOR/FIA/Character_FIA_Rifleman.et";
	SCR_FreeSpawnData data = new SCR_FreeSpawnData(prefab, "0 0 0");
	m_RespawnComponent.RequestSpawn(data);
}
```

**SCR_PossessSpawnHandlerComponent**

```enforce
void RequestPossessEntity(IEntity ent)
{
	SCR_PossessSpawnData data = SCR_PossessSpawnData.FromEntity(ent);
	m_RespawnComponent.RequestSpawn(data);
}
```

**SCR_SpawnPointSpawnHandlerComponent**

```enforce
void RequestSpawnPointSpawn(SCR_BasePlayerLoadout loadout, SCR_SpawnPoint spawnPoint)
{
	SCR_SpawnPointSpawnData data = new SCR_SpawnPointSpawnData(loadout.GetLoadoutResource(), spawnPoint.GetRplId());
	m_RespawnComponent.RequestSpawn(data);
}
```
