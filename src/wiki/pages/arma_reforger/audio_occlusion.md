---
title: Arma Reforger:Audio: Occlusion
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Occlusion
---

## Signals

| Signal Name | SoundComponent | CharacterSoundComponent | WeaponSoundComponent | VehicleSoundComponent | VoNComponent | CommunicationSoundComponent<br>Takes values from `SignalsManagerComponent` and stores into a snapshot. | SoundManager |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Interior | 0..1 | 0..1 | 0..1 | 0..1 | 0..1 | 0..1 | 0..1 |
| UnderPlayerControl | n/a | True if the character is the player character | True if the weapon is handled by the player character | True if the player character is in the vehicle | True if the character is the player character | True if the character is the player character | n/a |
| IsInPlayerVehicle | n/a | True if the character is in the same vehicle as the player character | True if the weapon is a child of the vehicle, the player character is in. | n/a | True if the character is in the same vehicle as the player character | True if the character is in the same vehicle as the player character | n/a |
| RoomSize | Returns volume in m3 of room the entity is currently in | Returns volume in m3 of room the entity is currently in | Returns volume in m3 of room the entity is currently in | Returns volume in m3 of room the entity is currently in | Returns volume in m3 of room the entity is currently in | Returns volume in m3 of room the entity is currently in | n/a |

## Buildings

Building occlusion describes sound attenuation based on whether or not a sound emitter and the listener are in the same room (or a room at all).
The system goes hand-in-hand with Light Portals and the room areas created by the system.

### Signals and Variables

There are four main signals and [variables](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Audio_Variables) that handle building occlusion:

- `GInterior`: float between 0 and 1 (1 if the current camera position is inside a building, 0 if outside, may gradually interpolate from 0 to 1 when entering a portal).
- `Interior`: per-entity signal that returns 1 if the entity is inside a building.
- `GRoomSize`: returns the size of the room the listener is located in, or 0 if it's not in any room.
- `RoomSize`: per-entity signal that returns the size of the room the entity is located in, or 0 if it's not in any room.

The listener and emitter are considered to be "in the same room" if
- `GInterior` OR the emitter's Interior signal are 1 **and**
- `GRoomSize` and the emitter's `RoomSize` signal are identical

### Examples

| **Listener inside, emitter outside (or vice versa)**<br>If the listener is inside (`GInterior` = 1) and the emitter is outside (`Interior` = 0), or vice versa, full attenuation is applied. |  |
| --- | --- |
| **Listener inside, emitter inside, matching roomSizes**<br>If both listener and emitter are inside (`GInterior` = 1, `Interior` = 1) and GRoomSize and the emitter's RoomSize signal are identical, then the emitter is considered to be in the same room as the listener. |  |
| **Listener inside, emitter inside, but no matching roomSizes** |  |
| **Portal radius**<br>Portals have a certain radius around them at which the `Interior` and `GInterior` values gradually lerp from 0 to 1. If an entity stands in the middle of a doorway, for example, the `Interior` signal will return 0.5 (`GInterior` for the listener respectively). With the current occlusion setup, this is only used for players/listeners. |  |
| **Edge-case: Different buildings, same RoomSizes**<br>If the emitter and listener are in two different buildings and the rooms they are in happen to have identical room sizes, no attenuation will be applied. |  |

## Vehicles

Occlusion inside vehicle cabins uses the Coverage System and is defined by the `GCurrVehicleCoverage` [audio variable](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Audio_Variables) as an arbitrary float value.

- Each compartment of a vehicle can be given a "base coverage" value, defined on the `CompartmentManagerComponent`.
- Inside the SlotManagerComponent, using the "Slot Mappers" class, each Slot can be assigned a coverage value that will be added to the overall coverage as long as the slot is not destroyed (e.g windows can break, resulting in a lowering of the coverage value).
- Therefore, the overall coverage for each vehicle department is the sum of the Base Coverage defined in the `CompartmentManagerComponent` and all additional slot coverage values defined in the `SlotManagerComponent`.
