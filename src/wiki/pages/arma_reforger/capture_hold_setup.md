---
title: Arma Reforger:Capture & Hold Setup
url: https://community.bistudio.com/wiki/Arma_Reforger:Capture_&_Hold_Setup
---

Setting up a new **Capture & Hold** scenario based is easy, thanks to the Prefab system and already existing **base world** available as part of the Capture & Hold mod.

Arma Reforger 1.2.1 **Capture & Hold** is now packed in base game (`worlds/CaptureAndHold/CAH_BaseWorld.ent`) instead of the official mod.

## Creation Steps

- Open Arma Reforger Tools. If this is not visible in your Steam Library, activate "tools" in the filter.
- Click on "Create New"
- Give a good Project Name like "CAH The Cowshed by xyz" so that people can search for it and also directly understand what this is about.

### Initial Setup

Start by opening the Workbench and launch the World Editor from either the Quick Launch screen or Editors context menu.

### Scenario Setup

**Note:** By creating a sub-world of an existing CAH world you are already adding some elements to the world:

Continue to place the required elements to your world.

Drag and drop the following elements from the Resource Browser:

**Note:** The GarbageManager is not necessary anymore.

Now **save** the scenario and proceed.

### System Test

Once done, save your changes once more and close the World Editor.

**Note:** It is highly recommended to use Bohemia Interactive-prepared scenarios for inspiration and usage examples of individual features and components available in Arma Reforger.

### Scenario Header

To launch the scenario in-game a Scenario header (named `MissionHeader`) config must be created.

Exit the World Editor and return to the Resource Browser.

In the Resource Browser find your mod root directory and create a "Missions" directory inside.

Open the newly created Mission Header config.

In the **World** field, navigate to and select the world file. In our example it is the `MyScenario.ent` in the mod's root directory.

### In-Game Test

Launching Arma Reforger with this newly created mod enabled should allow you to see the and launch the scenario directly:

### Publish it to the Workshop

Now that the scenario can be launched from the game, only a few things remain.
Testing the scenario **in multiplayer** is important, making sure that there are enough spawn points and other gameplay details.

**Note:** For detailed information look at [Multiplayer Scripting](https://community.bistudio.com/wiki/Arma_Reforger:Multiplayer_Scripting).

Last, but not least is to **publish** the mod in the Bohemia Interactive Workshop for other people to see and play!

Important: Until your masterpiece has been tested, you should use Visibility: "Private" (only for you) or "Test" (Test mod category that can be enabled in the Workbench filter).

**Note:** See [Arma Reforger:Mod Publishing Process](https://community.bistudio.com/wiki/Arma_Reforger:Mod_Publishing_Process).

### Additional Settings Information

This is added as additional information so that you get up and running with a simple version first and then start worrying about the details.

#### Score Ending

- Hierarchy Window: Select the object "GameMode_CaptureAndHold"
- Object Properties: Select "SCR_ScoringSystemComponent"
- In the settings for "Scoring:Actions" change the Score Limit

#### Time Ending

- Hierarchy Window: Select the object "GameMode_CaptureAndHold"
- Object Properties: Select "SCR_CaptureAndHoldManager"
- In the settings for "End Game Duration" change the value which is given in seconds

#### Scoring: Multipliers

You can adjust several actions so that they will have a bigger impact on the game / scoring.

#### Kill Feed

It can be thrilling to leave the default values for this but for some communities the fun starts when they get more details in the "Kill Feed".

With this setting there will be no secrets about who was eliminated by whom.

#### Activate Unconsciousness

If you want to use it, you must activate it in the GameMode_CaptureAndHold. By default it is deactivated:

#### Spawn-Areas - No-Go Zone - Death Zone

You don't want the enemy to get too close to your spawn point? Simple: Use the spawn areas.

#### GarbageManager - Delete bodies from map

2024-03 This got disabled for now and is not available - might come back so this documentation stays.
If you are annoyed of too many bodies in your CAH areas, you need to make these adjustments:

1. Select your instance of CAH GameMode
1. Select `SCR_BaseGameMode`
1. Scroll down and select "Garbage System Config" and click on the search icon
1. Select "Override" in the right mouse button menu for GarbageSystem.conf

This will create a copy of the locked file into your scenario folder so that you are able to edit it.

The system will automatically open the new folder:

Open the file and edit it.

**Important:** Only the classes which are defined will be deleted.

This example shows how dead bodies are deleted after 180 seconds when player are further away than 3 meters.

1. Add a new class filter
1. Select "only destroyed"
1. Define Lifetime and the distance a living player has to have when the class is deleted.

### Common Issues

#### Can't initialize the game (World Editor won't start)

"World Editor and some other game depended features will not be available. But rest of workbench will be functional."

**Solution:** Delete the official CAH from "... \My Games\ArmaReforger\addons". If the error remains, delete all addons.

#### Editor crashes when using the garbage collector

**Solution:** Do not use it. A developer stated said that it is not needed anymore.

#### Repair your C&H scenario after version 1 of Arma Reforger

Sadly, with version 1 of Arma Reforger all Capture And Hold Scenarios have to be repaired and re-published.

- Open your scenario.
- Accept this message with "Yes".Place a new CaptureAndHold area as described above since it was removed while validating. Do not worry about the Garbage Collector: It is gone for good.

#### Repair your C&H scenario after version 1.2 of Arma Reforger

With version 1.2 the official Capture And Hold Scenario was implemented in the game and you have to remove the dependency from your scenario and re-publish it.

- Without opening Reforger Tools:
  - Edit the addon.gproj from the main directory of your scenario
  - Remove the Dependency "591AF5BDA9F7CE8B". A corrected version without any other mods will look like this:
```json
Dependencies {   
"58D0FB3206B6F859" 
}
```

Save and open your scenario:

- Publish
- If there are errors or problems selecting a Category: Don't give up - try again. Restart Workbench.

After you have fixed your scenario, it is important to delete the official Capture&Hold from .%userprofile%\Documents\My Games\ArmaReforger\addons\CaptureAMPHold_591AF5BDA9F7CE8B Otherwise Reforger will run into errors. CAUTION: Move the official Capture&Hold to a safe location, as you will need to copy it back for adjustments to other maps.
