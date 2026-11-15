---
title: Arma Reforger:Audio: Building Doors
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Building_Doors
---

Door sounds do **not** use `SoundComponent` and are played directly via the `AudioSystem`.

## Prefab Configuration

- The `.acp` reference is configured on `DoorComponent`'s **SoundFileName**.
- Sounds positions are configured on `DoorComponent`'s **SoundPoints**.

## Events

- `SOUND_OPEN_START` - triggered only when the door is fully closed
- `SOUND_OPEN_FINISH`
- `SOUND_CLOSE_START` - triggered only when the door is fully open
- `SOUND_CLOSE_FINISH`
- `SOUND_MOVEMENT` - triggered when the door starts to move, stopped when the door stops moving
