# Port ledger

Every feature of the old enfusion-mcp project, and what happens to it in this repo. Run `/port-status` to update this file.

- **Status**: `ported`, `not needed`, or `not yet`.
- **Detail**: for `ported`, a link to the new Tool or skill. For `not needed`, the reason. For `not yet`, the port ticket, if one exists.
- **Suggestion**: what the use of the old server in Claude Code session history suggests. Only the owner decides.

History: 53 sessions with the old server, from 2026-11-05 to 2026-11-15.

## Tools

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `animation_graph` | not yet |  | never used in 53 sessions, suggest: not needed |
| `api_search` | ported | [tool api_search](../src/api-docs/tools.rs) | used 53 times in 4 of 53 sessions, 0 errors, last 2026-11-15, suggest: port |
| `asset_search` | not yet |  | used 7 times in 1 of 53 sessions, 0 errors, last 2026-11-15, suggest: port |
| `building_setup` | not yet |  | never used in 53 sessions, suggest: not needed |
| `component_search` | not yet |  | never used in 53 sessions, suggest: not needed |
| `config_create` | not yet |  | never used in 53 sessions, suggest: not needed |
| `game_browse` | not yet |  | used 33 times in 2 of 53 sessions, 0 errors, last 2026-11-15, suggest: port |
| `game_duplicate` | not yet |  | never used in 53 sessions, suggest: not needed |
| `game_read` | not yet | [#7](https://github.com/lanka-community/arma-reforger-tools-mcp/issues/7) | used 34 times in 3 of 53 sessions, 21 errors, last 2026-11-15, suggest: port |
| `gamemode_scaffold` | not yet |  | never used in 53 sessions, suggest: not needed |
| `layout_create` | not yet |  | never used in 53 sessions, suggest: not needed |
| `layout_recipe` | not yet |  | never used in 53 sessions, suggest: not needed |
| `mod` | not yet |  | never used in 53 sessions, suggest: not needed |
| `prefab` | not yet |  | used 2 times in 1 of 53 sessions, 0 errors, last 2026-11-15, suggest: port |
| `project` | not yet |  | never used in 53 sessions, suggest: not needed |
| `scenario_create` | not yet |  | never used in 53 sessions, suggest: not needed |
| `scenario_create_conflict` | not yet |  | never used in 53 sessions, suggest: not needed |
| `script_create` | not yet |  | never used in 53 sessions, suggest: not needed |
| `server_config` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_cleanup` | not yet | [#62](https://github.com/lanka-community/arma-reforger-tools-mcp/issues/62) | never used in 53 sessions, suggest: not needed |
| `wb_clipboard` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_component` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_connect` | ported | [tool workbench_status](../src/workbench/tools.rs) | never used in 53 sessions, suggest: not needed |
| `wb_diagnose` | ported | [tool workbench_status](../src/workbench/tools.rs) | never used in 53 sessions, suggest: not needed |
| `wb_entity_create` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_entity_delete` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_entity_duplicate` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_entity_inspect` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_entity_list` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_entity_modify` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_entity_select` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_execute_action` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_knowledge` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_launch` | ported | [tool workbench_launch](../src/workbench/tools.rs) | never used in 53 sessions, suggest: not needed |
| `wb_layers` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_localization` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_open_resource` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_play` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_prefabs` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_projects` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_reload` | not needed | the Workbench bridge calls ReloadScripts itself after it copies the Handler script (#40). The old Tool was never used in 53 sessions, and a user compiles their own scripts in the Script Editor (F7). | never used in 53 sessions, suggest: not needed |
| `wb_resources` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_save` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_script_editor` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_state` | ported | [tool workbench_status](../src/workbench/tools.rs) | used 1 times in 1 of 53 sessions, 1 errors, last 2026-11-15, suggest: port |
| `wb_stop` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_terrain` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_undo_redo` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wb_validate` | not yet |  | never used in 53 sessions, suggest: not needed |
| `wiki_read` | ported | [tool wiki_read](../src/wiki/read.rs) | used 8 times in 3 of 53 sessions, 0 errors, last 2026-11-15, suggest: port |
| `wiki_search` | ported | [tool wiki_search](../src/wiki/tools.rs) | used 27 times in 4 of 53 sessions, 0 errors, last 2026-11-15, suggest: port |
| `workshop_info` | not yet |  | never used in 53 sessions, suggest: not needed |

## Prompts

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `create-mod` | not yet |  | never used in 53 sessions, suggest: not needed |
| `modify-mod` | not yet |  | never used in 53 sessions, suggest: not needed |

## Resources

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `class-info` | not yet |  | never used in 53 sessions, suggest: not needed |
| `group-info` | not yet |  | never used in 53 sessions, suggest: not needed |
| `pattern-info` | not yet |  | never used in 53 sessions, suggest: not needed |

## Templates

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `config` | not yet |  | its tools (config_create) were never used in 53 sessions, suggest: not needed |
| `gproj` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `layout` | not yet |  | its tools (layout_create, layout_recipe) were never used in 53 sessions, suggest: not needed |
| `localization` | not yet |  | no old tool uses it, suggest: not needed |
| `prefab` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `scenario` | not yet |  | its tools (scenario_create) were never used in 53 sessions, suggest: not needed |
| `script` | not yet |  | its tools (script_create, mod) were never used in 53 sessions, suggest: not needed |
| `server-config` | not yet |  | its tools (server_config) were never used in 53 sessions, suggest: not needed |

## Recipes

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `air_vehicle` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `attachment` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `building` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `character` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `firearm` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `gamemode` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `gamemode/ambient_ai` | not yet |  | its tools (gamemode_scaffold) were never used in 53 sessions, suggest: not needed |
| `gamemode/auto_load_save` | not yet |  | its tools (gamemode_scaffold) were never used in 53 sessions, suggest: not needed |
| `gamemode/battle_prep` | not yet |  | its tools (gamemode_scaffold) were never used in 53 sessions, suggest: not needed |
| `gamemode/rank_persistence` | not yet |  | its tools (gamemode_scaffold) were never used in 53 sessions, suggest: not needed |
| `gamemode/scaled_respawn` | not yet |  | its tools (gamemode_scaffold) were never used in 53 sessions, suggest: not needed |
| `gamemode/seeding` | not yet |  | its tools (gamemode_scaffold) were never used in 53 sessions, suggest: not needed |
| `gamemode/spawn_validation` | not yet |  | its tools (gamemode_scaffold) were never used in 53 sessions, suggest: not needed |
| `generic` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `ground_vehicle` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `group` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `item` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `prop` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `spawnpoint` | not yet |  | used through prefab (2 calls), suggest: decide together with prefab |
| `ui/icon_overlay` | not yet |  | its tools (layout_recipe) were never used in 53 sessions, suggest: not needed |
| `ui/info_panel` | not yet |  | its tools (layout_recipe) were never used in 53 sessions, suggest: not needed |
| `ui/progress_hud` | not yet |  | its tools (layout_recipe) were never used in 53 sessions, suggest: not needed |
| `ui/status_hud` | not yet |  | its tools (layout_recipe) were never used in 53 sessions, suggest: not needed |
| `ui/timer_hud` | not yet |  | its tools (layout_recipe) were never used in 53 sessions, suggest: not needed |

## Patterns

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `admin-tool` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `custom-action` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `custom-component` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `custom-faction` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `custom-vehicle` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `game-mode` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `hud-widget` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `modded-behavior` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `spawn-system` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |
| `weapon-reskin` | not yet |  | its tools (mod) were never used in 53 sessions, suggest: not needed |

## Knowledge pages

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `AI_And_Behavior/behavior-tree` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `AI_And_Behavior/navmesh-terrain` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Audio_And_Sound/audio-system` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/core-concepts` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/INDEX` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/node-cheatsheet` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/node-reference` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/procedural-pap-siga` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/script-integration` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/state-machine-guide` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/animation/vehicle-animation` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/character-gear-and-anim-data` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Character_And_Animation/gadgets-actions` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/capture-and-hold` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/conflict-building-compositions` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/conflict-layer-reference` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/conflict-scenario-setup` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/conflict-seeding-system` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/conflict-supply-system` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/modded-gamemode-cookbook` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `GameModes_And_Scenarios/scenario-framework` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Inventory_And_Damage/damage-system` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Modding_And_Extensions/faction-creation` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Modding_And_Extensions/game-master` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Modding_And_Extensions/modding-basics` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Modding_And_Extensions/prop-creation` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Modding_And_Extensions/scripting-misc` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Modding_And_Extensions/ui-notifications-hints` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/config-system` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/enforceScript-language` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/entity-lifecycle` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/input-manager` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/multiplayer-replication` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/resource-usage` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/script-invoker` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/scripting-best-practices` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Scripting_And_Core/serialization` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Terrain_And_Environment/world-environment` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Tools_And_Workbench/mcp-net-api` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Tools_And_Workbench/particle-editor` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Tools_And_Workbench/resource-manager-blender` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Tools_And_Workbench/workbench-plugins` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Tools_And_Workbench/world-editor` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `UI_And_Interface/ui-dialogs-tooltips` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Vehicles_And_Physics/damage-vehicles` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Vehicles_And_Physics/physics-transforms` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Vehicles_And_Physics/vehicle-modding` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |
| `Weapons_And_Attachments/weapon-components` | not yet |  | its tools (wb_knowledge) were never used in 53 sessions, suggest: not needed |

## Animation graph code

| Feature | Status | Detail | Suggestion |
| --- | --- | --- | --- |
| `animation-graph-code` | not yet |  | its tools (animation_graph) were never used in 53 sessions, suggest: not needed |
