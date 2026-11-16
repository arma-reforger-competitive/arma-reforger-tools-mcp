---
title: Arma Reforger:Audio: Vehicle Damage
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Vehicle_Damage
---

- Vehicle damage is connected to the damage state signals that are updated from the `OnStateChanged()` event on `SCR_HitZoneStateSignal` (checking `SCR_HitZone`).
- The signal setup is on `SCR_VehicleSoundComponent`'s `GetHitZoneStateSignalData()`.
- Multiple scripted hit zones can update the same signal. In that case, the signal has the value of the most damaged scripted hit zone.
- TireDamage signals are updated directly from `SCR_WheelHitZone`.

## Wheeled

### Engine

- When in idle, engine RPM is modulated `EngineDamageRPMRevs` signal, creating a sense of unsteady and unreliable engine RPM
- `SOUND_TIMING_BELT` - One-shot sound, periodically triggered. Triggering frequency depends on engine RPM
- `SOUND_ENGINE_RATTLE_LP` - Looped sound modulated by engine RPM and thrust
- `SOUND_BACK_FIRE` - One-shot sound occasionally triggered

### Gearbox

- `SOUND_VEHICLE_GEARSHIFT` - grinding sound layer is mixed in when the gearbox is damaged

### Wheel Damage

- Driving on a flat tire or rim is mixed into wheel rolling sounds if the wheel is damaged or destroyed

## Helicopters

### Engine

- `SOUND_TURBINE_LP` - the damaged turbine layer is played when the engine is damaged
- `SOUND_ENGINE_RATTLE` - one-shot sound is occasionally triggered.

### Main Rotor

- `SOUND_MAINROTOR_LP` - a loud whooshy sounding layer is mixed in, if the main rotor is damaged

### Tail rotor

- `SOUND_TAILROTOR_DAMAGED_LP` - metal grinding/scraping sound is triggered, if the tail rotor is damaged.

### Gearbox

- Metal grinding/rattling layer is mixed into `SOUND_MAINROTOR_LP` if the gearbox is damaged.

## Smoke and Fire

Three sound sources are accompanying smoke and fire particles on vehicles.
- **Engine smoke/fire:**
  - On engine position
  - Uses the EngineFireState signal
  - Representing engine smoking or vehicle fire
  - Defined on `SCR_FlammableHitZone`
  - Particles size is static, defined on `SCR_FlammableHitZone`
- **Supplies fire:**
  - On supplies position
  - Size/type of sound is driven by the SuppliesFireState signal
  - Will include ammo cook-off layer
- **Fuel tank fire**
  - On fuel tank position
  - Size/type of sound is driven by the FuelTankFireState signal

## Signals

### EngineFireState

- 0 - no particles
- 10 - 30 - smoke
- 40 - fire

### SuppliesFireState

- 0 - not burning
- 1 - 4 will return based on what particle effect is used. If small, medium, large or massive (Uses `SCR_ESecondaryExplosionScale` enum).

### FuelTankFireState

- 0 - not burning
- 1 - 4 will return based on what particle effect is used. If small, medium, large or massive (Uses `SCR_ESecondaryExplosionScale` enum).
