---
title: Arma Reforger:Audio: Sound Components
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Sound_Components
---

At the most basic level, Sound Components are Components that give an Entity the ability to play sounds defined in ACP files.
The ACP files available to the Entity are defined in the Filenames section of its SoundComponent.

Via GameCode or a Script, the `SoundComponent` can receive the instruction to play a Sound. This instruction will contain a string with the desired Sound Event Name.
The SoundComponent will then search through its connected ACP files and play the first Sound whose name matches the Sound Event Name provided by the instruction.

There are different SoundComponents: Some are generic, like the `BaseSoundComponent` and `SimpleSoundComponent`,
and some are meant to be used for a specific purpose, e.g the `WeaponSoundComponent` or `VehicleSoundComponent`.

The main reason for having multiple different SoundComponents is performance.
More "advanced" SoundComponents perform a variety of checks and calculations that might not be needed in every use case, such as interior detection or listening in on animations for sound events.

It is therefore recommended to familiarise oneself with the different SoundComponent types and use that one that matches the required featureset the closest.

The most common feature of "advanced" SoundComponent variations is it to read Signal values from an Entity's SignalsManagerComponent as well as the Gameworld's `GameSignalsManager`.

## SoundComponent Variants

| SoundComponent Type | Dynamic Simulation | Trigger Update | Can read Signals from Signals Manager | Sound position's dynamic update | Listen to animation events | Dedicated features |
| --- | --- | --- | --- | --- | --- | --- |
| `BaseSoundComponent` | No | Yes<br>but must be called manually via UpdateTrigger | No | No | No | n/a |
| `SimpleSoundComponent` | Yes | Yes<br>but must be called manually via UpdateTrigger | No | No<br>but UpdateSoundJob is called when any of assigned `.acp`s is in the audibility range. | No | n/a |
| `SoundComponent` | Yes | Yes | Yes | Yes | Yes | - Reads local signals from the SignalsManagerComponent.<br>- Reads Global Signals from the GameSignalsManager.<br>- Listens to Animation Events (BaseItemAnimationComponent or BaseAnimPhysComponent).<br>- EOnFrame flag enables calling UpdateSoundJob() from entity EOnFrame.<br>- Manages sound transformation offset using bone, vector offset, and sound points.<br>- Manages Component activation based of the presence of playing sound.<br>- Updates sound position to entity position.<br>- Updates triggers for sound nodes. |
| `WeaponSoundComponent` | Yes | Yes | Yes | Yes | Yes | - Manages Signals for the SOUND_SHOT Event when using speed of sound simulation. On a shot event, a signal snapshot with current signal values is created and this snapshot is used once shot sound is triggered. |
| `CharacterSoundComponent` | Yes | Yes | Yes | Yes | Yes | - Updates Surface signals for footstep Eound Events. An Animation Event is considered as Footstep Event if its bone name contains the "foot" string. |
| `VehicleSoundComponent` | Yes | Yes | Yes | Yes | Yes | - Dedicated slots for wheel sound. Sound position offset is defined directly on the component. The Surface signal for each wheel is tied to this position. |
| `CommunicationSoundComponent` | Yes | Yes | Yes | Yes | Yes | - See CommunicationSoundComponent |

## Components

### CommunicationSoundComponent

For purposes of the communication protocol, the dedicated Communication Sound Component was created. Simplified, this component have two main purposes.
Based on input signals, it is able to construct a sentence that is represented as
- a list of `.wav` files that the audio system can play in sequence
- a string array that is used for displaying subtitles.

The sentence definition is done in one place in audio project, so it should keep the whole configuration and maintenance of the system manageable.

#### Animation Sound Priority Ranges

| Range | Description |
| --- | --- |
| **< 0** | Discarded - Negative values are not allowed and used by the priority queue. |
| **0..99** | Queued if current playing sound has a higher priority |
| **100..199** | Discarded if current playing sound has a higher priority<br>100 = IgnoreQueue, 1XX = Priority (Example: 199 = Priority Queue with 99 Priority) |
| **> 199** | Discarded - Values higher than 199 are not allowed and used by the priority queue. |

### SCR_AmbientSoundComponent

#### HandleQueryEntities()

- By default `SCR_AmbientSoundsComponent` performs sphere queries over entities and stores the output into appropriate data structures. Those can be then used by any configured SCR_AmbientSoundsEffects.
- From script, we can access counts of the `EQueryType` types
- On engine side references to entities of the following types are stored. There is no script API to access them directly.

#### Looped Sounds System

- System for playing looped sounds with posibility to create multiple instances of the same event and without need for a signal for each sound instance
- System keeps a pool of triggered sounds and check if, they are playing or tries to trigger them.
- Used only by insects particles
- API: see `SCR_AmbientSoundsComponent`
```enforce
SCR_AudioHandleLoop SoundEventLooped(string soundEvent, vector transformation[4])
void TerminateLooped(SCR_AudioHandleLoop audioHandleLoop)
void UpdateLoopedSounds()
```

## Activeness

For certain features like position update, or trigger functionality, the `SoundComponent` needs to be active.
That means that the `SoundComponent` is added to the `SndSystem` and `SoundComponent.UpdateSoundJob()` is called.

### Rules

- If any sound is playing on the SoundComponent, it is added to SndSystem. When the sound finishes playing, the SoundComponent is removed from SndSystem, unless additional flags on the SoundComponent are used.
- If any ACP on the SoundComponent uses triggers, the SoundComponent is always automatically added to SndSystem. There it is handled by the Dynamic Simulation, meaning the SndSystem checks the SoundComponent's distance from the listener, and if the listener is in the audible distance of the triggered sound with the largest audible range, it will start calling `SoundComponent.UpdateSoundJob()`
- Distance management flag enables addition of any SoundComponent to Dynamic simulation on SndSystem. In that case largest audible distance (evaluated from both triggered and not-triggered events is used).
- Include Inactive flag makes sure, that SoundComponent is not removed from SndSystem when the entity is deactivated.
  - If there is a trigger sound on the component and you are not sure, if EOnActivate/EOnDeactivate are called on the entity, set to true.
  - If there is a trigger sound on the component and you know EOnActivate/EOnDeactivate events are called and you want to use them to improve SndSystem performance, set to false.
- If `SoundComponent.UpdateSoundJob()` is supposed to be always called, no matter how far is listener is from the component, use OnFrameUpdate flag

### OnPostInit SndSystem Components

Components are added to `SndSystem` in `OnPostInit()` if one of the following:
- any ACP on the SoundComponent uses triggers
- OnFrameUpdate is set to true
- Distance management is set to true

**EOnActivate**
SoundComponent is added to SndSystem

**EOnDeactivate**
SoundComponent is removed from SndSystem unless **Include Inactive** is set to true
