---
title: Command line interface (CLI)
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_CLI.html
---

Some command line parameters are only available on certain configurations denoted by preprocessor symbols enabled when building those configurations:

- **public** - always enabled, including public retail builds of workbench and game used by players. *This doesn't mean that we should communicate this parameter to players.*
- **ENF_DEVELOPER** - enabled for diag executables and Workbench

## Common

### File Paths

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -gproj FILE_FULL_PATH | public |  | Full path to the game project file (defines must have metadata about the engine and game like filesystems, input manager config files, physics settings etc.). The path may be relative to the working directory, but must begin with './'. |
| -gprojConfig CONFIG_NAME | public |  | Forces to load specific configuration (PC, PS4, XBOX_ONE etc.) from the gproj. |
| -cfg USER_ENGINE_SETTINGS_PATH | public |  | Forces to load specific user engine settings config (things like graphics settings, window position and size etc) |
| -language LANGUAGE_CODE | public |  | Set language of the game by language code (e.g. cs_cz), supported language codes depends on StringTable which is set in gproj. |

### Job system

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -jobsysShortWorkerCount NUM | public |  | Set the number of threads working on short jobs (jobs that must finish in one update loop). Value is limited to at most number of CPUs or internal engine limits, whichever is lower on given system. |
| -jobsysLongWorkerCount NUM | public |  | Set the number of threads working on long jobs (jobs that can span multiple iterations of update loop). Value is limited to at most number of CPUs. |

### Window management

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -posX PIXEL_COORD | public |  | Initial position of the window on screen. |
| -posY PIXEL_COORD | public |  | Initial position of the window on screen. |
| -screenWidth PIXEL_SIZE | public |  | Size of the window on screen. |
| -screenHeight PIXEL_SIZE | public |  | Size of the window on screen. |
| -window | public | *N/A* | Starts GameApp in a windowed mode. |
| -forceUpdate | public | *N/A* | Forces the application to render and update even when window looses focus. |
| -noFocus | public | *N/A* | Prevent window focus stealing on game initialization. |
| -vsync X | public | off = force off, on = force on | Force display vertical synchronization regardless on user setting, useful e.g. for autotests where we need to perform time measures. |

