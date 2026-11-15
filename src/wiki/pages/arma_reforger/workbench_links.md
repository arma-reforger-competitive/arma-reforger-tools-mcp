---
title: Arma Reforger:Workbench Links
url: https://community.bistudio.com/wiki/Arma_Reforger:Workbench_Links
---

A Workbench link is a protocol link that allows to share a direct point to a resource/script/game world location.

**Important:** Using a Workbench link requires registering the `enfusion://` protocol in Windows, clicking the `Register enfusion:// protocol button` button in [Workbench Options > Workbench tab](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Options#Register_.22enfusion:.2F.2F.22_protocol).

## Format

A Workbench link is composed of the `enfusion://` protocol followed by the Module name to open; eventually an addon ID, the relative file path and eventual parameters separated by a semicolon:
- enfusion://ResourceManager**/**~ArmaReforger:Configs/Factions/BLUFOR.conf
- enfusion://ScriptEditor**/**scripts/Game/Editor/Containers/UIInfo/SCR_UIInfo.c**;**8
- enfusion://WorldEditor**/**worlds/arland/arland.ent**;**3458.4,34.5587,2820.21**;**-15.107,297.881,0**;**0.4375,1989,8,20**;**46247

| Module | Module Name | Base Link | Parameters |
| --- | --- | --- | --- |
| [Arma Reforger:Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager) | ResourceManager | enfusion://ResourceManager | n/a |
| [Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor) | ScriptEditor | enfusion://ScriptEditor | - file line number |
| [Arma Reforger:World Editor](https://community.bistudio.com/wiki/Arma_Reforger:World_Editor) | WorldEditor | enfusion://WorldEditor | - camera's world position (x, y, z)<br>- camera's angles (x, y, z) - angles in -180..+180 range, z is always 0<br>- daytime (in range 0..1), year, month, day<br>- entity ID - to select a specific entity |
| The following modules do not support a file parameter - a link can only be used to open the associated editor. |  |  |  |
| [Arma Reforger:Particle Editor](https://community.bistudio.com/wiki/Arma_Reforger:Particle_Editor) | ParticleEditor | enfusion://ParticleEditor | rowspan="6" n/a |
| [Arma Reforger:Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Editor) | AnimEditor | enfusion://AnimEditor |  |
| [Arma Reforger:Audio Editor](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor) | AudioEditor | enfusion://AudioEditor |  |
| [Arma Reforger:Behavior Editor](https://community.bistudio.com/wiki/Arma_Reforger:Behavior_Editor) | BehaviorEditor | enfusion://BehaviorEditor |  |
| [Arma Reforger:String Editor](https://community.bistudio.com/wiki/Arma_Reforger:String_Editor) | *Localization*Editor | enfusion://LocalizationEditor |  |
| [Arma Reforger:Procedural Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Procedural_Animation_Editor) | ProcAnimEditor | enfusion://ProcAnimEditor |  |

**Note:** A link can be found prefixed with `https://enfusionengine.com/api/redirect?to=`;
the [Enfusion Engine website](https://enfusionengine.com) provides a redirection for platforms that do not see `enfusion://` as a valid protocol (e.g Discord).

## Link Creation

### Arma Reforger:Resource Manager

Create a link by clicking on any resource in **Resource Browser** with [RMB] then selecting **Copy Link** option from the context menu.

### Arma Reforger:Script Editor

Create a link by selecting from the top menu *Edit →* **Copy link option** or by using the [Ctrl+Shift+L] shortcut (can be changed in the [shortcuts options section](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Options#Shortcuts))

### Arma Reforger:World Editor

Create a link by selecting from the top menu *Game →* **Copy view link** option or by using the [Ctrl+Shift+L] shortcut (can be changed in the [shortcuts options section](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Options#Shortcuts))
