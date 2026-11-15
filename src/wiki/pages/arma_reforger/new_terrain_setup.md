---
title: Arma Reforger:New Terrain Setup
url: https://community.bistudio.com/wiki/Arma_Reforger:New_Terrain_Setup
---

This tutorial explains the very first steps of how to setup a new terrain.

## Terrain Setup

### Create a New World

In [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor), click on **Create new World** to open the **Create world** interface.

In there, set **world type** to **Base scene** (the new world is to start 100% from scratch, **Sub-scene** would make an "additional layer" on an existing, opened terrain).

### Create the Terrain Entity

1. In the **Resource Browser**, search for "terrain" and drag and drop a `enfusion://ResourceManager/~ArmaReforger:Prefabs/World/DefaultWorld/GenericTerrain_Default.et` Prefab in the world.
**Important:** This prefab will set correct terrain parameters like for instance **Terrain** [Layer Preset](https://community.bistudio.com/wiki/Arma_Reforger:Collision_Layer), without which vehicles might not behave correctly! <br>Incorrect behavior can be manifested, for example, by **bouncing or colliding wheels of vehicles**.

1. Set its position to X: 0, Y:0, Z:0
**Warning:** This step is important as terrain collision/contact and Terrain Tools may encounter issues otherwise.

1. Save the terrain (using e.g [Ctrl+S]).
1. Right-click on the created GenericTerrainEntity and choose **Create new terrain...**; this will open the New Terrain interface, allowing to set terrain grid resolution values.
1. When everything is setup, press the Create button.

**Et voilà !** The newly created terrain is now ready to be sculpted with [World Editor's sculpting tools](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor:_Terrain_Creation_Tool#Sculpt).

## Environment Setup

### Sun

In the **Create** tab, search for "light" to drag and drop a `GenericWorldLightEntity` in the world.

#### Transformation

- **Angle X** - Set light source's pitch angle (e.g -20° to have a low-on-horizon sun)
- **Angle Y** - Set light source's yaw angle: 0 = from the South, 180 = from the North, 90 = from the West, 270 = from the East
- **Angle Z** - Set light source's roll angle (does nothing)

#### Light color properties

- **Specular Mul** - It is a multiplier of the Sun hotspot's specular intensity
- **Sun Angular Size** - the size of the Sun in a hotspot

#### Global Light color properties

- **Direct Light LV** - Intensity of the Sun
- **Direct Light Color** - Color of the Sun light
- **Indirect Light LV** - Intensity of "ambient" light
- **Indirect Light Color** - Color of the "ambient" light

#### Global Indirect light modificators

- **Probe Reflection EV** - Intensity of reflection at "LOW" roughness materials (chrom, glass etc.)
- **Probe Reflection Color** - Color modificator for Reflection
- **Probe Diffuse EV** - Intensity of diffuse light at "HI" roughness materials (wall, plastic cloth etc.)
- **Probe Diffuse Color** - Color modificator for Diffuse

### Skybox

#### Sky Preset

In **world** entity's properties, set **Sky Preset** field (under **Environment**) to `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Sky/Atmosphere/Atmosphere.emat` - this setups a "real world" atmosphere for the world.

**Note:** The screen will turn white for a moment, this is normal as the HDR takes time to adapt to the new brightness.

**Important:** Using a static skybox (e.g `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Sky/SkyBox/HDRi_01.emat`) is a quick possibility but does not allow for further configuration such as adding Clouds or Celestial Bodies.

#### Celestial Bodies

In **world** entity's properties, add celestial bodies by adding items to the **Planet Preset** list (by clicking "+") and add the following:
- `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Sky/Planets/Sun_01.emat` - set a material to the skybox's light source
- `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Sky/Planets/Moon_01.emat`
- `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Sky/Planets/Stars_01.emat`

#### Clouds

In **world** entity's properties, set **Clouds Renderer** to **SkyVolCloudsRenderer**.

Set **Clouds Preset** to `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Sky/Clouds/Clouds_Volumetric.emat` - this setups the volumetric clouds.

### Fog

In **Resource Browser**, search for `fog .et` to drag and drop `enfusion://ResourceManager/~ArmaReforger:Prefabs/World/DefaultWorld/FogHaze_Default.et` for fog to reach this world.

### Ocean

In **world** entity's properties,
- set the **Ocean Material** field to e.g `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Water/Ocean/ocean.emat` (along others in the `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Water/Ocean` directory) - this setups the ocean's surface.
- set the **Ocean Simulation** field to e.g `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Water/Ocean/Simulation/oceanSimIsland.emat` (along others in the `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/Water/Ocean/Simulation` directory) - this setups the ocean's simulation.

### Post-Process Effects

In **Resource Browser**, search for `PP_ .et` to drag and drop a `enfusion://ResourceManager/~ArmaReforger:Prefabs/World/DefaultWorld/GenericWorldPP_Default.et` Prefab that setups the most commonly required post-process effects (PPAA, HDR, etc).

Typical PP effects:

| Effect | Priority | Description |
| --- | --- | --- |
| SSR | 0 | Screen space reflection |
| HeightmapAO | 1 | Voxel ambient occlusion used for trees and rocks |
| GodRays | 2 | Fog effect (screen space) |
| UnderWater | 3 | Effect for underwater scenes |
| Rain Effect | 4 | Rain effects in the scene |
| n/a | 5..13 | Unused range left free for game usage |
| SSDO | 14 | Screen space diffuse occlusion |
| HBAO | 15 | Screen space ambient occlusion |
| HDR | 16 | Camera setting |
| PPAA | 17 | Post-process antialiasing |
| n/a | 18 | Unknown |
| Noise | 19 | Night noise (to prevent brightness/gamma advantage - uses `enfusion://ResourceManager/~ArmaReforger:Common/Postprocess/NightNoise.emat`) |

### Weather

**Important:** Be sure to properly set the Geographic Coordinates:

|  | Latitude | Longitude |
| --- | --- | --- |
| **+** | North | East |
| **-** | South | West |

In the **Resource Browser**, search for `WeatherM .et` to drag and drop a `enfusion://ResourceManager/~ArmaReforger:Prefabs/World/Game/TimeAndWeatherManager.et` entity in the world.
- In its **Weather State Machine** field, set the `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/WeatherPresets/StateMachine/WeatherStates.conf` config.
- In its **Weather Parameters** field, set the `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/WeatherPresets/WeatherParameters.conf` config.

Set the weather by opening the **Weather Editor** by its icon (a sun behind a cloud) in the main toolbar.
- Click "Enable Preset"
- Set the **Preset File** field to `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/WeatherPresets/StateMachine/WeatherStates.conf` config.
- Set the **Master Cycle** field to e.g `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/WeatherPresets/StateMachine/Items/Variant_Clear_items.conf` config (along others in `enfusion://ResourceManager/~ArmaReforger:Terrains/Common/WeatherPresets/StateMachine/Items` directory).

### Local Environment Probe

In the **Resource Browser**, search for `envp .et` to drag and drop an `enfusion://ResourceManager/~ArmaReforger:Prefabs/World/DefaultWorld/EnvProbe_Default.et` Prefab in the world to properly capture local reflection / diffuse.

## AI Setup

A world needs some elements for the AI to behave as expected:
- an AI world provide navmeshes to AIs for them to navigate the world; drag and drop `enfusion://ResourceManager/~ArmaReforger:Prefabs/AI/SCR_AIWorld.et` in the world
**Important:** Only **one** AIWorld-type entity can exist in the world; the present AIWorld can however contain multiple `NavmeshWorldComponent` in order to hold multiple navmeshes.

**Note:** See the [Arma Reforger:Navmesh Tutorial](https://community.bistudio.com/wiki/Arma_Reforger:Navmesh_Tutorial) for more information on how to create, and [setup](https://community.bistudio.com/wiki/Arma_Reforger:Navmesh_Tutorial#Usage) a navmesh.

- a faction manager is required to make the AIs understand their affiliation and enmities
**Important:** A faction manager requires a **game mode** to work.<br>Drag and drop e.g `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Modes/Plain/GameMode_Plain.et` (or any game mode in the `enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/Modes` directory) for a simple game mode.

- a perception manager is required for AI spotting, fighting and understanding its environment; drag and drop `enfusion://ResourceManager/~ArmaReforger:Prefabs/World/Game/PerceptionManager.et`
