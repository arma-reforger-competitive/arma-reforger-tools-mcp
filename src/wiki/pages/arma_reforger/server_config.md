---
title: Arma Reforger:Server Config
url: https://community.bistudio.com/wiki/Arma_Reforger:Server_Config
---

Arma Reforger servers uses [JSON](https://en.wikipedia.org/wiki/JSON) configuration format. Template and Example are available at the bottom of this page.

**Important:** Two **very important** aspects of dedicated server public hosting:
- Keep fastValidation to `true`
- Limit max FPS with the `-maxFPS` startup parameter in order to save performance

**Note:**
- Parameters inside the JSON file are case-sensitive!
- Values are strings, unless mentioned otherwise.
- For Community config generators, see the following:
  - https://arscfg.johandejong.dev/
  - https://armareforger.se/server-generator/

## Summary

Default ports to be opened for hosting:

| Port | Protocol | Optional | Reason |
| --- | --- | --- | --- |
| 2001 | UDP | No | Public port |
| 17777 | UDP | Yes | A2S port |
| 19999 | UDP | Yes | RCON port |

## Root

### bindAddress

IP address to which the server socket will be bound. In most cases, this should be left empty.
It can be used to restrict connections to a particular network interface. When left out or empty, `0.0.0.0` is used, which allows connections through any IP address.

**Important:**
- This entryshould generally be left out of server config to use default value and should only be set by experienced users with specific network needs.
- IP**v6** is **not** supported by Arma Reforger.

*was **gameHostBindAddress** before 0.9.8.73*

### bindPort

number value, range 1..65535, default: 2001

UDP port to which the server socket will be bound.

**Important:** This entry should generally be left out of server config so that it automatically uses same value as publicPort.
It should only be used by users with specific port forwarding needs.

*was **gameHostBindPort** before 0.9.8.73*

### publicAddress

IP address registered in backend.
This should be set to the public IP address to which clients can connect in order to reach the server (either IP of the server itself or IP of the machine that will forward data to the server).
If the entry is missing or empty, then the public IP address will be automatically detected and used by the backend.

**Important:**
- This entry should generally be left out of server config to let the server automatically detect and use its public address used to reach it from internet.
- It is possible to use the "local" keyword to automatically detect the local address of the network card.
- IP**v6** is **not** supported by Arma Reforger.

*was **gameHostRegisterBindAddress** before 0.9.8.73*

### publicPort

number value, range 1..65535, default: 2001

UDP port registered in backend. If the server itself has a public IP address, this should be the same value as in bindPort. Otherwise, this is the UDP port that is forwarded to the server.

*was **gameHostRegisterPort** before 0.9.8.73*

### a2s

Arma Reforger 0.9.9

*0.9.9.31*

Steam Query protocol definition - see the a2s section below.

**Note:** Before Arma Reforger 0.9.9, a2s settings were:
- `a2sQueryEnabled` - bool value, default: false
- `steamQueryPort` - number value, range 1..65535, default: 17777

### rcon

Arma Reforger 1.0.0

The RCON Protocol is a UDP-based communication protocol that allows console commands to be issued to the server via a **R**emote **CON**sole, or RCON - see the rcon section below.

### game

Define the server's settings - see the game section below.

**Important:** Only **one** scenario can be defined - Arma Reforger does **not** allow for mission rotation as of v0.9.8.

### operating

Arma Reforger 0.9.7

Define various server settings - see the operating section below.

## a2s

Arma Reforger 0.9.9

*0.9.9.31*

See [A2S server queries](https://developer.valvesoftware.com/wiki/Server_queries).

### address

required

IP address to which A2S socket will be bound. It can be used to restrict A2S queries to a particular network interface.

### port

number value, range 1..65535, default: 17777

Change Steam Query UDP port on which game listens to A2S requests

## rcon

Arma Reforger 1.0.0

(Optional)

See [RCON protocol](https://www.battleye.com/downloads/BERConProtocol.txt).

### address

required

IP address to which the RCON socket will be bound. It can be used to restrict connection to a particular network interface.

### port

number value, range 1..65535, default 19999

RCON protocol port on which the game listens

### password

string

**Important:** The password:
- is required for RCON to start
- does **not** support spaces
- must be at least **3** characters long

### maxClients

Arma Reforger 1.1.0

number value, range 1..16, default 16

The maximum number of clients that can connect to RCON at the same time.

**Note:** RCON lacks client disconnect support, so when a user exits the RCON client, the server maintains the connection for an additional 30 seconds before a timeout removes the user from the list.

### permission

string

Permission for all RCON clients.
Possible values:

| Key | Value |
| --- | --- |
| admin | The admin can perform any command. |
| monitor | The monitor can only perform commands which do not change the server's state. |

### blacklist

array value, default []

A list of commands excluded from execution.

### whitelist

array value, default []

If defined, it specifies the list of commands that can be executed, and no other command is allowed.

## game

### name

length 0..100 characters

### password

length 0..x characters

Password required to join the server.

### passwordAdmin

length: 0..x characters

Defines the server's admin password, allows a server administrator to login and control the server, to access this either open the chat input box by pressing [C] in the lobby or [enter] in-game followed by: `#login [the admin password]`

**Important:** This password does not support spaces in it.

*was root/passwordAdmin before 0.9.8.73*

### admins

Arma Reforger 0.9.9

*0.9.9.31*

array value of **IdentityIds** and/or steamIds

List players as server admins - they can be checked in script using `BackendApi.IsListedServerAdmin(int playerId)` regardless of their logged-in state.

Arma Reforger 1.0.0 Listed users can `#login` without a password.

Arma Reforger 1.3.0
  - list is now limited to only 20 unique IDs
  - listed admins can use priority queue when joining server
    - they will have their own queue which have priority before normal players when joining the server
    - priority queue works only with admins which are specified by **IdentityIds**
    - **admins specified by their SteamId are not currently supported for priority queue**

### scenarioId

The scenario's `.conf` file path is defined here.
See the [listScenarios](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#listScenarios) startup parameter to list available scenarios and obtain their `.conf` file path.

Arma Reforger 1.6.0
```
--------------------------------------------------
Official scenarios (31 entries)
--------------------------------------------------
{ECC61978EDCC2B5A}Missions/23_Campaign.conf (Conflict - Everon)
{002AF7323E0129AF}Missions/Tutorial.conf (Training)
{59AD59368755F41A}Missions/21_GM_Eden.conf (Game Master - Everon)
{2BBBE828037C6F4B}Missions/22_GM_Arland.conf (Game Master - Arland)
{F45C6C15D31252E6}Missions/27_GM_Cain.conf (Game Master - Kolguyev)
{C700DB41F0C546E1}Missions/23_Campaign_NorthCentral.conf (Conflict - Northern Everon)
{28802845ADA64D52}Missions/23_Campaign_SWCoast.conf (Conflict - Southern Everon)
{94992A3D7CE4FF8A}Missions/23_Campaign_Western.conf (Conflict - Western Everon)
{FDE33AFE2ED7875B}Missions/23_Campaign_Montignac.conf (Conflict - Montignac)
{DAA03C6E6099D50F}Missions/24_CombatOps.conf (Combat Ops - Arland)
{C41618FD18E9D714}Missions/23_Campaign_Arland.conf (Conflict - Arland)
{DFAC5FABD11F2390}Missions/26_CombatOpsEveron.conf (Combat Ops - Everon)
{3F2E005F43DBD2F8}Missions/CAH_Briars_Coast.conf (Capture & Hold - Briars)
{F1A1BEA67132113E}Missions/CAH_Castle.conf (Capture & Hold - Montfort Castle)
{589945FB9FA7B97D}Missions/CAH_Concrete_Plant.conf (Capture & Hold - Concrete Plant)
{9405201CBD22A30C}Missions/CAH_Factory.conf (Capture & Hold - Almara Factory)
{1CD06B409C6FAE56}Missions/CAH_Forest.conf (Capture & Hold - Simon's Wood)
{7C491B1FCC0FF0E1}Missions/CAH_LeMoule.conf (Capture & Hold - Le Moule)
{6EA2E454519E5869}Missions/CAH_Military_Base.conf (Capture & Hold - Camp Blake)
{2B4183DF23E88249}Missions/CAH_Morton.conf (Capture & Hold - Morton)
{C47A1A6245A13B26}Missions/SP01_ReginaV2.conf (Elimination)
{0648CDB32D6B02B3}Missions/SP02_AirSupport.conf (Air Support)
{0220741028718E7F}Missions/23_Campaign_HQC_Everon.conf (Conflict: HQ Commander - Everon)
{68D1240A11492545}Missions/23_Campaign_HQC_Arland.conf (Conflict: HQ Commander - Arland)
{BB5345C22DD2B655}Missions/23_Campaign_HQC_Cain.conf (Conflict: HQ Commander - Kolguyev)
{10B8582BAD9F7040}Missions/Scenario01_Intro.conf (Operation Omega 01: Over The Hills And Far Away)
{1D76AF6DC4DF0577}Missions/Scenario02_Steal.conf (Operation Omega 02: Radio Check)
{D1647575BCEA5A05}Missions/Scenario03_Villa.conf (Operation Omega 03: Light In The Dark)
{6D224A109B973DD8}Missions/Scenario04_Sabotage.conf (Operation Omega 04: Red Silence)
{FA2AB0181129CB16}Missions/Scenario05_Hill.conf (Operation Omega 05: Cliffhanger)
{CB347F2F10065C9C}Missions/CombatOpsCain.conf (Combat Ops - Kolguyev)
```

Arma Reforger 1.3.0
```
--------------------------------------------------
Official scenarios (20 entries)
--------------------------------------------------
{ECC61978EDCC2B5A}Missions/23_Campaign.conf (Conflict - Everon)
{59AD59368755F41A}Missions/21_GM_Eden.conf (Game Master - Everon)
{002AF7323E0129AF}Missions/Tutorial.conf (Training)
{2BBBE828037C6F4B}Missions/22_GM_Arland.conf (Game Master - Arland)
{C700DB41F0C546E1}Missions/23_Campaign_NorthCentral.conf (Conflict - Northern Everon)
{28802845ADA64D52}Missions/23_Campaign_SWCoast.conf (Conflict - Southern Everon)
{94992A3D7CE4FF8A}Missions/23_Campaign_Western.conf (Conflict - Western Everon)
{FDE33AFE2ED7875B}Missions/23_Campaign_Montignac.conf (Conflict - Montignac)
{DAA03C6E6099D50F}Missions/24_CombatOps.conf (Combat Ops - Arland)
{C41618FD18E9D714}Missions/23_Campaign_Arland.conf (Conflict - Arland)
{DFAC5FABD11F2390}Missions/26_CombatOpsEveron.conf (Combat Ops - Everon)
{3F2E005F43DBD2F8}Missions/CAH_Briars_Coast.conf (Capture & Hold - Briars)
{F1A1BEA67132113E}Missions/CAH_Castle.conf (Capture & Hold - Montfort Castle)
{589945FB9FA7B97D}Missions/CAH_Concrete_Plant.conf (Capture & Hold - Concrete Plant)
{9405201CBD22A30C}Missions/CAH_Factory.conf (Capture & Hold - Almara Factory)
{1CD06B409C6FAE56}Missions/CAH_Forest.conf (Capture & Hold - Simon's Wood)
{7C491B1FCC0FF0E1}Missions/CAH_LeMoule.conf (Capture & Hold - Le Moule)
{6EA2E454519E5869}Missions/CAH_Military_Base.conf (Capture & Hold - Camp Blake)
{2B4183DF23E88249}Missions/CAH_Morton.conf (Capture & Hold - Morton)
{C47A1A6245A13B26}Missions/SP01_ReginaV2.conf (Elimination)
```

Arma Reforger 1.2.1
```
: --------------------------------------------------
: Official scenarios (17 entries)
: --------------------------------------------------
: {ECC61978EDCC2B5A}Missions/23_Campaign.conf (Conflict - Everon)
: {59AD59368755F41A}Missions/21_GM_Eden.conf (Game Master - Everon)
: {002AF7323E0129AF}Missions/Tutorial.conf (Training)
: {2BBBE828037C6F4B}Missions/22_GM_Arland.conf (Game Master - Arland)
: {C700DB41F0C546E1}Missions/23_Campaign_NorthCentral.conf (Conflict - Northern Everon)
: {28802845ADA64D52}Missions/23_Campaign_SWCoast.conf (Conflict - Southern Everon)
: {DAA03C6E6099D50F}Missions/24_CombatOps.conf (Combat Ops - Arland)
: {C41618FD18E9D714}Missions/23_Campaign_Arland.conf (Conflict - Arland)
: {DFAC5FABD11F2390}Missions/26_CombatOpsEveron.conf (Combat Ops - Everon)
: {3F2E005F43DBD2F8}Missions/CAH_Briars_Coast.conf (Capture & Hold: Briars )
: {F1A1BEA67132113E}Missions/CAH_Castle.conf (Capture & Hold: Montfort Castle)
: {589945FB9FA7B97D}Missions/CAH_Concrete_Plant.conf (Capture & Hold: Concrete Plant)
: {9405201CBD22A30C}Missions/CAH_Factory.conf (Capture & Hold: Almara Factory)
: {1CD06B409C6FAE56}Missions/CAH_Forest.conf (Capture & Hold: Simon's Wood)
: {7C491B1FCC0FF0E1}Missions/CAH_LeMoule.conf (Capture & Hold: Le Moule)
: {6EA2E454519E5869}Missions/CAH_Military_Base.conf (Capture & Hold: Camp Blake)
: {2B4183DF23E88249}Missions/CAH_Morton.conf (Capture & Hold: Morton)
```

Arma Reforger 1.1.0
```
: --------------------------------------------------
: Official scenarios (9 entries)
: --------------------------------------------------
: {ECC61978EDCC2B5A}Missions/23_Campaign.conf (Conflict - Everon)
: {59AD59368755F41A}Missions/21_GM_Eden.conf (Game Master - Everon)
: {94FDA7451242150B}Missions/103_Arland_Tutorial.conf (Training)
: {2BBBE828037C6F4B}Missions/22_GM_Arland.conf (Game Master - Arland)
: {C700DB41F0C546E1}Missions/23_Campaign_NorthCentral.conf (Conflict - Northern Everon)
: {28802845ADA64D52}Missions/23_Campaign_SWCoast.conf (Conflict - Southern Everon)
: {DAA03C6E6099D50F}Missions/24_CombatOps.conf (Combat Ops - Arland)
: {C41618FD18E9D714}Missions/23_Campaign_Arland.conf (Conflict - Arland)
: {DFAC5FABD11F2390}Missions/26_CombatOpsEveron.conf (Combat Ops - Everon)
```

```
: --------------------------------------------------
: Workshop scenarios (8 entries)
: --------------------------------------------------
: {6EA2E454519E5869}Missions/CAH_Military_Base.conf
: {7C491B1FCC0FF0E1}Missions/CAH_LeMoule.conf
: {F1A1BEA67132113E}Missions/CAH_Castle.conf
: {589945FB9FA7B97D}Missions/CAH_Concrete_Plant.conf
: {2B4183DF23E88249}Missions/CAH_Morton.conf
: {3F2E005F43DBD2F8}Missions/CAH_Briars_Coast.conf
: {9405201CBD22A30C}Missions/CAH_Factory.conf
: {1CD06B409C6FAE56}Missions/CAH_Forest.conf
```

Arma Reforger 1.0.0
```
: {ECC61978EDCC2B5A}Missions/23_Campaign.conf (Conflict - Everon)
: {59AD59368755F41A}Missions/21_GM_Eden.conf (Game Master - Everon)
: {94FDA7451242150B}Missions/103_Arland_Tutorial.conf (Training)
: {2BBBE828037C6F4B}Missions/22_GM_Arland.conf (Game Master - Arland)
: {C700DB41F0C546E1}Missions/23_Campaign_NorthCentral.conf (Conflict - Northern Everon)
: {28802845ADA64D52}Missions/23_Campaign_SWCoast.conf (Conflict - Southern Everon)
: {DAA03C6E6099D50F}Missions/24_CombatOps.conf (Combat Ops - Arland)
: {C41618FD18E9D714}Missions/23_Campaign_Arland.conf (Conflict - Arland)
```

Arma Reforger 0.9.9
```
: {ECC61978EDCC2B5A}Missions/23_Campaign.conf (Conflict - Everon)
: {59AD59368755F41A}Missions/21_GM_Eden.conf (Game Master - Everon)
: {90F086877C27B6F6}Missions/99_Tutorial.conf (Tutorial)
: {2BBBE828037C6F4B}Missions/22_GM_Arland.conf (Game Master - Arland)
: {C700DB41F0C546E1}Missions/23_Campaign_NorthCentral.conf (Conflict - Northern Everon)
: {28802845ADA64D52}Missions/23_Campaign_SWCoast.conf (Conflict - Southern Everon)
: {DAA03C6E6099D50F}Missions/24_CombatOps.conf (Combat Ops - Arland)
: {C41618FD18E9D714}Missions/23_Campaign_Arland.conf (Conflict - Arland)
```

### maxPlayers

number value, range 1..128, default: 64

Set the maximum amount of players on the server.

*was **playerCountLimit** before 0.9.8.73*

### visible

bool value, default: true (since 0.9.8.73)

Set the visibility of the server in the Server Browser.

### crossPlatform

Arma Reforger 0.9.9

bool value, default: false

Accept all platforms if set to true, use supportedPlatforms if set to false.

**Note:** It is recommended to use **crossPlatform** only, leaving supportedPlatforms undefined.

### supportedPlatforms

array value, default: ["PLATFORM_PC"]

Define the platforms which the server accepts, allowing crossplay.
Possible values:

| Key | Value |
| --- | --- |
| PLATFORM_PC | PC |
| PLATFORM_XBL | Xbox console |
| PLATFORM_PSN | PlayStation console (since Arma Reforger 1.2.1.169) |

**Important:** It is recommended to use **crossPlatform** instead and leave this property undefined:

| `crossPlatform = true;` | is equivalent to | `supportedPlatforms = ["PLATFORM_PC", "PLATFORM_XBL", "PLATFORM_PSN"];` |
| --- | --- | --- |
| `crossPlatform = false;` | is equivalent to | `supportedPlatforms = ["PLATFORM_PC"];` |

These two being the only valid combinations.

*was **supportedGameClientTypes** before 0.9.8.73*

### gameProperties

Define the scenario's settings - see the gameProperties section below.

### modsRequiredByDefault

Arma Reforger 1.2.1

bool value, default: true

Overrides default value for required for all mods.

### mods

The list of mods required by the client that will automatically be downloaded and activated on join.
It can be created by activating wanted mods in the [Workshop](https://community.bistudio.com/wiki/Arma_Reforger:Workshop) and then, via the **Mod manager**, result can be converted to ready to be used string after navigating to **JSON** tab and then clicking on the **Copy to clipboard** button.

#### modID

GUID of the mod. Single GUID can be obtained from Workshop webpage

Alternatively you can grab it from Workbench options when mod is running or directly from gproj file

#### name

This parameter does not do anything and is only used as sort of comment, with human readable name of the mod.

#### version

**Note:** The `version` mod parameter is optional. If it missing, the latest mod version will be used.

#### required

Arma Reforger 1.2.1

bool value, default: true or value of modsRequiredByDefault

Is optional parameter to specify if this addon is required for server to start.
If set to false then addon will be automatically removed from list with warning in logs if it cannot be for some reason downloaded from the Workshop.

## gameProperties

### serverMaxViewDistance

number value, range 500..10000, default: 1600

### serverMinGrassDistance

number value, range 0 / 50..150, default: 0

Minimum grass distance in meters. If set to 0 no distance is forced upon clients.

### fastValidation

bool value, default: true (since 0.9.6)

Validation of map entities and components loaded on client when it joins, ensuring things match with initial server state.
- true (enabled) - minimum information required to make sure data matches is exchanged between client. When a mismatch occurs, no additional information will be available for determining where client and server states start to differ. All servers that expect clients to connect over internet should have fast validation enabled.
- false (disabled) - extra data for every replicated entity and component in the map will be transferred when new client connects to the server. When a mismatch occurs, it is possible to point at particular entity or component where things start to differ. When developing locally (ie. both server and client run on the same machine), it is fine to disable fast validation to more easily pin point source of the problem.

**Warning:** **Always** set this value to `true` for a public server!

### networkViewDistance

number value, range 500..5000, default: 1500

Maximum network streaming range of replicated entities.

### battlEye

bool value, default: true (since 0.9.8.73)

true to enable BattlEye, false to disable it.

### disableThirdPerson

bool value, default: false

Force clients to use the first-person view.

### VONDisableUI

bool value, default: false

Force clients to not have VON (Voice Over Network) UI.

### VONDisableDirectSpeechUI

bool value, default: false

Force clients to not have VON (Voice Over Network) Direct Speech UI.

### VONCanTransmitCrossFaction

Arma Reforger 1.0.0

bool value, default: false

Option to allow players to transmit on other factions radios. true is allow to communicate, false is listen-only

### missionHeader

This property overwrites the scenario's header, allowing to e.g change the scenario's displayed name:

```json
"missionHeader": {
	"m_sName": "My Very Own Hosted Conflict",
	"m_sDetails": "If you teamkill or commit war crimes, you WILL be banned!",
	"m_iStartingHours": 7,
	"m_iStartingMinutes": 30,
	"m_bRandomStartingWeather": true
}
```

Generic scenario properties (from `SCR_MissionHeader`):

| `"missionHeader": {`<br>`"m_sName": "My Very Own Hosted Conflict",`<br>`"m_sAuthor": "Bohemia and I",`<br>`"m_sDescription": "Play Conflict here!",`<br>`"m_sDetails": "Here are the rules: have fun!",`<br>`"m_iPlayerCount": 64,`<br>`"m_bIsSavingEnabled": true,`<br>`"m_sSaveFileName": "MyConflict_Save",`<br>`"m_sBriefingConfig": "{0123456789ABCDEF}Configs/MyBriefingConfig.conf",`<br>`"m_bOverrideScenarioTimeAndWeather": true,`<br>`"m_iStartingHours": 7,`<br>`"m_iStartingMinutes": 30,`<br>`"m_bRandomStartingDaytime": false,`<br>`"m_fDayTimeAcceleration": 6,`<br>`"m_fNightTimeAcceleration": 12,`<br>`"m_bRandomStartingWeather": true,`<br>`"m_bRandomWeatherChanges": false,`<br>`"m_fXpMultiplier": 10,`<br>`"m_bMapMarkerEnableDeleteByAnyone": false,`<br>`"m_iMapMarkerLimitPerPlayer": 99`<br>`}` | - **m_sName**: scenario name<br>- **m_sAuthor**: scenario author<br>- **m_sDescription**: brief description of this scenario's purpose<br>- **m_sDetails**: detailed description of this scenario (i.e. rules)<br>- **m_iPlayerCount**: The count of players for this scenario<br>- **m_bIsSavingEnabled**: enable saving scenario state<br>- **m_sSaveFileName**: name of save file for this scenario; when undefined, the name of associated world file will be used<br>- **m_sBriefingConfig**: configuration file for briefing screen<br>- **m_bOverrideScenarioTimeAndWeather**: if the scenario allows it, its daytime and weather will use values from this header<br>- **m_iStartingHours**: starting time of day (hours)<br>- **m_iStartingMinutes**: starting time of day (minutes)<br>- **m_bRandomStartingDaytime**: randomise the start time (overrides above parameters)<br>- **m_fDayTimeAcceleration**: time acceleration during the day (1 = 100%, 2 = 200% etc)<br>- **m_fNightTimeAcceleration**: time acceleration during the night (1 = 100%, 2 = 200% etc)<br>- **m_bRandomStartingWeather**: the weather is randomised on start<br>- **m_bRandomWeatherChanges**: weather can change during gameplay<br>- **m_fXpMultiplier**: player XP multiplier (when enabled in gamemode; 1 for default)<br>- **m_bMapMarkerEnableDeleteByAnyone**: determines whether map markers can be deleted only by player who placed them or by anyone within faction<br>- **m_iMapMarkerLimitPerPlayer**: how many map markers per player can exist at a time |
| --- | --- |

Conflict-specific properties (from `SCR_MissionHeaderCampaign`):

| `"missionHeader": {`<br>`"m_iControlPointsCap": 42,`<br>`"m_fVictoryTimeout": -1,`<br>`"m_iStartingHQSupplies": 9001,`<br>`"m_iMinimumBaseSupplies": 500,`<br>`"m_iMaximumBaseSupplies": 10000,`<br>`"m_bCustomBaseWhitelist": true,`<br>`"m_bIgnoreMinimumVehicleRank": true,`<br>`"m_fSupplyOffloadAssistanceReward": 0.25,`<br>`"m_aCampaignCustomBaseList": {`<br>`{`<br>`"m_sBaseName": "MajorBaseMorton",`<br>`"m_bIsControlPoint": false,`<br>`"m_bCanBeHQ": true,`<br>`"m_bDisableWhenUnusedAsHQ": true,`<br>`"m_fRadioRange": 500`<br>`}`<br>`}`<br>`}` | - **m_iControlPointsCap**: how many control points are required to win (-1 = default)<br>- **m_fVictoryTimeout**: how long a faction needs to hold a control point, in seconds (-1 = default)<br>- **m_iStartingHQSupplies**: how much supplies should the main HQ start with (-1 = default)<br>- **m_iMinimumBaseSupplies**: minimum starting amount of supplies in small bases (-1 = default)<br>- **m_iMaximumBaseSupplies**: maximum starting amount of supplies in small bases (-1 = default)<br>- **m_bCustomBaseWhitelist**: use custom base list whitelist instead of blacklist<br>- **m_bIgnoreMinimumVehicleRank**: disable rank requirements for vehicle spawning<br>- **m_fSupplyOffloadAssistanceReward**: fraction of XP awarded to players unloading supplies which they have not loaded themselves (-1 = default)<br>- **m_aCampaignCustomBaseList**: array of custom bases (from `SCR_CampaignCustomBase`):<br>- **m_sBaseName**: base entity name as set up in World Editor<br>- **m_bIsControlPoint**: whether or not the base is a control point<br>- **m_bCanBeHQ**: whether or not the base can be a headquarter<br>- **m_bDisableWhenUnusedAsHQ**: whether or not the base is disabled if not used as HQ<br>- **m_fRadioRange**: base's radio range in metres (-1 = default) |
| --- | --- |

### persistence

Arma Reforger 1.6.0

Configuration values for the [Arma Reforger:Persistence System](https://community.bistudio.com/wiki/Arma_Reforger:Persistence_System) that can be optionally defined. The system is setup to automatically work for most use cases by default.

```json
"persistence": {
	"autoSaveInterval": 15,
	"hiveId": 1337,
	"databases": {
		"webapi": {
			"preset": "{0123456789ABCDEF}Configs/Systems/Persistence/Database/JsonWebApi.conf",
			"options": {
				"url": "https://persistence-db.armareforger.local:5539/api/v1?query=parameter&and=more",
				"headers": {
					"X-API-KEY": "4rm4R3f0rg3r",
					"Content-Type": "application/json"
				}
			}
		}
	},
	"storages": {
		"session": {
			"database": "webapi"
		}
	}
}
```

#### autoSaveInterval

number value, range 0..60, default: 10

Interval (in minutes) at which automatic saves are created, if possible for the current mission. Disabled if set to 0.

#### hiveId

number value, range 0..16383, default: 0

Number identifying each hive (server) when they share the same persistence database. Used for separating UUIDs for save game data.

#### databases

(Case-insensitive) named objects representing an override of the system configuration provided databases or the addition of a new one.
Optional child properties:
- "preset" - refers to database config presets that are defined in game/mod data. Full resource name.
- "options" - a complex object which corresponds to the individual database type of the preset. Usually, a key-value collection

#### storages

(Case-insensitive) named objects representing an override of the system configuration-provided storages.
Optional child properties:
- "database" - changes the storage to use a different database than the default configured. Must match the name given in the databases object or "main".

## operating

Arma Reforger 0.9.7

### lobbyPlayerSynchronise

bool value, default: true (since 0.9.8.73)

If enabled, the list of player identities present on server is sent to the GameAPI along with the server's heartbeat.

**Note:** This setting fixes the discrepancy between the real and reported number of players on the server.

### disableCrashReporter

Arma Reforger 0.9.8

bool value, default: false

If enabled, the automatic server-side [Crash Report](https://community.bistudio.com/wiki/Arma_Reforger:Crash_Report) is disabled. Has the same effect as [-disableCrashReporter](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#disableCrashReporter).

### disableNavmeshStreaming

Arma Reforger 1.0.0

Arma Reforger 1.0.0 bool value, default: false<br>
Arma Reforger 1.2.0 array value, default undefined

A disabled navmesh streaming loads the entire navmesh in memory - this provides slightly better server performance and reaction times of moving AIs at the cost of a bigger memory footprint (up to hundreds of MB depending on the terrain) - see [2024-07-07 Modding Update](https://reforger.armaplatform.com/news/modding-update-june-7-2024)
- Arma Reforger 1.0.0 If set to true, navmesh streaming is disabled on all navmesh components
- Arma Reforger 1.2.0 If not provided, no navmesh streaming is disabled; if provided empty, streaming of all navmeshes is disabled; if filled, only disables provided navmeshes's streaming

*was **bool** before 1.2.0*

### disableServerShutdown

Arma Reforger 0.9.8

bool value, default: false

If enabled, the server will not automatically shutdown if connection to backend is lost.
Related to room requests errors - other causes like corrupted config will still shutdown the server.

### disableAI

Arma Reforger 1.1.0

bool value, default: false

If enabled, the server will prevent initialization and ticking of AIWorld and its components.
Will completely disable AI functionality on the server.

### playerSaveTime

Arma Reforger 0.9.8

number value, default: 120

Default period in seconds for saving players for both Online and Local storage (player save can still be requested on demand).

### aiLimit

Arma Reforger 0.9.8

*0.9.8.64*

number value, default: -1

Sets the top limit of AIs. No system will be able to spawn any AIs when this ceiling is reached (through `aiWorld.CanAICharacterBeAdded()`)

A negative number is not considered as valid value and is then ignored - limit is not applied.

This param is overridden by the [-aiLimit](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#AILimit) startup parameter.

### slotReservationTimeout

Arma Reforger 0.9.9

*0.9.9.31*

number value, range 5..300, default 60

Sets the duration (in seconds) for how long will the backend and server reserve a slot for kicked player.
It is considered disabled when set to the minimal value, the value being the same as for a normal disconnect.

It can be used in scripts *via* Game Mode events:

```enforce
OnPlayerAuditTimeouted(int iPlayerID)	// invoked when player did not reconnect in time
OnPlayerAuditRevived(int iPlayerID)		// invoked when player successfully reconnected in time
```

**Note:** Reservation works only for **replication** kicks.

### joinQueue

Arma Reforger 1.2.1

*1.2.1.66*

Configuration for player join queue to the server

#### maxSize

Arma Reforger 1.2.1

number value, range 0..50, default 0 (disabled)

Sets maximum size of how many people can be at one time in join queue to the server.

## Template

```json
{
	"bindAddress": "",
	"bindPort": 0,
	"publicAddress": "",
	"publicPort": 0,
	"a2s": {
		"address": "",
		"port": 0
	},
	"rcon": {
		"address": "",
		"port": 0,
		"password": "",
		"permission": "monitor",
		"blacklist": [],
		"whitelist": []
	},
	"game": {
		"name": "",
		"password": "",
		"passwordAdmin": "",
		"admins" : [],
		"scenarioId": "",
		"maxPlayers": 0,
		"visible": true,
		"gameProperties": {
			"serverMaxViewDistance": 1600,
			"serverMinGrassDistance": 0,
			"networkViewDistance": 1500,
			"disableThirdPerson": false,
			"fastValidation": true,
			"battlEye": true,
			"VONDisableUI": false,
			"VONDisableDirectSpeechUI": false,
			"VONCanTransmitCrossFaction": false
		},
		"mods": []
	}
}
```

## Example

```json
{
	"bindAddress": "0.0.0.0",
	"bindPort": 2001,
	"publicAddress": "192.168.9.10",
	"publicPort": 2001,
	"a2s": {
		"address": "192.168.9.10",
		"port": 17777
	},
	"rcon": {
		"address": "192.168.9.10",
		"port": 19999,
		"password": "changeme_withoutspaces",
		"permission": "monitor",
		"blacklist": [],
		"whitelist": []
	},
	"game": {
		"name": "Server Name - Mission Name",
		"password": "",
		"passwordAdmin": "changeme",
		"admins" : [
			"76561198200329058"
		],
		"scenarioId": "{ECC61978EDCC2B5A}Missions/23_Campaign.conf",
		"maxPlayers": 32,
		"visible": true,
		"crossPlatform": true,
		"supportedPlatforms": [
			"PLATFORM_PC",
			"PLATFORM_XBL"
		],
		"gameProperties": {
			"serverMaxViewDistance": 2500,
			"serverMinGrassDistance": 50,
			"networkViewDistance": 1000,
			"disableThirdPerson": true,
			"fastValidation": true,
			"battlEye": true,
			"VONDisableUI": true,
			"VONDisableDirectSpeechUI": true,
			"missionHeader": {
				"m_iPlayerCount": 40,
				"m_eEditableGameFlags": 6,
				"m_eDefaultGameFlags": 6,
				"other": "values"
			}
		},
		"mods": [
			{
				"modId": "59727DAE364DEADB",
				"name": "WeaponSwitching",
				"version": "1.0.1"
			},
			{
				"modId": "59727DAE32981C7D",
				"name": "Explosive Goats beta",
				"version": "0.5.42"
			}
		]
	},
	"operating": {
		"lobbyPlayerSynchronise": true,
		"joinQueue" : {
			"maxSize" : 12
		},
		"disableNavmeshStreaming": [
			"Soldiers",
			"BTRlike"
		]
	}
}
```