### Crash Reporter

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -silentCrashReport | public | *N/A* | Crash Report Dialog is suppressed and automatically accepted to report into Slack and Crashstash without user input/message. |
| -disableCrashReport | public | *N/A* | Crash Report will be automatically canceled. |
| -minidump TYPE | ENF_DEVELOPER | **Normal**, DataSegs, FullMemory | Will set different flags for minidump generation to include additional data in minidumps. Values are not case sensitive and correspond with: **MiniDumpNormal**, **MiniDumpWithDataSegs** and **MiniDumpWithFullMemory**. [Reference](https://learn.microsoft.com/en-us/windows/win32/api/minidumpapiset/ne-minidumpapiset-minidump_type#constants) |

### Logging and User data

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -logAppend | public | *N/A* | Changes a default cleanup of the log files during engine startup to append method. |
| -logFS | public | *N/A* | Creates filesystem.log and logs every read/write operation. It is IO demanding and can produce quite a big file in a short time. |
| -logLevel LOG_LEVEL | public | spam *(lowest)*, verbose, debug, normal, warning, error, fatal *(highest)* | Set the highest use log level shown in the console. Log levels with higher priority are included as well. For example, using *verbose* will also enable *debug*, *normal*, *warning*, *error*, and *fatal*. |
| -logTime FORMAT | public | none, **time** (default), datetime | Sets format of logged time per line. |
| -logStats INTERVAL | public | 60 | Logs performance and other statistics into the console log. When no interval is provided, the 60 seconds default is used. |
| -logUiNonLocalized | ENF_ENABLE_DIAG |  | Enable DiagMenu → UI → Log Non Localized option to log all non localized texts passed to visible text widgets. |
| -profile RELATIVE_PATH / -profile ABSOLUTE_PATH | public |  | Set path to directory under which profile directory is located. This profile directory contains various user's data, configs, temporary files etc. Final location depends on platform and whether relative or absolute path is used. Any intermediate directories that do not exist will be created. |
| -logsDir NAME / -logsDir PATH | public |  | Set name of folder where game instance specific data are stored like logs, memory dumps, temporary files etc. <PROFILE_PATH>/logs/<NAME> or you can set directly path. |
| -keepNumOfLogs NUMBER | public | 10 | Set number of how many logs are kept before they are automatically removed. Default: 10 |

### Script and Virtual machine

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -debugger ADDRESS | public | localhost | Sets the script debugger to specific address. localhost is default. |
| -debuggerPort PORT | public | 1000 | Sets the script debugger to specific port. 1000 is default. |
| -VMErrorMode MODE | public | "silent", "log_only", "full", "fatal" | Sets script virtual machine error reporting mode. "silent": just silently continue. "log_only": prints error into log, and continue. "full": show VME dialog, prints error into log. "fatal": prints error into log, and force exit the game. |
| -noThrow | public | *N/A* | Disable all kinds of error message dialogs. (VME's, asserts, crashes, ...) |
| -scriptExtProfiler | public | *N/A* | Turn on external profiler support for script VM. |
| -checkInstance | public | *N/A* | Turn on memory allocations logging in script VM. |
| -configuration CONFIGURATION_NAME | public |  | Force script defines from script configuration (can be found in project settings, it is usually pc,xbox,ps4...) instead of hard-coded ones. |
| -scrDefine DEFINE_NAME | public |  | Add define for to script compiler (mutually exclusive with -configuration). |
| -scrNoInline | ENF_DEVELOPER | *N/A* | Disable inline optimization (for debug purposes). |

### Replication

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -rpl-timeout-disable | ENF_DEVELOPER |  | The client/server timeout will be disabled. **Warning: If a connection will stop responding the server performance will start deteriorating until it will eventually run out of memory and crash. Strictly for debugging purposes.** |
| -rpl-timeout-ms TIME | public |  | Sets the timeout value in milliseconds. |
| -rpl-reconnect | ENF_DEVELOPER |  | Instead of immediately raising connection failure the replication tries to reconnect. **Warning: Endless reconnection loop is designed for debug only!** |
| -rpl-vcons NUMBER | ENF_DEVELOPER | 0..200, default 0 | Number of virtual connections (fake connections creating artificial load on replication). |
| -rpl-validation-rdb-disable | ENF_DEVELOPER |  | Disables the validation of clients resource database. **DEVELOPER ONLY!** |
| -rpl-validation-scr-disable | ENF_DEVELOPER |  | Disables the validation of clients scripts checksum. **DEVELOPER ONLY!** |
| -rpl-validation-version-disable | ENF_DEVELOPER |  | Disables the validation of clients executable version. **DEVELOPER ONLY!** |
| -rpl-validation-devbin-disable | ENF_DEVELOPER |  | Disables the validation of developer binary flag, allowing mixing of developer and non-developer binaries in same multiplayer session. |
| -rpl-validation-addons-disable | ENF_DEVELOPER |  | Disables the validation of loaded addons checksum which validates missing/additional addons or incorrect load order. |
| -nds NUMBER | public |  | Network Dynamic Simulation (nds) is a server feature that only streams in relevant replicated entities for each client. NUMBER stands for diameter, default is 2 in each direction. Use "-nds 0" to turn off. |
| -nwkResolution NUMBER | public |  | What resolution Spatial Map cells should be set at. Cannot go under 100m or above 1000m. |
| -staggeringBudget NUMBER | public | 1..10201, default **-nds** value | How many stationary spatial map cells are allowed to be processed in one tick. |
| -streamingBudget NUMBER | public |  | Global streaming budget equally distributed between all connections. Cannot go under 100. |
| -streamsDelta NUMBER | public |  | A tool to limit the amount of streams being opened for a client. Default 100, must be between 1 and 1000. |

### Debug

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -maxFPS FPS | public |  | Maximum limiter of FPS, useful when need to lock some maximum frame rate. |
| -createDB | public |  | Force regenerate file database. Useful after SVN update, when some resources were moved elsewhere. |
| -nosound | public |  | All sound play/update request fails. AudioSystem exists but does nothing. |
| -EnableWARP | public |  | Allow creating [WARP](https://docs.microsoft.com/en-us/windows/win32/direct3darticles/directx-warp) device for DX12 rendering backend. Does not force WARP, just allows it. |
| -GPUAdapter N | public |  | Force using graphics adapter on index N. |
| -log-rdb-checksum | public |  | Enable logging detailed info about RDB checksum computation. |
| -log-scr-checksum | public |  | Enable logging of all script files used in compilation and their checksums. |
| -SingleThreadedUpdate | public | *N/A* | Disable multithreaded update. |
| -diagMenu PATH | ENF_ENABLE_DIAG |  | Override filename with stored diag menu state (F1 in diag menu - save, F2 - load). |
| -freezeCheck TIME | public | 0..600, **300** | Override time in seconds to forcefully crash on application freeze or completely disable detection. Default: 300 (5 min). Max: 600 (10 min). Disabled: 0. |
| -freezeCheckMode MODE | public | **crash**, minidump | Change behavior on freeze detection. "crash" - force crash. "minidump" - generates minidump and waits (Windows only). Default is "crash", except Workbench which uses "minidump". |

### Backend

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -config PATH | public |  | Provide user configuration with server environment setup, addon list and other parameters in JSON format. |
| -bindIP IP | public |  | Override gameHostRegisterBindAddress and gameHostRegisterAddress values from config file. |
| -bindPort PORT | public |  | Override gameHostBindPort and gameHostRegisterPort values from config file. |
| -noBackend | public |  | All backend related and possible scripted http communication is suppressed. |
| -backendFreshSession | public | *N/A* | Skip initial load request from DS configuration file or from script. |
| -backendLocalStorage | public | *N/A* | Override to force Save & Load to work with local JSON files in profile folder instead of backend. |
| -backendDisableStorage | public | *N/A* | Disable use of storage, no loads and saves can be performed both online and local. |
| -a2sIpAddress ADDRESS | public |  | Set Bind IP Address for Steam Query Protocol. |
| -a2sPort PORT | public |  | Set Bind Port for Steam Query Protocol. |
| -steelShieldEnable KEY | public | Project specific key (string). Optional! | Enable Nitrado Steel Shield protection with optional project key. If key is not present, internal baked-in key from setup is used. |
| -agones | public | *N/A* | Enable Agones platform (SDK). |
| -agonesAlwaysAllocated | public | *N/A* | Enable Agones to keep Allocated state. Graceful Shutdown stays enabled. |

### Platform Service

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -username | ENF_DEVELOPER |  | Allow to override username in platform service. Can be used to allow multiple local clients on same server for development testing purposes. |

### Modding

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -addons ID1,ID2,ID3 | public |  | List of addons separated by ','. IDs specified in their gproj files. |
| -addonsDir PATH1,PATH2,PATH3 | public |  | Add directory where addons are looked for. |
| -addonDownloadDir PATH | public |  | Specify the directory where to download addons from in-game or Workbench. |
| -addonTempDir PATH | public |  | Specify the directory for temporary content. |
| -addonsVerify | public | *N/A* | Verifies the integrity of all installed addons. Works only on Dedicated Servers. |
| -addonsRepair | public | *N/A* | Verifies integrity and repairs corrupted addons automatically. Works only on Dedicated Servers. |
| -addonsDownloadPool | ENF_DEVELOPER |  | Specifies maximum number of parallel addon downloads. |

### Animation

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -AnimDebugger | ENF_DEVELOPER | *N/A* | Enable animation debugging. |

### AI

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -AILimit NUMBER | public |  | Sets wanted AILimit on AIWorld. |
| -activeAILimit NUMBER | public |  | Sets wanted Active AI Limit on AIWorld. |
| -AIPartialSim NUMBER | public | 4 | Sets in how many batches before all AIs will be processed. |
| -disableNavmeshStreaming NAVMESH_PROJECT1,NAVMESHPROJECT2,... | public |  | Disable navmesh streaming on all navmesh worlds. Specify list or let empty for all. |
| -disableAI | public | *N/A* | Disable AIWorld initialization and ticking. |

## Workbench

### Workbench parameters

| Parameter | Visibility | Description |
| --- | --- | --- |
| -gproj PATH_TO_GPROJ | public | Specific game project path. |
| -rebuild-database-only | public | Workbench automatically exits when the file database is refreshed. |
| -wbSilent | public | Do not open any window, just initialise engine and workbench modules and exit. Can be used to validate engine/game initialization and script compilation. |
| -exitAfterInit | public | Workbench automatically exits once it's completely initialized and all initialization CLI commands are executed. |
| -clearSettings | public | Clear windows registry settings at start. |
| -forceSettings ABSOLUTE_FILE_PATH | public | Forces to use another workbench settings from a file instead of usual settings stored in windows registry. |
| -wbModule=MODULE_NAME | public | Workbench module specifier: ResourceManager, WorldEditor, ParticleEditor, AnimEditor, ScriptEditor, AudioEditor, BehaviorEditor. Note: You need to include -run after each instance of -wbmodule if you want them to launch! |
| -plugin=PLUGIN_NAME | public | Workbench plugin specifier. Plugin parameters **require** an equal sign (e.g -plugin=SCR_Plugin -value1=42). |
| -noGameScriptsOnInit | public | Skip compiling game scripts during Workbench initialization. **A lot of stuff will not work correctly.** |
| -scriptAuthorizeAll | public | Potentially dangerous script commands will be automatically allowed in Workbench. |

### Workbench LSP parameters

| Parameter | Visibility | Possible values / Default | Description |
| --- | --- | --- | --- |
| -lsp | public | N/A | Launch LSP server after workbench starting. After using this CLI command, user can't manual turn off the LSP server and change LSP port. |
| -lspPort PORT | public | 5776 | Workbench uses selected port for providing TCP connection. After using this CLI command, user can't manual turn off the LSP server and change LSP port. 5776 is default. |

### "-wbmodule" and "-plugin" common additional parameters

| Parameter | Visibility | Description |
| --- | --- | --- |
| -run | public | Opens specified workbench module. **Example:** -wbmodule=WorldEditor -run |
| -load FILE_PATH | public | Loads given resource in specified module at startup. **Example:** -wbmodule=WorldEditor -run -load "world/myworld.ent" |

### Module: World Editor

| Parameter | Visibility | Description |
| --- | --- | --- |
| -forceSaveAll | public | Re-saves all entity layers of opened map and all prefabs and configs used in the opened map. |

### Module: WorldEditor Plugin: GenerateFlowMaps

| Parameter | Visibility | Description |
| --- | --- | --- |
| *no params needed* | public | Alternative usage to ResourceManager Plugin: GenerateFlowMaps. By default does not exit when finished and operates upon a loaded world. **Example:** -wbmodule=WorldEditor -run -load "worlds/Eden/Eden.ent" -plugin=GenerateFlowMaps |
| -exitAfter | public | Optional: Exit WB when finished. |
| -world=ENT_FILE_PATH | public | Loads the given world in WE and runs the plugin. Can be combined with -exitAfter. |
| -forceSaveAll | public | Triggers **Force Save All** action available in File menu. |

### Module: WorldEditor Plugin: NavmeshGeneration

| Parameter | Visibility | Description |
| --- | --- | --- |
| -autogenerate [PROJECT_NAMES] | public | Triggers headless navmesh generation on a loaded world. Optional value is a comma-separated list of navmesh project names to build (default ALL). The world to generate for is supplied by -load. **Example:** -wbmodule=WorldEditor -run -load "worlds/myworld.ent" -autogenerate projA,projB |
| -saveDir DESTINATION_FOLDER | public | Optional. Directory where data will be exported to. |
| -logNavmeshGen | public | **This parameter does nothing at the moment as it is always enabled.** |
| -gameSystem | public | Allows to setup the system config with which NavMesh should be generated. |

### Module: ResourceManager

| Parameter | Visibility | Description |
| --- | --- | --- |
| -builddata |  | Builds all data of one addon to a specified folder. Two mandatory parameters: **PlatformName** and **TargetFolder**. Third **AddonName** is optional. Platforms: PC, PC_WB, XBOX_ONE, PS4, HEADLESS. **Example:** -wbmodule=ResourceManager -builddata PS4 "c:\Data\PS4Data" |
| -metaFiles |  | Optional with -builddata. Copies .meta files to the built data. |
| -loadBuiltData |  | Optional with -builddata. Built resource is loaded into memory and possible errors are printed into the log. |
| -includeTypes |  | Optional with -builddata. Specifies a list of file types to include. **Example:** -includeTypes "edds,xob,emat" |
| -excludeTypes |  | Optional with -builddata. Specifies a list of file types to exclude. |
| -filterPath |  | Optional with -builddata. Allows to limit data build to a single folder or file. |
| -filterTags |  | Optional with -builddata. Can be used to exclude/include files based on tagging. Uses '&' (and), '\|' (or), '!' (negation) operators. |
| -resavedata | public | Resaves and reimports all data of one addon. Optional **AddonName** parameter. |
| -packAddon | public | Uses XPacker to pack all data of current addon into an output directory. |
| -packAddonDir DIR | public | Specify output directory for packed PAKs. |
| -packAddonCacheDir DIR | public | Specify folder for compressed and built data cache during packing. |
| -publishAddon | public | Main parameter for addon publishing (updates only, not initial publish). |
| -publishAddonDir DIR | public | Specify output directory for publishing PAKs. |
| -publishAddonCacheDir DIR | public | Specify folder for compressed data cache during publishing. |
| -publishAddonVersion VERSION | public | Specify version number. |
| -publishAddonChangeNote NOTE | public | Specify change note of this update. |
| -publishAddonChangeNoteFile PATH | public | Specify change note loaded from a file. |
| -publishAddonPreviewImage PATH | public | Specify path to preview image file. |
| -publishAddonScreenshots DIR | public | Specify folder for screenshots (scanned for jpg, png, bmp). |
| -wbBackendLogin EMAIL PSW | public | Force another user to be logged in. |
| -imageSetBuild PATH | public | Do ImageSet texture reimport for ImageSet on PATH. Used with -imageSetTexture DIR. |
| -imageSetTexture DIR | public | Works in combination with -imageSetBuild. |

### Module: ResourceManager Plugin: ResavePlugin

| Parameter | Visibility | Description |
| --- | --- | --- |
| -extension=layout |  |  |

### Module: ResourceManager Plugin: ResourceProcessorPlugin

| Parameter | Visibility | Description |
| --- | --- | --- |
| -FileTypes=extension | public | List of extensions separated by comma. **Example:** -FileTypes=xob or -FileTypes="xob,edds" |
| -PathStartsWith=DIR | public | Filter by path prefix. |
| -PathContains=string | public | Filter by path content. |
| -PathEndsWith=string | public | Filter by path suffix. |

### Module: ResourceManager Plugin: WorldDataExport

| Parameter | Visibility | Description |
| --- | --- | --- |
| -type=DATA_TYPE |  | Optional. Types: All, 2DMap, SoundMap, 2DRasterization. Default is "All". |
| -saveDir=DIR |  | Optional. Directory where data will be exported to. |

### Module: ResourceManager Plugin: GenerateFlowMaps

| Parameter | Visibility | Description |
| --- | --- | --- |
| -world=ENT_FILE_PATH |  | Generates flow maps for all rivers of the given world and exits WB afterwards. **Example:** -wbmodule=ResourceManager -plugin=GenerateFlowMaps -run -world="worlds/Eden/Eden.ent" |

### Module: ResourceManager Plugin: BuildShoreMap

| Parameter | Visibility | Description |
| --- | --- | --- |
| -world=ENT_FILE_PATH |  | Generates shore map for given world and exits WB afterwards. **Example:** -wbmodule=ResourceManager -plugin=BuildShoreMap -run -world="worlds/Eden/Eden.ent" |

### Module: ResourceManager Plugin: AddonBuildInfoTool

| Parameter | Visibility | Description |
| --- | --- | --- |
| -addonPath=ADDON_DIR_PATH |  | Required. Absolute path to directory containing .gproj file. |
| -ciInfoDst=DST_FILE_PATH |  | Required. Path where JSON file with exported build information should be saved. |

### Module: ScriptEditor

| Parameter | Visibility | Description |
| --- | --- | --- |
| -validate[=CONFIGURATION_NAME] | public | Checks if game scripts are compilable. Returns "-1" on failure, "0" on success. Default: ALL configurations. **Example:** -wbmodule=ScriptEditor -validate |

### Module: LocalizationEditor

| Parameter | Visibility | Description |
| --- | --- | --- |
| -saveDir ABS_PATH |  | Specify save dir for build command. |
| -build |  | Build runtime string tables from opened string table. **Example:** -wbmodule=LocalizationEditor -run -load {C014582791ECBF24}Language/localization.st -build -saveDir c:\tmp\ |
