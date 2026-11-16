---
title: Arma Reforger:Audio: SCR SoundManagerModule
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_SCR_SoundManagerModule
---

`SCR_SoundManagerModule` is a module that plays simple one-shot sounds without the need for a `SoundComponent` to exist on a given entity.
- `SCR_SoundManagerModule` is a core entity present in every world.
- If functions in `SCR_SoundManagerModule` are enough for a given sound, prioritise it before adding a `SoundComponent` on the entity.
- If the entity already has `SoundComponent` because of some other sound, do not use `SCR_SoundManagerModule`.
- Signals for a given sound can be set only before the sound playback and cannot be updated during the sound playback.

**Important:**
- `SCR_SoundManagerModule` is **not** present on headless applications (e.g dedicated server).
- Do **not** use `SCR_SoundManagerModule` for UI sounds.
- Do **not** use `SCR_SoundManagerModule` for managing looped sounds. Looped sounds always need `SoundComponent` to work properly in multiplayer.

**Note:** Before Arma Reforger 1.6.0, `SCR_SoundManagerModule` was named `SCR_SoundManagerEntity`.

## Signals

- By default, the following Variables are used:
  - `GCurrVehicleCoverage`
  - `GIsThirdPersonCam`
  - `GInterior`
- the `Distance` signal can be enabled using the flags in `SCR_AudioSourceConfiguration`
- any additional feature-specific signals can be added using the `SCR_AudioSource` API if needed.

## SCR_AudioSourceDefinition

Arma Reforger 1.3.0

- Class used for setting sound parameters
- This class is used in `SCR_SoundDataComponent`, or can be used in any feature in script if needed.
  - Sound Project (resource) - acp that contains m_sSoundEventName
  - Sound Event Name (string) - Sound event name
  - Static (flag) - If false, sound will follow the entity's position

## SCR_SoundDataComponent

`SCR_SoundDataComponent` is the best way to configure multiple sounds

It contains an `SCR_AudioSourceDefinition` array for sounds configuration and `SCR_SoundManagerModule` contains methods that can work with this component directly.

## Usage

### SCR_SoundManagerModule Scripting

Sound is defined using `SCR_AudioSourceConfiguration`. You need it to play the sound. Two methods:
- use `SCR_SoundDataComponent`, which contains the `SCR_AudioSourceConfiguration` array and is meant to be used when multiple scripts need to play sound on a given entity.
- define `SCR_AudioSourceConfiguration` directly in a class. Useful when better control over `SCR_AudioSourceConfiguration` is needed or when there is only a couple of events to be played on the entity

### SCR_SoundDataComponent Workflow

Store the string event name in script, then two methods:
1. create and play sound using `CreateAndPlayAudioSource(IEntity owner, string eventName)`
1. alternatively create `SCR_AudioSource` using `CreateAudioSource(IEntity owner, string eventName)` - checks if the sound is in the audible range and play `SCR_AudioSource` using `PlayAudioSource(SCR_AudioSource audioSource)` - sets occlusion signals, distance signal, triggers the sound, and adds `SCR_AudioSource` to the pool.

### SCR_AudioSourceConfiguration Workflow

Validate a `SCR_AudioSourceConfiguration` instance, then two methods:
- create and play sound using `CreateAndPlayAudioSource(IEntity owner, SCR_AudioSourceConfiguration audioSourceConfiguration)`
- alternatively Create a `SCR_AudioSource` using `CreateAudioSource(IEntity owner, SCR_AudioSourceConfiguration audioSourceConfiguration)` and play `SCR_AudioSource` using `PlayAudioSource(SCR_AudioSource audioSource)`
