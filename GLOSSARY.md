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

**GUID**:
The unique id of a mod or of one resource in a mod, written as 16 hex characters. A mod's GUID is also its ID on the Workshop.
_Avoid_: UUID, ID (alone), hash

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

**Element**:
One unit inside a Faction with its own Leader: the platoon HQ, a rifle squad, or the weapons squad.
_Avoid_: Section, support element, group (the game's own group system)

**Leader**:
A Player Slot that runs the Briefing for its side: PL, PSG, SL, or WSL. These role names and their meaning come from the guide's glossary (`posibnyk/GLOSSARY.md`).
_Avoid_: PC, CMD, commander (alone), officer

**Squad net**:
The radio channel of one Element. Every player in the Element can talk on it.
_Avoid_: Squad channel, group frequency

**Platoon net**:
The radio channel that links the Leaders of one Faction.
_Avoid_: Command net, platoon channel

**Briefing**:
The phase before the Event starts, where the Leaders of each side look over the map, plan, place markers, and say they are ready.
_Avoid_: Planning phase, briefing screen

**Preparation phase**:
The timed phase after all Leaders are ready. Players are in their Start area and cannot fire.
_Avoid_: Staging phase, safe start, warmup

**Start area**:
The place where a Faction's players are kept during the Preparation phase, with one spawn point for each Element.
_Avoid_: Main, base, staging area

**Spectator camera**:
What a player sees after they die in a One-life Event.
_Avoid_: Ghost cam, death cam

## This project

**Claude Code plugin**:
The package that a user installs in Claude Code, named `reforger`. It holds the MCP server and the Know-how.
_Avoid_: Plugin (alone)

**Capability**:
One part of the MCP server: a job that needs running code, offered to Claude as one or more Tools.
_Avoid_: Plugin, feature, module

**Tool**:
One action that Claude can call on the MCP server, for example `wiki_read`.
_Avoid_: Command, function

**Know-how**:
Reforger knowledge written as a skill for users: steps, rules, examples, and template files.
_Avoid_: Guide, docs

**Developer skill**:
A skill for agents that work on this repo. It does not ship in the Claude Code plugin.
_Avoid_: Know-how

**Knowledge base**:
The Enfusion script API docs, wiki pages, and engine pages that ship with the Claude Code plugin.
_Avoid_: KB, index, database

**Section**:
One heading in a page of the Knowledge base, with all the text and smaller headings under it, up to the next heading of the same or a higher level.
_Avoid_: Chunk, part, paragraph

**Game files**:
The scripts, Prefabs, and configs inside the installed game's .pak archives.
_Avoid_: Game data, assets

**Workbench**:
Bohemia's editor for making Reforger mods.
_Avoid_: Editor, Enfusion Workbench

**Workbench plugin**:
A script that runs inside Workbench and does a job there when the MCP server asks for it.
_Avoid_: Plugin (alone)

**Workbench bridge**:
The connection through which Tools control a running Workbench.

**Port**:
Moving one Capability from the old enfusion-mcp project into this repo by translating its TypeScript into Rust.
_Avoid_: Migration, copy

**Port ledger**:
The list of every feature of the old enfusion-mcp project, each with one status: ported, not needed, or not yet. It is in `docs/port-ledger.md`, and `/port-status` keeps it up to date.
_Avoid_: Port list, checklist

**Gap**:
A moment when Claude, while working on a real mod, needs a Capability or Know-how that the Claude Code plugin does not have. A Gap ends as a Port, as a new Capability or Know-how, or as wontfix.
_Avoid_: Missing feature, feature request
