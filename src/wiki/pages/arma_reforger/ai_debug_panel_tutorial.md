---
title: Arma Reforger:AI Debug Panel Tutorial
url: https://community.bistudio.com/wiki/Arma_Reforger:AI_Debug_Panel_Tutorial
---

The AI Debug Panel is an important tool for AI debug, allowing to see in real time an AI's condition.

## Usage

Use the [scrDefine](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#scrDefine) startup parameter:

```
-scrDefine AI_DEBUG
```

Select a Character or a Group with [Arma Reforger:Game Master](https://community.bistudio.com/wiki/Arma_Reforger:Game_Master). Open the [Arma Reforger:Diag Menu](https://community.bistudio.com/wiki/Arma_Reforger:Diag_Menu), select **AI → AI Script → Open Debug Panel**. It will create a debug panel for this AI Agent.

## Debug Panel

A debug panel is assigned to only one AI Group/Character and shows data about it.

### Available Data

| Data | Available for |  |
| --- | --- | --- |
| Data | **Groups** | **Characters** |
| Call sign | Yes | Yes |
| Current action | Yes | Yes |
| List of available actions and their priority | Yes | Yes |
| Threat level | No | Yes |
| Unit roles | No | Yes |
| Unit states | No | Yes |

### Available Buttons

#### Dump Debug Msgs

Dumps all events of this AI. You can also select a time interval.

#### Breakpoint

[Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor) will hit a breakpoint on this AI's next update.

#### Locate

*Character only*

Press to locate the selected AI in the world.

## Add Debug Panel Information

To add more information to the debug panel, mod the `SCR_AIAgentDebugPanel` class.

Please note that the same class is used for both Groups and Characters!
