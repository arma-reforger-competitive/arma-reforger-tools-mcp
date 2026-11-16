---
title: Arma Reforger:File Types
url: https://community.bistudio.com/wiki/Arma_Reforger:File_Types
---

This page lists file types and their respective extensions with which Arma Reforger and the Workbench work.

| Extension | File Type | Saved/Used by | Description |
| --- | --- | --- | --- |
| **.acp** | Audio component | [Arma Reforger:Audio Editor](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor) | Describe audio playback<br>Manages sound events:<br>- event ("SOUND_SHOT", "SOUND_RELOAD" etc)<br>- sound file + parameters (signals, playback rules etc)<br>Referenced by BaseSoundComponent children |
| **.adeb** | Animation debug | Animation offline debugger | Binary stream format<br>Contains:<br>- 16b - File version defined in AnimDebuggerSerializationVersion<br>- List of all debugged descriptors<br>- For each frame list of all debug instances<br>File can be loaded in animation editor using Live debug tab |
| **.ae** | Animation Events Table | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | Lists all events used in the animation workspace<br>StepEvent, SoundEvent, SynchronizationEvent, etc |
| **.afm** | Audio Final Mixer | [Arma Reforger:Audio Editor](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor) | "Sound mixer" referenced by the .acp<br>Takes all the sound inputs and produces the final sound output |
| **.agf** | Animation graph file | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | Contains sheets (that contain [Animation Nodes](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor:_Nodes)) of an animation graph |
| **.agr** | Animation graph | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | "root format" sheets collection with graph parameters, e.g commands, variables, IK chains, etc<br>Depends on .ast |
| **.anm** | Animation file | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor)<br>+ various plugins | Binary animation file |
| **.asc** | Height map (Esri grid) | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Height map file which can be exported or imported in Workbench (Terrain tools in World Editor)<br>**Note:** Not used in runtime neither directly by the editor - only for import/export. |
| **.asi** | Animation Set Instance | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | Bridges .anm with .ast abstract names<br>Depends on .ast<br>See [Animation Editor: Templates and Instances Tutorial](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor:_Templates_and_Instances_Tutorial) |
| **.ast** | Animation Set Template | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | Structural skeleton to be filled by .asi<br>Provides a list of entries to be filled<br>See [Animation Editor: Templates and Instances Tutorial](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor:_Templates_and_Instances_Tutorial) |
| **.asy** | Animation Sync Table | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | See [Animation Editor: Sync Tutorial](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor:_Sync_Tutorial) |
| **.aw** | Animation Workspace | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | Workbench's Animation Editor "project" - saves/opens current workspace |
| **.bt** | Behavior tree | [Arma Reforger:Behavior Editor](https://community.bistudio.com/wiki/Arma_Reforger:Behavior_Editor) | AI behaviour tree |
| **.bterr** | Terrain data | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Terrain data - general terrain height map editor information |
| **.bttile** | Terrain data | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Terrain data, height-map data for terrain tile used by the editor<br>(original imported height map without modifications by roads etc) |
| **.c** | Enforce Script Source | [Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor) | Enforce Script Source file |
| **.conf** | Generic config | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s [Config Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Config_Editor) |  |
| **.ct** | Component Template | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Defines an Entity Component |
| **.dds** | Texture | Art Tools | Source image format - other formats are also supported like png, jpeg, tiff, tga, hdr |
| **.desc** | Terrain dialogs configuration | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Configuration of terrain texture import/generating dialogs (height map, satellite texture, normal map) |
| **.edds** | Enfusion DDS Texture | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s [Texture Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Texture_Editor) | Enfusion proprietary (extended) compressed DDS texture |
| **.emat** | Material Definition | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager) | Defines material for 3D objects, terrain, post processes, water, sky objects etc |
| **.ent** | World Scene | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | World contains terrain and all other objects that make up the your game world |
| **.et** | Entity Template | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Defines an Entity content (prefab) |
| **.fbx** | 3D model source file | 3D software | .txo and .xob source format |
| **.fnt** | Font | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s [Font Viewer](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Font_Viewer) | Font for UI widget system, binary format |
| **.gamemat** | Game material | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s [Config Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Config_Editor) | Definition of in-game material (properties, sound, etc) |
| **.gproj** | Project-specific Settings | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Every Project has its own settings stored here |
| **.imageset** | ImageSet Definition | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s [Imageset Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Imageset_Editor) | Defines a set of images by cuts from a texture |
| **.layer** | World Layer | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | World file (.ent) can have mutliple layers stored in separate .layer files - can only be opened through the .ent file |
| **.layout** | Layout Definition | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s [Layout Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Layout_Editor) | UI layout definition |
| **.meta** | Meta Information | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager) | Every registered asset has its meta file containing asset-specific settings and asset GUID |
| **.nmn** | Navmesh | Navmesh Generator Tool | Binary file with single navmesh instance - see [Navmesh Tutorial](https://community.bistudio.com/wiki/Arma_Reforger:Navmesh_Tutorial) |
| **.pak** | Package (of data) | Workbench | Game or Mod data archive |
| **.pap** | PA project | [Arma Reforger:Procedural Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Procedural_Animation_Editor) | See [Arma Reforger:Procedural Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Procedural_Animation_Editor) |
| **.physmat** | Physics material | [Config Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Config_Editor) | Physical material (bounciness, adherence etc) |
| **.pre** | ResourceViewer Preset | Resource Browser | A preset with several options to allow customisation of Workbench's preview windows (usually saved in `%userprofile%\Documents\My Games\ArmaReforgerWorkbench\profile\ResourceViewerPresets`) |
| **.ptc** | Particle System Definition | [Arma Reforger:Particle Editor](https://community.bistudio.com/wiki/Arma_Reforger:Particle_Editor) | Particle effects definition |
| **.ragdoll** | Ragdoll Definition | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager) | Character ragdoll file |
| **.rdb** | Resource Database | Workbench | Resource database listing all resources in the project |
| **.sig** | Signal | [Arma Reforger:Audio Editor](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor) | Describes the logic of playing an audio component or encapsulate manipulation parameters - this audio resource is shared among audio components |
| **.siga** | PA signal | [Arma Reforger:Procedural Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Procedural_Animation_Editor) | Describes the control logic for parts of a PA project - this resource is shared among PA projects |
| **.smap** | Soundmap data | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Information for music and audio effects |
| **.snd** | Sound container | [Arma Reforger:Audio Editor](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor) | Arma Reforger 1.6.0 A wrapper for .wav and [Opus](https://en.wikipedia.org/wiki/Opus_(audio_format)) .ogg files |
| **.st** | String Table | [Arma Reforger:String Editor](https://community.bistudio.com/wiki/Arma_Reforger:String_Editor) | Strings used for localisation |
| **.stars** | Database | Proprietary format | Runtime database of stars. Contains up to 100.000 stars describing their real positions and colors<br><br>Array of Hunks (ID = "ITEM") each Hunk describes a Star object with following data:<br>- Ra - Right ascension (float)<br>- Dec - Declination (float)<br>- ProMoRa - Proper motion in right ascension (float)<br>- Parallax - (float)<br>- Magnitude - Star brightness (float)<br>- Color - Star color (uint32 RGBA)<br>- HIP Catalog - Dummy (not used uint32)<br>- Name - Star name, with max. 256 characters length<br>[Source data](https://cdsarc.cds.unistra.fr/viz-bin/Cat?I/311) |
| **.styles** | Widget Style Definition | [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s [Styles Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Styles_Editor) | Defines UI style |
| **.terr** | Terrain data | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Terrain height map information |
| **.topo** | Topography data | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Information for 2D map |
| **.ttf** | TrueType Font | Resource Browser | Source font file format supported by Enfusion |
| **.ttile** | Terrain data | [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | Terrain data, height-map data for terrain tile used in run time (here are baked modifications by roads etc) |
| **.txa** | Animation file | [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | Text animation file (source for .anm) - source animation |
| **.txo** | Model | Resource Browser<br>+ various plugins (see [Enfusion Blender Tools](https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools)) | Text model file (source for .xob) - source model |
| **.vhcsurf** | Vehicle surface | [Config Editor](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Config_Editor) | Surface properties that are used inside vehicle simulation (roughness, steering noise etc.) |
| **.wav** | Supported audio file format | Audio software | WAV file format |
| **.xob** | Model | Resource Browser | Binary model file |
