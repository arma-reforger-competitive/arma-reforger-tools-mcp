---
title: File Types and Extensions
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_FileTypes.html
---

**Remarks:** List of file types and extensions Enfusion and Workbench works with in an alphabetical order.

| Ext | File Type | Saved by | Description | Comment |
| --- | --- | --- | --- | --- |
| .acp | Audio component | Audio Editor | Describe audio playback. Manages sound events: event ("SOUND_SHOT", "SOUND_RELOAD" etc), sound file + parameters (signals, playback rules etc) | BaseSoundComponent children reference these |
| .adbg | Animation debug | Animation offline debugger | Binary stream format. Contains: 16b - File version defined in AnimDebuggerSerializationVersion, list of all debugged descriptors, for each frame list of all debug instances | There is no backward version compatibility as binary format is hard to maintain. File can be loaded in animation editor using Live debug tab |
| .ae | Animation Events Table | Animation editor | Lists all events used in the animation workspace. StepEvent, SoundEvent, SynchronizationEvent, etc |  |
| .aex | Animation Expression Table | Animation editor | Contains shared expressions which can be used in various animation nodes |  |
| .afm | Audio Final Mixer | Audio Editor | "Sound mixer" referenced by the .acp. Takes all the sound inputs and produces the final sound output |  |
| .agf | Animation graph file | Animation editor | Contains sheets (that contain Animation Nodes) of an animation graph |  |
| .agr | Animation graph | Animation editor | "Root format" sheets collection with graph parameters, e.g commands, variables, IK chains, etc. Depends on **.ast** |  |
| .anm | Animation file | Resource Browser + various plugins | Binary animation file |  |
| .apr | Animation export profile | External editor | File used to retarget bones on export from Motion Builder and Blender | This file is not needed during the runtime, yet it could be handy for modders to have access to the example ones |
| .asc | Height map (Esri grid) | World editor | Height map file which can be exported or imported in Workbench (Terrain tools in the World Editor) | Not used in runtime neither directly by the editor - only for import/export |
| .asi | Animation Set Instance | Animation editor | Bridges **.anm** with **.ast** abstract names. Depends on **.ast** |  |
| .ast | Animation Set Template | Animation editor | Structural skeleton to be filled by **.asi**. Provides a list of entries to be filled |  |
| .asy | Animation Sync Table | Animation editor | TODO |  |
| .aw | Animation Workspace | Animation editor | Workbench's Animation Editor "project" - saves/opens current workspace |  |
| .bt | Behavior tree | Behavior editor | AI behaviour tree |  |
| .bterr | Terrain data | World editor | Terrain data, for editor | General terrain height map editor info |
| .bttile | Terrain data | World editor | Terrain data, height-map for tile for editor | Height map data for terrain tile used by editor (original imported height map without modifications by roads etc) |
| .c | Enforce script | Script Editor | Enforce source file | Different extension should/could be considered |
| .conf | Generic config | Config editor |  |  |
| .ct | Component Template | World Editor | Defines an Entity Component | A different extension should/could be considered (.prefab) |
| .dds | Texture | Art tools | Source image format | Also other formats are supported like png, jpegs, tiffs, tga, hdr |
| .desc | Terrain dialogs configuration | World editor | Configuration of terrain texture import/generating dialogs (height map, satellite texture, normal map) | Not used in runtime |
| .dlg | Dialogue config | Dialogue Editor | Configuration file representing one dialogue scene |  |
| .edds | Enfusion DDS Texture | Resource Browser | Enfusion proprietary (extended) compressed DDS texture |  |
| .emat | Material Definition | Resource Manager | Defines material for 3D objects, terrain, post processes, water, sky objects etc. |  |
| .ent | World Scene | World Editor | World contains terrain and all other objects that make up your game world | A different extension should/could be considered |
| .et | Entity Template | World Editor | Defines an Entity content | A different extension should/could be considered (.prefab) |
| .fbx | 3D model source file | 3D software | Source format for the TXO and XOB |  |
| .fbxa | Animation source file | 3D software | Just renamed FBX file, but Workbench will import it as an animation, not 3D model |  |
| .fnt | Font | Font Editor | Font for UI widget system, binary format |  |
| .gamemat | Game material | Config Editor | Definition of in-game material (properties, sound, etc) | Game-specific resource |
| .gproj | Project-specific Settings | World Editor | Every Project has its own settings stored here | Different extension should/could be considered |
| .imageset | ImageSet Definition | Resource Browser | Defines a set of images by cuts from a texture |  |
| .layer | World Layer | World Editor | World file (.ent) can have multiple layers stored in separate .layer files | Cannot be opened as is, only thru the ent file |
| .layout | Layout Definition | Resource Browser | UI layout definition |  |
| .meta | Meta Information | Resource Browser | Every registered asset has its meta file containing asset-specific settings and asset GUID |  |
| .nmn | Navmesh | Navmesh generator tool | Binary file that is used as a header for navmesh stored in .ntile |  |
| .ntile | Navmesh data | Navmesh generator tool | Navmesh data, multiple navmesh tiles per ntile, multiple .ntile files per navmesh | Number of tiles is at most 100 in one .ntile |
| .pap | PA project | Procedural Animation Editor | Procedural animation project file |  |
| .physmat | Physics material | Config Editor | Physical material (bounciness, adherence etc) |  |
| .pre | ResourceViewer Preset | Resource Browser | A preset with several options to allow customization of Workbench's preview windows | Presets are usually saved in user profile /profile/. Should **not** be committed to SVN since its local config |
| .ptc | Particle System Definition | Particle Editor | Particle effects definition |  |
| .ragdoll | Ragdoll Definition | Resource Browser | Character ragdoll file | No preview at the moment |
| .rdb | Resource Database | Workbench | Cache file of all resources in the project | This file is being created automatically |
| .rnw | AI Road Network | Road Network tool | Describes state of AI road network | Main use to store road blocks and restore them |
| .sig | Signal | Audio Editor | Describes the logic of playing an audio component or encapsulate manipulation parameters | Audio resource shared among audio components |
| .siga | PA signal | Procedural Animation Editor | Describes the control logic for parts of a PA project | A resource shared among PA projects |
| .smap | Soundmap data | World Editor | Information for music and audio effects |  |
| .smd | Shore Map Data | World Editor | Contains data for generated shore map for given world | File has same name as the world and is placed in the world root directory |
| .snd | Sound Data | Audio Editor | Runtime audio file compiled from a .wav source | Contains compressed data of supported formats (Opus) or uncompressed WAV data |
| .st | String Table | String Editor | Strings used for localization | Not used in the runtime |
| .stars | Database | Proprietary format | Runtime database of stars. Contains up to 100,000 stars describing their real positions and colors | Legacy resource type from Carrier Command. Does not use BaseSource (loaded using IFF methods). [Source data](https://cdsarc.u-strasbg.fr/viz-bin/Cat?I/311). Array of Hunks (ID = "ITEM"), each describing a Star: Ra (float), Dec (float), ProMoRa (float), Parallax (float), Magnitude (float), Color (uint32 RGBA), HIP Catalog (uint32, unused), Name (max 256 chars) |
| .styles | Widget Style Definition | Resource Browser | Defines UI style |  |
| .terr | Terrain data | World editor | Terrain data | General terrain height map info |
| .topo | Topography data | World editor | Information for 2D map |  |
| .ttf | TrueType Font | Resource Browser | Source font file format supported by Enfusion |  |
| .ttile | Terrain data | World editor | Terrain data, height-map for tile | Height map data for terrain tile used in run time (here are baked modifications by roads etc) |
| .txa | Animation file | Animation editor | Text animation file (source for .anm) | **Do not distribute in binary build** |
| .txo | Model | Resource Browser + various plugins like BlenderPlugin | Text model file (source for .xob) | **Do not distribute in binary build** |
| .vhcsurf | Vehicle surface | Config editor | Surface properties that are used inside vehicle simulation (roughness, steering noise etc.) |  |
| .wav | Source audio file | Audio software | [WAV](https://en.wikipedia.org/wiki/WAV) source file, compiled to .snd for runtime use | **Do not distribute in binary build** |
| .wst | World streaming data | Resource Browser | Binary file, containing streamable entity data | Optional file for worlds, with enabled streaming (streamBlock property in GenericWorldEntity) |
| .xob | Model | Resource Browser | Binary model file |  |
