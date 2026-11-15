---
title: Arma Reforger:General Game Mode Setup
url: https://community.bistudio.com/wiki/Arma_Reforger:General_Game_Mode_Setup
---

The **SCR_BaseGameMode** entity is the lowest usable game mode implementation that allows you to set up a custom game mode.

The core idea behind `SCR_BaseGameMode` is the expansion via specialised components of `SCR_BaseGameModeComponent` type.

These provide the user with API that allows hooking onto certain game events as necessary. Many provided components can work standalone and can be mixed and matched.

## Example Scenarios

To easily get familiar with how game modes can be set up now, please see the following scenarios:

| Mode | Link | Description |
| --- | --- | --- |
| **Plain** | `enfusion://ResourceManager/~ArmaReforger:worlds/MP/MpTest.ent` | - Simplest scenario<br>- Automatic respawn<br>- Free for all |

### Basic Setup

To setup the most basic scenario the user must add the following. For the simplest set-up add all items from the **"Pre-made"** column.

#### Free for All

| Type | Pre-made Prefab | Base Prefab | Description |
| --- | --- | --- | --- |
| **SCR_BaseGameMode** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Modes/Plain/GameMode_Plain.et` | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Modes/GameMode_Base.et` | Specialises game mode by using components and their combinations. Must always have a *SCR_RespawnSystemComponent* and specialised *SCR_RespawnHandlerComponent* to allow proper spawning. See Requirements. |
| **SCR_FactionManager** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Factions/FactionManager_FFA.et` | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Factions/FactionManager_Base.et` | Provides list of available factions and their properties. For free for all modes, use the FFA faction. |
| **SCR_LoadoutManager** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Loadouts/LoadoutManager_FFA.et` | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Loadouts/LoadoutManager_Base.et` | Provides list of available loadouts and their properties. For free for all modes, use the FFA faction. |
| **SCR_SpawnPoint** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Spawning/SpawnPoint_FFA.et` | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Spawning/SpawnPoint_Base.et` | Provides information to the respawn system, marking a location available for spawning at. Set per faction. For free for all modes, use the FFA faction. |

#### Faction Based

| Type | Pre-made Prefab | Base Prefab | Description |
| --- | --- | --- | --- |
| **SCR_BaseGameMode** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Modes/Plain/GameMode_Plain.et` | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Modes/GameMode_Base.et` | Specialises game mode by using components and their combinations. Must always have a *SCR_RespawnSystemComponent* and specialised *SCR_RespawnHandlerComponent* to allow proper spawning. See Requirements. |
| **SCR_FactionManager** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Factions/FactionManager_USxUSSR.et` | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Factions/FactionManager_Base.et` | Provides list of available factions and their properties. |
| **SCR_LoadoutManager** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Loadouts/LoadoutManager_USxUSSR.et` | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Managers/Loadouts/LoadoutManager_Base.et` | Provides list of available loadouts and their properties. |
| **SCR_SpawnPoint** | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Spawning/SpawnPoint_USSR.et` (USSR)<br>`enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Spawning/SpawnPoint_US.et` (US) | `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Spawning/SpawnPoint_Base.et` | Provides information to the respawn system, marking a location available for spawning at. Set per faction. |

## Requirements

**Note:** See [Arma Reforger:Respawn Setup](https://community.bistudio.com/wiki/Arma_Reforger:Respawn_Setup) in order to setup a game mode's respawn properly.

## Instances

**SCR_BaseGameMode**: single instance in the world - mandatory.

Ties game mode together, manages attached **SCR_BaseGameModeComponent**(s), calls necessary methods.

```enforce
BaseGameMode baseGameMode = GetGame().GetGameMode();
SCR_BaseGameMode gameMode = SCR_BaseGameMode.Cast(baseGameMode);
```

**SCR_FactionManager**: single instance in the world - mandatory.

Provides available factions and API for retrieving and finding these by keys or indices.

```enforce
FactionManager baseFactionManager = GetGame().GetFactionManager();
SCR_FactionManager factionManager = SCR_FactionManager.Cast(baseFactionManager);
```

**SCR_LoadoutManager**: single instance in the world - mandatory.

Provides available loadouts and API for retrieving and finding these by indices.

```enforce
SCR_LoadoutManager loadoutManager = GetGame().GetLoadoutManager();
```

**SCR_RespawnSystemComponent:** single instance in the world, attached to game mode. Mandatory.

Provides means of setting and request/response player faction, loadout and spawn points.

```enforce
SCR_GameModeBase gameMode; /* = yourInstance; */
SCR_RespawnSystemComponent respawnSystemComponent = gameMode.GetRespawnSystemComponent();
```

**SCR_RespawnComponent:** Not to confuse with **SCR_RespawnSystemComponent**!

This component is and should always be attached to a player controller. That means that remote client will always be able to retrieve their **local** SCR_RespawnComponent only!
The authority will be able to retrieve and iterate over all SCR_RespawnComponent.

```enforce
int myId; /* = my_local_id */
RespawnComponent baseRespawnComponent = GetGame().GetPlayerManager().GetPlayerRespawnComponent(myId);
SCR_RespawnComponent respawnComponent = SCR_RespawnComponent.Cast(baseRespawnComponent);
// Client:		if myId is local playerId, respawn component is returned. Null otherwise. (Owned PlayerController component only)
// Authority:	if myId is valid playerId, respawn component is returned. Null otherwise. (Any PlayerController component)
```

## Game State

The game has three pre-defined states that can be set by the server and are automatically replicated with appropriate callback to the clients. This can be omitted, leaving the game always in GAME state.

- **PREGAME** (e.g. wait for *n* players?)
- **GAME** (core game loop)
- **POSTGAME** (e.g. show scoreboard, next scenario voting)

For particular implementation and details, see `SCR_EGameModeState`, `SCR_GameModeEndData` and `SCR_BaseGameMode.OnGameStateChanged()`.

### Gamemode End

The game mode transitions into POSTGAME state once the authority deems so by calling the **EndGameMode()** method.

This method accepts an instance of `SCR_GameModeEndData`, which is a small serialisable structure that is replicated to all clients and contains somewhat arbitrary data.
This can be used to synchronise the end state (*winner player, winner faction, ...*) to show e.g. the end-game screen.

## Troubleshooting

**My game mode is not working as it was before, errors are logged into the console regarding missing game mode, what do I do?**

Navigate to the Basic Setup section.

Drag prefabs from the Faction Based table if you want to have US versus USSR game mode or Free for All table if FFA is what you are looking for.
