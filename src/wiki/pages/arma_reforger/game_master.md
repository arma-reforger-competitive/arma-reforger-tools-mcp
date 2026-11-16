---
title: Arma Reforger:Game Master
url: https://community.bistudio.com/wiki/Arma_Reforger:Game_Master
---

**Game Master** is a game mode where nothing is planned, and where a player chooses what the next events will be.
It allows to create a real-time experience as a dedicated Game Master; curating events, placing assets, or just exploring by oneself.

It is equivalent to, and is the generic game mode name of [Arma 3 Zeus](https://community.bistudio.com/wiki/Arma_3_Zeus).

"Game Master" is also used to designate the player in the role of the current curator.

## Description

- the player that is **Server Host** (or the declared **Server Admin**) *always* has access to the Game Master interface
- if no Game Master is present, the **first player to connect** obtains the Game Master role
- the role cannot be transferred unless that player disconnects

| Possible | Not Possible |
| --- | --- |
| - create, edit, move and kill the AI units and vehicles present in the world<br>- edit, move and kill a player<br>- provide groups with waypoints<br>- create, move and delete prefabs on the fly<br>- spawn at will into the game | - modify the base terrain |

## Controls

| Keyboard/Mouse | Controller | Description |
| --- | --- | --- |
| [W][A][S][D] | [padLJoyMove] | Move the camera horizontally |
| [Q][Z] | [padLT][padRT] | Move the camera vertically |
| [RMB+Mouse] | [padRJoyMove] | Rotate the camera |
| [V] | Hold [padBack] | Edit scenario properties |
| Hold [Y] | [padStart+padLB] | Exit the interface |

## Interface

The interface is split into four panels:
- Mode
- Toolbar
- Scenario Menu
- Entity Menu

## Mode

**Gamepad:** The Mode interface is accessible using [padUp].

The top interface allows to switch between Game Master and Armavision.

**Note:** See [Armavision](https://community.bistudio.com/wiki/Arma_Reforger:Armavision) for more information.

## Toolbar

**Gamepad:** The Toolbar interface is accessible using [padDown].

### Map

Shortcut: [M] / Hold [padStart]

Toggle the in-game map.

### Flashlight

Shortcut: [L] / [padRB+padRSB]

Toggle flashlight for improved visibility in the dark. The light is only visible when it is dark and only to the Game Master, not to other players or AI.

### Toggle Interface

Shortcut: [I] / [padRB+padUp]

Hide editor interface. When hidden, use the shortcut to reveal it again.

### Guided Tour

Step-by-step introduction of the current mode.

### Clear Destroyed Entities

Delete all killed soldiers and destroyed vehicles. This will free up budget they may still occupy.

## Scenario Menu

**Gamepad:** The Scenario Menu interface is accessible using [padLeft].

Scenario properties shortcut: [V] / Hold [padBack]

### Scenario Menu Interface

This interface allows to define playable factions when none are defined (by clicking the big `"+"`) and to create faction spawn points as well as set *faction* **objectives** addressed to the **players** - the AI will ignore them.

**Gamepad:** The objectives menu is accessible by selecting a faction and holding [padY].

#### Move

Shortcut: [Alt+2]

#### Seize

Shortcut: [Alt+3]

#### Defend

Shortcut: [Alt+4]

#### Custom

Shortcut: [Alt+5]

Can be one of:

- Objective
- Recon
- Suppress
- Stand down
- Wait
- Assemble
- Prepare
- Rescue captive
- Rescue captives
- Board vehicle
- Locate contact
- Take captive
- Target killing
- Locate vehicle
- Steal vehicle
- Destroy vehicle
- Locate intel
- Locate asset
- Ambush
- Locate base
- Prepare defenses

### Playable Factions

Set which factions can be joined by players. This interface is also accessible by clicking the big `"+"` when no factions are configured.

**Important:** Be aware that removing a playable faction will kill any players already assigned to it.

### Game

- Enable respawn
  - Spawn near radio operators
  - Respawn time
- Server-wide ambient music

### Time and Date

- Time of the day (quick selection: 05:45, 08:52, 12:00, 20:12, 22:06, 00:00)
  - Time (slider, 00:00..24:00 by 15 minutes steps)
- Date (day, month, year, default 8 August 1989)
- Time progression (Y/N)
  - Time progression multiplier (default 1.0×, range 0.1×..12.0×, step 0.1×)

### Weather

- Automate weather (Y/N)
  - Weather: choose between Clear, Cloudy, Overcast, Rain and Fog
- Automate wind (Y/N)
  - Wind speed (range 0.0..15.0 m/s)
  - Wind direction (North, Northeast, East, Southeast, South, Southwest, West, Northwest)

## Entity Browser

Shortcut: [Tab] / [padBack]

Open the Entity Browser to place various characters, vehicles, structures and more.

### Controls

| Keyboard/Mouse | Controller | Description |
| --- | --- | --- |
| [Z] / [C] | [padLT] / [padRT] |  |
| [R] | [padRSB] | Reset filters |
| [Esc] | [padB] | Close |

### Filters

The Entity Browser allows to sort by:

- Faction
- Entity type
- Role
- Trait
- Content (if an entity is modded or not)

### Budgets

Budgets are the available "resources" that can be allocated on each Game Master aspects. They are shown at the bottom-right of the screen in the Entity Browser interface.

Once a budget reaches 100%, it is impossible to add more of its items unless previous ones are removed from the game.

For example, placing too many units will make the **AI Budget** reach 100% - more AI cannot be placed, and some previously placed ones (usually the unused ones) must be deleted in order to place new ones.

Other budget items (e.g vehicles) can still be placed.

**Object Budget**
Budget for all objects such as props and compositions.

**AI Budget**
Budget for computer-controlled entities such as non-player characters.

**Vehicle Budget**
Budget for cars, armored vehicles, etc.

**System Budget**
Budget for respawn points, objectives, arsenals, etc.

### Entity Placement

Once an entity has been selected in the Entity Browser, it can be placed in the world.

| Keyboard Controls | Gamepad Controls | Description |
| --- | --- | --- |
| [LMB] | [padA] | Place |
| [Space] | Hold [padA] | Place as a player |
| [Ctrl+LMB] | [padRB+padA] | Place and keep placing |
| [Shift] | [padRB+padLjoyLeftRight] | Rotate |
| [Alt] Move vertically | [padRB+padLjoyUpDown] | Move vertically |
| n/a | [padRB+padLSB] | Snap to surface |
| [RMB] | [padB] | Cancel placing |

## Gamepad-Specific Menu

Holding [padX] will open a radial menu which content depends on what is under the cursor.

Possible options:

| Action | Keyboard equivalent |
| --- | --- |
| Ping | [Y] |
| Move Camera | [F] |
| Teleport Player | [Space] |
| Take Control | [C] |
| Create new AI group | [RMB] option (on AI unit(s)) |
| Snap to terrain | [RMB] option |
| Play | [Space] |
| Heal | [RMB] option |
| Resupply | [RMB] option |
| Start Bleeding | [RMB] option |
| Stop Bleeding | [RMB] option |
| Cast Lightning | 2× [G] |
| Neutralize | [End] |
| Find in Entity Browser | n/a |
| Edit Properties | 2× [LMB] |

## Waypoints

A waypoint is a move/action order provided to an AI **group**.

**Important:** Not to be confused with **Objectives**, which are "waypoints" but for factions.

**Gamepad:** The waypoints menu is accessible by selecting the group then holding [padY].

| Name | Keyboard Shortcut | Prefab | Description | Priority | Radius | Timeout | Behaviour Tree |
| --- | --- | --- | --- | --- | --- | --- | --- |
| **Move** | [Alt+1] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_Move.et` | - Simple move waypoint<br>- Completes once the group leader reaches the waypoint radius. | 0.0 | 5.0 | n/a | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Move.bt` |
| **Forced Move** | [Alt+2] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_ForcedMove.et` | - Move waypoint ignoring autonomous behavior<br>- Completes once the group leader reaches the waypoint radius. | 2000.0 | 5.0 | n/a | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Move.bt` |
| **Move Relaxed** | [Alt+3] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_Patrol.et` | - Like simple move only different move type, slower | 0.0 | 5.0 | n/a | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Patrol.bt` |
| **Search and Destroy** | [Alt+4] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_SearchAndDestroy.et` | - Timed waypoint, for which duration the waypoint radius must not contain any known enemies. When it does during that time, the timer resets.<br>- AI investigate area inside given waypoint radius. | 0.0 | 20.0 | 600.0 | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Move.bt` > `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_SearchAndDestroy.bt` |
| **Defend** | [Alt+5] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_Defend.et` | - Timed waypoint for which duration AI will stand guard inside given radius.<br>- By default never completes | 0.0 | 30.0 | Never completes | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Move.bt` > `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Defend.bt` |
| **Get In** | [Alt+6] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_GetInNearest.et` | - Timed waypoint for which duration AI checks inside WP radius for available functional vehicle to mount<br>- Completes once all units have found a position and mounted, or times out.<br>- It is possible to adjust boarding parameters in prefab (driver/gunner/cargo allowance) | 0.0 | 20.0 | 30.0 | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Move.bt` > `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_GetInNearest.bt` |
| **Get Out** | [Alt+7] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_GetOut.et` | - AI moves to waypoint radius and disembarks<br>- It is possible to adjust boarding parameters in prefab (driver/gunner/cargo allowance) | 0.0 | 9.0 | n/a | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Move.bt` > `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_GetOut.bt` |
| **Suppressive Fire** | [Alt+8] | `enfusion://ResourceManager/~ArmaReforger:PrefabsEditable/Auto/AI/Waypoints/E_AIWaypoint_Suppress_Editor.et` | - Units shoot at WP position<br>- waypoint does not complete | 2000 | n/a | Never completes | `enfusion://BehaviorEditor/~ArmaReforger:AI/BehaviorTrees/Waypoints/WP_Suppress.bt` |
