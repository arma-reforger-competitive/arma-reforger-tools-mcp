---
title: Enforce script language: Script Profiling
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Enforce_ScriptProfiling.html
---

## Ingame Profiler

Dev menu (Win+Alt):

Statistics → Script Profiler → frame / continuous

## External Profiler - Tracy

You need to tell ScriptVM to add script profile scopes to profiler (they are turned off by default to not spam C++ devs)

- Option 1: Ingame dev menu (Win+Alt): Statistics → Script prof. external → true
- Option 2: Game command line: "game.exe *-scriptExtProfiler* "

In Tracy look for "Scr::" prefix.

## Memory Allocations

Dumping all allocations from script:

- need to run game with command line "*-checkInstance*"
- from script call:
```
Debug.DumpInstances(true);
```
- example of output/script log (when CSV formatting is on):
```
==== Script allocations (script side) ====
count,class,file,line,method
144,array<@AnimSoundEvent>,scripts/3_Game/InventoryItemType.c,16,LoadSoundEvents
1,ActionEatTetracyclineAntibiotics,scripts/4_World/Classes/UserActionsComponent/ActionConstructor.c,123,ConstructActions
1,Wetness,scripts/4_World/Classes/PlayerNotifiers/NotifiersManager.c,19,NotifiersManager
55,map<string,@WidgetCacheObject>,scripts/5_Mission/GUI/WidgetCacheObject.c,16,WidgetCacheObject
1,Param1<int>,scripts/3_Game/tools/UtilityClasses.c,24,Init
1,CraftGhillieTop,Scripts/4_World/Classes/Recipes/Recipes/_RecipeList.inc,11,CreateAllRecipes
1,array<@Param>,scripts/4_World/Plugins/PluginBase/PluginItemDiagnostic.c,23,PluginItemDiagnostic
861,array<SoundObjectBuilder>,scripts/3_Game/DayZAnimEventMaps.c,34,LoadTable
```
