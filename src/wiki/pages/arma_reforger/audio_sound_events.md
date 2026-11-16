---
title: Arma Reforger:Audio: Sound Events
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Sound_Events
---

Sound Events are strings serving as identifiers for the sound system.
They are defined by naming a [Sound Node](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Nodes#Sound) and used by calling a Sound Event with the same name, e.g. via code or script.

## Script Events

**Note:** All script events are stored in `SCR_SoundEvent`.

## GameCode Events

### BaseMuzzleComponent

| Event Name | Description |
| --- | --- |
| `SOUND_DRY` | When firing with no bullets in magazine. |
| `SOUND_DRY_SAFETYON` | When firing with safety on. |
| `SOUND_RELOAD` | When reloading ACK.<br>Not used right now. Animations are used instead. |

### CaseEjectingEffectComponent

| Event Name | Description |
| --- | --- |
| `SOUND_BULLET_CASING` | Upon collision with the ground. |

### CharacterSoundComponent

| Event Name | Description |
| --- | --- |
| `SOUND_BODYFALL` | Upon character falling (Ragdoll). |
| `SOUND_BODYFALL_TERMINAL` | Upon character falling, resulting in death. |
| `SOUND_CHAR_RAGDOLL` | When impacting in Ragdoll state. |
| `SOUND_SWIM_START` | When character starts swimming. |
| `SOUND_SWIM_STOP` | When character stops swimming. |
| `SOUND_WATER_ENTER` | When entering water surface when EntryLevel over threshold *(0.1f).* |
| `SOUND_WATER_ENTER_RAGDOLL` | When entering water in ragdoll state. |
| `SOUND_WATER_EXIT` | When exiting water surface. |

### CommunicationSoundComponent

| Event Name | Priority | Description |
| --- | --- | --- |
| `SOUND_HIT` | 99 | - Played when character gets damage<br>- Player<br>- Grunt<br>- AI<br>- Projectile damage: Would play sound that is connected to SOUND_REPORTS_STATUS_HIT<br>- Other damage: Grunt<br>**Signals:**<br>- IsAI - Player = 0, AI = 1, Set on SignalsManagerComponent<br>- DamageType |
| `SOUND_DEATH` | n/a | When character dies |
| `SOUND_BREATH_IN` | -20 | Breathing in of character. |
| `SOUND_BREATH_OUT` | -20 | Breathing out of character. |
| `SOUND_BREATH_REG_IN` | -20 | Breathing in (regular) of character. |
| `SOUND_BREATH_REG_OUT` | -20 | Breathing out (regular) of character. |
| `SOUND_BREATH_END` | -20 | Upon stamina restored and breathing stops. |

### DoorComponent / SoundPointEventInfo

| Event Name | Description |
| --- | --- |
| `SOUND_OPEN_START` | When opening the door from fully closed state. |
| `SOUND_OPEN_FINISH` | When door is fully opened from closed state. |
| `SOUND_CLOSE_START` | When closing the door from fully opened state. |
| `SOUND_CLOSE_FINISH` | When door is fully closed from open state. |
| `SOUND_MOVEMENT` | When the door is opening/closing. |

### ExplosionEffect

| Event Name | Description |
| --- | --- |
| *Custom Event* | Raised when explosion triggered. |

### GrenadeMoveComponent

| Event Name | Description |
| --- | --- |
| `SOUND_HIT` | Upon contact & deflection of grenade. |

### HitSoundEffect

| Event Name | Description |
| --- | --- |
| `SOUND_HIT` | Upon bullet hitting surface / player. |

### MusicManagerController

| Event Name | Description |
| --- | --- |
| `SOUND_MILITARYBASE` | Theme played at military bases. |
| `SOUND_FOREST` | Theme played in forests. |
| `SOUND_VILLAGE` | Theme played in villages. |
| `SOUND_COASTLINE` | Theme played at coastlines. |

### ProjectileSoundsModule

| Event Name | Description |
| --- | --- |
| `SOUND_SONIC_CRACK` | Upon sonic crack when speed > SpeedOfSound. |
| `SOUND_SONIC_CRACK_SECONDARY` | Upon projectile transitioning from supersonic to subsonic. |
| `SOUND_SONIC_CRACK_IMPACT` | Upon projectile impacting before it reaches the listener. |
| `SOUND_FLYBY` | Upon sonic crack when speed  <= SpeedOfSound. |

### RadioBroadcastSoundComponent

| Event Name | Description |
| --- | --- |
| `SOUND_RADIO_TURN_ON` | When turning on the radio. |
| `SOUND_RADIO_TURN_OFF` | When turning off the radio. |
| `SOUND_RADIO` | After turning the radio on. |

### Vehicle

| Event Name | Description |
| --- | --- |
| `SOUND_COLLISION` | - In a collision between 2 vehicles, sound is played from vehicle with higher mass.<br>- use eventNoRepeatTime to filter out duplicate collisions. |

### VoNComponent

| Event Name | Description |
| --- | --- |
| `VON_DIRECT` | VON for direct speech |
| `VON_RADIO` | VON for radio transmissions |
| `VON_RAW` |  |

### WeaponSoundComponent

| Event Name | Description |
| --- | --- |
| `SOUND_MELEE_IMPACT` | Upon dealing melee damage. |
| `SOUND_SHOT` | Upon firing a weapon. |
| `SOUND_SHOT_END` | After firing stops. |
| `SOUND_THROWN` | After a throwable item (e.g. grenades) has been thrown. |
| `SOUND_WPN_TOSAFETY` | When changing the fire mode to safety. |
| `SOUND_WPN_TOAUTO` | When changing the fire mode to auto. |
| `SOUND_WPN_TOBURST` | When changing the fire mode to burst. |
| `SOUND_WPN_TOSEMIAUTO` | When changing the fire mode to semi-auto. |
| `SOUND_DRY_SAFETYON` | When firing with safety on. |
| `SOUND_WPN_TOUGL` | When changing muzzle to UGL. |
| `SOUND_ZEROING_DOWN` | Upon zeroing down. |
| `SOUND_ZEROING_UP` | Upon zeroing up. |
