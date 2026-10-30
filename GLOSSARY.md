# Arma Reforger Tools

A Claude Code plugin that helps people learn, build, and fix Arma Reforger mods. It gives Claude capabilities (MCP tools) and know-how (skills).

## Reforger modding

**Entity**:
One object that exists in a World, for example a soldier, a vehicle, or a trigger.
_Avoid_: Object, actor

**Component**:
A piece of data or behaviour attached to an Entity.
_Avoid_: Script, module

**Prefab**:
A saved Entity with its Components and settings, used as a pattern to place new Entities.
_Avoid_: Template (in the game sense), blueprint

**World**:
A map with all the Entities placed on it.
_Avoid_: Level, terrain (terrain is only the ground)

**Scenario Framework**:
The built-in Reforger system for building missions from areas, layers, slots, and actions with little or no script code.

**Event**:
One planned play session for a group of players.
_Avoid_: Match, game night

**TvT**:
An Event where players fight on two or more sides, with no AI enemy as the main opponent.
_Avoid_: PvP

**One-life**:
An Event rule: a player who dies does not respawn.
_Avoid_: No-respawn, hardcore

**Faction**:
One side in the fight, for example US or USSR.
_Avoid_: Team, side

**Player Slot**:
One playable place in a Faction, with a role and a Loadout, that a player picks in the Lobby before the Event starts.
_Avoid_: Slot (alone), role (a role is only part of a Player Slot)

**Scenario Framework Slot**:
A place in a Scenario Framework mission where an object or AI is spawned.
_Avoid_: Slot (alone), spawn point

**Loadout**:
The gear a player starts with in a Player Slot.
_Avoid_: Kit, equipment

**Objective**:
A goal with a win or lose condition, for example "capture the base" or "hold until the timer ends".
_Avoid_: Task (that is the game's system), mission, goal

**Task**:
The game's own system for showing goals to players on the map and in the task list.

**Lobby**:
The screen before the Event starts, where players pick a Player Slot.
_Avoid_: Slotting screen, role select

## This project

**Capability**:
A job that needs running code, offered to Claude as one or more MCP tools.
_Avoid_: Feature, module

**Know-how**:
Reforger knowledge written as a skill: steps, rules, examples, and template files.
_Avoid_: Guide, docs

**Knowledge base**:
The scraped Enfusion script API docs and wiki pages that ship with the plugin.
_Avoid_: KB, index, database

**Game files**:
The scripts, Prefabs, and configs inside the installed game's .pak archives.
_Avoid_: Game data, assets

**Workbench**:
Bohemia's editor for making Reforger mods.
_Avoid_: Editor, Enfusion Workbench

**Workbench bridge**:
The connection through which MCP tools control a running Workbench.

**Port**:
Moving one Capability from the old enfusion-mcp project into this repo, reading and rewriting it as needed.
_Avoid_: Migration, copy
