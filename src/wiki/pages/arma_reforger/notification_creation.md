---
title: Arma Reforger:Notification Creation
url: https://community.bistudio.com/wiki/Arma_Reforger:Notification_Creation
---

This page explains how to create and declare a new notification for it to be used anywhere.

## Enum Declaration

Open the `ENotification` enum and add a new entry to the list.
Make sure that it has a unique int assigned to it! Also please add a description to it so people know what the notification is about and what variables it needs.

## Config Modding

Notifications are configured in Notifications.conf (`enfusion://ResourceManager/~ArmaReforger:Configs/Notifications/Notifications.conf`).
This config holds all the notifications and is added to the `PlayerController`.

### Add a Notification

Press the **+** button to add a new notification to the config.

Below is a list of *most* of the notification classes (some may be missing and classes for specific notifications will not be included).
Use this table to choose the correct Class and add it to the list. Outside of [Game Master](https://community.bistudio.com/wiki/Arma_Reforger:Game_Master) you would most likely need one of the following:

- `SCR_NotificationDisplayData`
- `SCR_NotificationPlayer`
- `SCR_NotificationPlayerTargetPlayer`

| Class | Description | Parameters | Format | Code Example |
| --- | --- | --- | --- | --- |
| `SCR_NotificationDisplayData` | A simple notification targeting nothing and displaying text | None | n/a | `SCR_NotificationsComponent.SendToEveryone(ENotification);` |
| `SCR_NotificationEditableEntity` | A notification that can hold the data of an EditableEntity | - Param1: EditableEntityID | - `%1`: EditableEntity Name | `SCR_NotificationsComponent.SendToEveryone(ENotification, editableEntityID);` |
| `SCR_NotificationEditableEntityEditableEntityTarget` | A notification that can hold the data of two EditableEntities | - Param1: EditableEntityID<br>- Param2: Target EditableEntityID | - `%1`: EditableEntity Name<br>- `%2`: Target EditableEntity Name | `SCR_NotificationsComponent.SendToEveryone(ENotification, editableEntityID, targetEditableEntityID);` |
| `SCR_NotificationEditableEntityTargetPlayer` | A notification that can hold the data of an EditableEntity and target Player | - Param1: EditableEntityID<br>- Param2: Target PlayerID | - `%1`: EditableEntity Name<br>- `%2`: Target Player Name | `SCR_NotificationsComponent.SendToEveryone(ENotification, editableEntityID, targetPlayerID);` |
| `SCR_NotificationPlayer` | A notification that can hold the data of a Player | - Param1: PlayerID | - `%1`: Player Name | `SCR_NotificationsComponent.SendToEveryone(ENotification, playerID);` |
| `SCR_NotificationPlayerAndFaction` | A notification that can hold the data of a Player and its faction | - Param1: PlayerID | - `%1` Player Name<br>- `%2` Faction Name | `SCR_NotificationsComponent.SendToEveryone(ENotification, playerID);` |
| `SCR_NotificationPlayerTargetEditableEntity` | A notification that can hold the data of an Player and target EditableEntity | - Param1: PlayerID<br>- Param2: Target EditableEntityID | - `%1` Player Name<br>- `%2` Target EditableEntity Name | `SCR_NotificationsComponent.SendToEveryone(ENotification, playerID, targetEditableEntityID);` |
| `SCR_NotificationPlayerTargetPlayer` | A notification that includes a Player and target Player | - Param1: PlayerID<br>- Param2: Target PlayerID | - `%1`: Player Name<br>- `%2`: Target Player Name | `SCR_NotificationsComponent.SendToEveryone(ENotification, playerID, targetPlayerID);` |
| `SCR_NotificationNumber` | A notification that includes a number (int).<br>**Note:** The number it takes is an Int.<br>You can however multiply a float to a max of 1000 (10 means float: 0.0, 100: 0.00 and 1000: 0.000) and set the Number Divider in the Notification to the same as you multiply the number to display it correctly. | - Param1: Number<br>- Param2: Number (Optional)<br>- Param3: Number (Optional)<br>- Param4: Number (Optional)<br>- Param5: Number (Optional) | - `%1`: Number | `SCR_NotificationsComponent.SendToEveryone(ENotification, number);`<br>`// up to`<br>`SCR_NotificationsComponent.SendToEveryone(ENotification, number1, number2, number3, number4, number5);` |
| `SCR_NotificationPlayerAndNumber` | A notification that includes the player name and a number (int).<br>**Note:** The number it takes is an Int.<br>You can however multiply a float to a max of 1000 (10 means float: 0.0, 100: 0.00 and 1000: 0.000) and set the Number Divider in the Notification to the same as you multiply the number to display it correctly. | - Param1: PlayerID<br>- Param2: Number<br>- Param3: Number (Optional)<br>- Param4: Number (Optional)<br>- Param5: Number (Optional) | - `%1`: Player Name<br>- `%2`: Number | `SCR_NotificationsComponent.SendToEveryone(ENotification, PlayerID, number);`<br>`// up to`<br>`SCR_NotificationsComponent.SendToEveryone(ENotification, PlayerID, number1, number2, number3, number4);` |

**Note:** See New Notification System Class to find out how to create a new class.

### Set the Key

Now scroll to the newly created notification and set the Notification Key to the enum created earlier.
Each notification needs to have its own unique key as it is used to get the Notification information from a database.

### Fill the Info Class

Next up click the button to create the Info class. There are currently three classes from which to choose.

| Class | Description |
| --- | --- |
| `SCR_UINotificationInfo` | The default notification |
| `SCR_ColoredTextNotificationUIInfo` | Similar to the default notification but the text can be colored |
| `SCR_SplitNotificationUIInfo` | A split notification. It has text left and right of the icon. Think of a "player kills player" notification, e.g: "PlayerKillerName" [Icon] "PlayerKilledName".<br>This one has some additional logic to it explained in Notification Color. |

The **Name** variable is the actual notification that you want to display. What format you can use depends on the class chosen earlier.

For example:
- `SCR_NotificationDisplayData` - only takes plain text
- `SCR_NotificationPlayer` - displays `%1` as player name
- `SCR_NotificationPlayerTargetPlayer` - displays `%1` as player name and `%2` as target player name

**SCR_ColoredTextNotificationUIInfo additional variables**

| Variable | Description |
| --- | --- |
| Notification text color | Works the same as the notification colour. Takes a `ENotificationColor` and instead of colouring the widgets it will color the text. |

**SCR_SplitNotificationUIInfo additional variables**

| Variable | Description |
| --- | --- |
| Name | Unlike the default notification the Name in the UI info is the left side of the split message. The center is the icon and right is the right message. It still works similar to the normal message and can take params. |
| Split Notification Right Message | The right side of the split notification message. It still works similar to the normal message and can take params. |
| Left notification Text color | Works very similar to Notification text color in the ColoredTextNotificationUIInfo as it will color the text but only the **left** part. If using any faction colored it will use Param1 to check faction color instead of using param2 (target). Only relevant for targeted notifications. |
| Right notification Text color | Works very similar to Notification text color in the ColloredTextNotificationUIInfo as it will color the text but only the **right** part. If using any faction colored it will use param2 (target) to check the color. Only relevant for targeted notifications. |
| Replace Left Color With Right Color If Factions Are Ally | A mouthful but as descriptive as they get. If any of the FACTION specific `ENotificationColor` enums are used it AND there are two entity targets (player or other entity) then it will check. Is param1 entity an ally of param2 entity. If they are allied then the color of param1 becomes the same as param2. This is to show for example teamkills. |

#### Notification Colour

Notifications colours are listed in an enum found in `ENotificationColor`.
The enum is a key used to get the information data.
In general the colors are straightforward though the `FACTION_RELATED_FRIENDLY_IS_NEGATIVE` and `FACTION_RELATED_FRIENDLY_IS_POSITIVE` have some additional logic attached to them - see below.

| Enum Value | Description |
| --- | --- |
| `NEUTRAL` | A neutral-coloured notification |
| `WARNING` | A warning-coloured notification |
| `POSITIVE` | A positive-coloured notification |
| `NEGATIVE` | A negative-coloured notification |
| `GM` | A Game Master-colored notification which all players can see |
| `GM_ONLY` | A Game Master-only coloured notification which all Game Masters can see (and only them) |
| `FACTION_FRIENDLY_IS_NEGATIVE` | If using Notification class that has a player or EditableEntity then it will check is the Entity faction (or if there are two entities then the target entity) is friendly to the player that receives the notification.<br>If friendly the notification will get the `NEGATIVE` color else it will get `POSITIVE`.<br>If either the player that receives the notification or the entity that is checked has no faction then it will display `NEUTRAL`.<br>Think of "player died".<br>- If friendly it is negative<br>- If enemy it is positive |
| `FACTION_FRIENDLY_IS_POSITIVE` | If using Notification class that has a player or EditableEntity then it will check is the Entity faction (or if there are two entities then the target entity) is friendly to the player that receives the notification.<br>If friendly the notification will get the `POSITIVE` color else it will get `NEGATIVE`.<br>If either the player that receives the notification or the entity that is checked has no faction then it will display `NEUTRAL`.<br>Think of new player joined the same faction as the player that receives the notification.<br>- If new friendly player joined it is positive<br>- If new enemy player joined it is negative |
| `FACTION_ENEMY_IS_NEGATIVE_ONLY` | Similar to `FACTION_FRIENDLY_IS_NEGATIVE` but will set color to `NEUTRAL` if faction is not friendly |
| `FACTION_FRIENDLY_IS_POSITIVE_ONLY` | Similar to `FACTION_FRIENDLY_IS_POSITIVE` but will set color to `NEUTRAL` if faction is not friendly |

#### Editor Set Position Data

This is an editor only setting and will essentially allow the Game Master to move to the notification's target.
This is really only relative if the notification has a position attached to it or an entity (Player or EditableEntity).
In most cases it is fine to keep it on default: `AUTO_SET_AND_UPDATE_POSITION`. The table below explains what each means.

| Enum | Description |
| --- | --- |
| `AUTO_SET_AND_UPDATE_POSITION` | If the notification has a target then the position is set and updated; if the target moves, so does the location attached to the notification.<br>If the Game Master clicks on the notification the camera will go to the current location of the target. |
| `AUTO_SET_POSITION_ONCE` | The position of the notification is only set once  when the notification is created and will never update.<br>This is used for notifications such as "Player died". The Game Master does not care about the current location of that player but would want to see where the player died. |
| `NEVER_AUTO_SET_POSITION` | Never set the notification position. This is used for positionless events, e.g if a player joins or leaves - the position is irrelevant and does not need to be set. |

## Send Notification

The only thing left is sending out the notification now that the notification data is created.

To do this call one of the following `SCR_NotificationsComponent` methods:

| Method | Server Only | Description | Parameters | Example |
| --- | --- | --- | --- | --- |
| `SCR_NotificationsComponent.SendToEveryone();` | Yes | Sends to notification to every player | - First variable is the ENotification key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendToEveryone(ENotification, NotificationParameters);` |
| `SCR_NotificationsComponent.SendToPlayer();` | Yes | Send the notification to specific players | - First variable is the specific Player ID the notification needs to be send to<br>- Second variable is the `ENotification` key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendToPlayer(PlayerID, ENotification, NotificationParameters);` |
| `SCR_NotificationsComponent.SendLocal();` | No | Sends the notification locally | - First variable is the ENotification key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendLocal(ENotification, NotificationParameters);` |
| `SCR_NotificationsComponent.SendToGameMasters();` | Yes | Sends the notification only to all game masters | - First variable is the `ENotification` key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendToGameMasters(ENotification, NotificationParameters);` |
| `SCR_NotificationsComponent.SendToGameMastersAndPlayer()` | Yes | Sends the notification to all game masters as well as the given player ID regardless if it is GM or not | - First variable is the specific Player ID the notification needs to be send to along with the GMs<br>- Second variable is the `ENotification` key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendToGameMastersAndPlayer(PlayerID, ENotification, NotificationParameters);` |
| `SCR_NotificationsComponent.SendLocalGameMaster();` | No | A combination of `SendLocal()` and `SendToGameMasters()`. Sends the notification local but player will only receive it has GM rights | - First variable is the `ENotification` key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendLocalGameMaster(ENotification, NotificationParameters);` |
| `SCR_NotificationsComponent.SendLocalNonGameMaster();` | No | Similar to `SendLocalGameMaster()` but the player will only receive the notification if they are NOT game master | - First variable is the `ENotification` key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendLocalNonGameMaster(ENotification, NotificationParameters);` |
| `SCR_NotificationsComponent.SendToGroup();` | Yes | Sends a notification to all players in a given group | - First variable is the specific group ID the players need to be a part of to receive the notification<br>- Second variable is the `ENotification` key<br>- Other variables depend on the `SCR_Notification*` class chosen earlier | `SCR_NotificationsComponent.SendToGroup(groupID, ENotification, NotificationParameters);` |

## Create a New SCR_Notification

It is possible to create a new notification class by creating a class that inherits from `SCR_NotificationDisplayData`.
This class already has a lot of function most of which you do not have to worry. Below in the table you find three of the most used.
In general however you can simply inherit from `SCR_NotificationPlayer`, `SCR_NotificationPlayerTargetPlayer` or `SCR_NotificationPlayerAndFaction` and override the `GetText()` method.

| Method | Description |
| --- | --- |
| `GetText()` | This returns the string that is actually displayed and sets the correct formatting such as getting the player name and adding it.<br>Simple example of `SCR_NotificationPlayerTargetPlayer`:<br>`int playerID, playerTargetID;`<br>`data.GetParams(playerID, playerTargetID);`<br>`return string.Format(m_info.GetName(), GetPlayerName(playerID), GetPlayerName(playerTargetID));`<br>This sets `%1` to the name of the player with *playerID* and `%2` to the name of the player with *playerTargetID*. |
| `SetFactionRelatedColor()` | If the `FACTION_RELATED_FRIENDLY_IS_NEGATIVE` or `FACTION_RELATED_FRIENDLY_IS_POSITIVE` colors are used then the notification needs to know what they need to check against the notification receiving player faction.<br>`int playerID;`<br>`data.GetParams(playerID);`<br>`data.SetFactionRelatedColor(GetFactionRelatedColorPlayer(playerID, m_info.GetNotificationColor()));`<br>ColoredTextNotifications and SplitNotifications have additional logics for these where they can set the faction color of texts. Check `SCR_NotificationPlayerTargetPlayer` for an example. |
| `SetPosition()` | Editor only, to help out the editor team make sure that the entity position is set or inherit for example from either `SCR_NotificationPlayer` or `SCR_NotificationPlayerTargetPlayer` which sets the position of the player if the first (or second) parameter is a *playerID*.<br>Here is the example found in `SCR_NotificationPlayer`:<br>`int playerID;`<br>`data.GetParams(playerID);`<br>`SetPositionDataPlayer(playerID, data);` |

**Note:**
- There are three notifications message prefab: `BaseNotificationMessage`, `NotificationMessage` and `NotificationMessageSplit`;
- Both `NotificationMessage` and `NotificationMessageSplit` inherit from `BaseNotificationMessage`.

## Localisation

Select Notifications.conf (`enfusion://ResourceManager/~ArmaReforger:Configs/Notifications/Notifications.conf`) in the Resource Browser and open the **Localisation Parser plugin** by pressing [Ctrl+Alt+L] (or click on it in [Resource Manager](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager)'s **Plugins** dropdown menu).

This opens a window where the **Config Path** field should aim at Editor.conf (`enfusion://ResourceManager/~ArmaReforger:Configs/Workbench/LocParser/Editor.conf`).

**Note:**
- It is possible to set the target stringtable with the **String Table Override** field.
- It is possible to override the config's `AR-` translation key prefix using the **Prefix Override** field.

Clicking **Logs** simulates the localisation process and logs results and warnings in the Log Console (press [F1] to display it if hidden).

Clicking **Localize** will edit the translation, taking the values originally set in the field to the *EN-US** translation field with a new localisation key (e.g `PREFIX-Notification_ENUM_NAME`).

**Important:** While the new stringtable translations are automatically saved in [String Editor](https://community.bistudio.com/wiki/Arma_Reforger:String_Editor), be sure to apply the newly created translation keys to the config as this is not done automatically.
