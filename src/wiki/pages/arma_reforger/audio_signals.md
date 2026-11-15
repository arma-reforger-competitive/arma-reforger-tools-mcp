---
title: Arma Reforger:Audio: Signals
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Signals
---

Signals are a way of interfacing between the GameWorld and Audio (and other areas).
At the core, a signal simply is a value defining the attribute of something, e.g the current RPM of a vehicle or the time of day.

Signals can be stored on Entities by using the SignalsManagerComponent.
These stored Signals will be automatically made available to the Entity's [SoundComponent](https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Sound_Components) so they can be used when generating sounds through their referenced `.acp` files.

Additionally, there is a number of Global Signals, which are stored on the Gameworld's GameSignalsManager (if one exists), as well as global [Audio Variables](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor:_Audio_Variables).
The "advanced" SoundComponents (SoundComponent + its children) will automatically access these Global Signals as well.

## Signal Setup

The Signal and Global Variables Simulation window allow audio designers and modders to easily modify signals that are used within an .acp or .sig file to 'simulate' their behaviour in game.
Although it was predominantly designed for the [Arma Reforger:Audio Editor](https://community.bistudio.com/wiki/Arma_Reforger:Audio_Editor), it is also present in the [Arma Reforger:Procedural Animation Editor](https://community.bistudio.com/wiki/Arma_Reforger:Procedural_Animation_Editor).

### Extra Options

- Edit Min/Max: Toggles to edit the minimum and maximum values of the simulated signals
- Copy all properties: Copies the minimum/maximum and value of all simulated signals
- Paste all properties: Pastes the minimum/maximum and value of all simulated signals

## Signals

**Note:** These lists are non-exhaustive.

### GameCode

#### AimingComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| AimElevation |  |  |
| AimTurn |  |  |
| AimElevationTime |  |  |
| AimTurnTime |  |  |

#### AmbientSoundsComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| AboveTerrain |  | Height (m) above the terrain. |
| SunAngle |  |  |
| RiverL |  |  |
| RiverR |  |  |
| RiverRB |  |  |
| RiverLB |  |  |
| RiverSlope |  |  |
| LakeL |  |  |
| LakeR |  |  |
| LakeRB |  |  |
| LakeLB |  |  |
| Arma Reforger 1.6.0 Grass |  |  |

#### AnimalSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Surface |  | Surface under the entity |

#### BaseStaminaComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Exhaustion |  |  |

#### CharacterAimingComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| WeaponElevation |  |  |
| WeaponTurn |  |  |

#### CharacterSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| MovementSpeed | - 0 (idle)<br>- 1 (walking)<br>- 2 (running)<br>- 3 (sprinting) | Movement type |
| Surface |  | Surface under the character.<br>**Note:** Updated only when bone containing "foot" string is defined in animation event. Not updated when character is swimming. |
| Stance | - 0 (Stand)<br>- 1 (Crouch)<br>- 2 (Prone) |  |
| Speed | 0..5.5 | Value taken from physics for playerCharacter. For remote proxy, speed is calculated as `(posLast- pos).Length() / time`. Speed is updated every 100 ms for improved accuracy and less oscillation.Speed is then clamped to a maximum of  5.5 |
| WaterDepth | 0..inf | Distance between ocean surface and terrain surface |
| WeaponStance | - 0 (Lowered)<br>- 1 (Raised)<br>- 2 (ADS) | Indicates the current stance that the character is holding the weapon. |
| RHItemSoundInfo |  | Determines what item is in the right hand of the character. |
| BackItemSoundInfo |  | Determines what item is on the character's back.<br>If there are two weapons on the back, the signal will return the weapon with higher SoundInt. |
| BackPackSoundInfo |  |  |
| JacketSoundInfo |  |  |
| VestSoundInfo |  |  |
| BootsSoundInfo |  | Determines what item is on player feet. |
| UnderPlayerControl | bool (0 or 1) | True if the character is handled by the client player. False if handled by a remote other player or AI - sort of "IsLocalPlayer". |
| SightSoundInt |  | Returns value set on `SightsComponent`. |
| IsInPlayerVehicle | bool (0 or 1) |  |
| Leaning | float -1..+1 |  |
| BushContact |  |  |
| BushHeight |  |  |

#### CharacterStaminaComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| BreathProgress |  |  |
| BreathMagnitude |  |  |
| BreathFrequency |  |  |

#### CommunicationSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| IsAI | bool (0 or 1) | Determines if the character is controlled by a player or by AI. |

#### Global

| Name | Value/Range | Description |
| --- | --- | --- |
| EnvironmentType |  | - 0 - Meadow<br>- 1 - Conifer (if Trees > 30)<br>- 2 - Leafy (if Trees > 30)<br>- 3 - Houses (if Buildings > 3)<br>- 4 - Hills (if aboveSea > 100) |
| GAboveSea |  | Distance from ocean level to camera |
| GAboveFreshWater |  | 2 if above water level<br>(-inf, 0) if submerged<br>Calculated to the camera position |
| GVehicleInterior |  | <0,1> "How much inside" the camera is within a vehicle. |
| **Removed in Arma Reforger 1.6.0<br>Replaced with Global Audio Variables** |  |  |
| GInterior | <0,1> | 1 - when player is inside |
| GRoomSize |  |  |
| GIsThirdPersonCam |  | <0,1> 1 if in third person camera |
| GCurrVehicleCoverage |  | <0,n> Coverage value of the current vehicle of the player character. |

#### HitSoundEffect

| Name | Value/Range | Description |
| --- | --- | --- |
| Surface |  | Surface that the bullet hit |
| ImpactSpeed | 0..inf | bullet velocity on impact in m/s |
| ImpactType | - 0 (None)<br>- 1 (Deflection)<br>- 2 (Penetration)<br>- 3 (LastImpact)<br>- 4 (OutOfBounds) | Bullet impact type |

#### SoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| AnimSelector | Integer | Selector signal based on userInt of animation event |
| Distance | ≥ 0 | Distance in meters between emittor & listener |
| Houses |  |  |
| Forest |  |  |
| Meadows |  |  |
| Sea |  |  |
| AboveSea |  |  |
| Interior |  | 1 if emitter inside the building (requires SoundWorld on map & buildings with properly set up BSP. |
| RoomSize |  |  |
| SurfaceWetness |  |  |

#### VehicleSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Speed | Integer |  |
| Surface |  |  |
| WaterDepth |  |  |
| Damper"N" | Integer | N - wheel number |
| Suspension"N" | Integer | N - wheel number |
| Latslip"N" | Integer | N - wheel number |
| UnderPlayerControl | bool (0 or 1) | **True** if the character is handled by the client player. **False** if handled by a remote other player or AI. |
| Brake |  |  |
| CollisionDV |  |  |
| ImpactSurface |  |  |
| HornState |  |  |
| Damage |  | In Script. Overall Damage |
| EngineDamage |  | In Script. Engine hit zone damage |
| AimTurnTimeTurret |  |  |
| AimElevationTimeTurret |  |  |
| AimTurnTurret |  |  |
| AimElevationTurret |  |  |

#### VoNComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Interior |  | 1 if emitter inside building |
| UnderPlayerControl | bool (0 or 1) | **True** if the character is handled by the client player. **False** if handled by a remote other player or AI. |
| IsInPlayerVehicle |  |  |
| GInterior |  |  |
| GIsThirdPersonCam |  |  |
| GCurrVehicleCoverage |  |  |

#### WeaponSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Suppressor | bool (0 or 1) | Suppressor attached |
| TimeSinceAutoSeqStart | ≥ 0 | time since the first shot of an automatic fire sequence (-1 if weapon does not have auto support) |
| TimeSinceAutoSeqEnd | ≥ 0 | time since the last shot of an automatic fire sequence (-1 if weapon does not have auto support) |
| IsADS | bool (0 or 1) | When Aiming Down Sights. |
| UnderPlayerControl | bool (0 or 1) | True if weapon is handeled by player character. False, if handeled by remote, AI or no character |
| IsInPlayerVehicle |  |  |
| SightSoundInt |  | Returns value set on SightsComponent. |
| Surface |  | Surface for deployed weapon |

### Script

#### SCR_AITalk

| Name | Value/Range | Description |
| --- | --- | --- |
| Vehicle |  |  |
| Character |  |  |
| TargetDistance |  |  |
| DirectionAngle |  |  |
| SoldierCalled |  |  |
| Role |  |  |
| Flank |  |  |
| Faction |  |  |
| Seed |  |  |

#### SCR_BellSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| TimeOfDay |  |  |

#### SCR_BuildingSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| WindSpeed |  |  |

#### SCR_ChimeraAIAgent

| Name | Value/Range | Description |
| --- | --- | --- |
| Seed |  |  |
| SoldierCaller |  |  |

#### SCR_ClockHandComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| ClockHand |  |  |

#### SCR_CompassComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Needle |  |  |
| Shake |  |  |
| InHand |  |  |
| InMap |  |  |
| Close |  |  |

#### SCR_EngineHitZone

| Name | Value/Range | Description |
| --- | --- | --- |
| EngineDamage |  |  |

#### SCR_FiringRangeController

| Name | Value/Range | Description |
| --- | --- | --- |
| DistanceOne |  |  |
| DistanceTen |  |  |
| DistanceHundred |  |  |
| RoundOne |  |  |
| RoundTen |  |  |
| RoundHundred |  |  |
| RoundThousand |  |  |

#### SCR_FiringRangeTarget

| Name | Value/Range | Description |
| --- | --- | --- |
| target_hit |  |  |

#### SCR_FlammableHitZone

| Name | Value/Range | Description |
| --- | --- | --- |
| Damage |  |  |

#### SCR_FuelNode

| Name | Value/Range | Description |
| --- | --- | --- |
| fuel + *tankID* |  |  |
| fuelTank |  |  |

#### SCR_GameModeCampaignMP

| Name | Value/Range | Description |
| --- | --- | --- |
| Base |  |  |
| CompanyCaller |  |  |
| CompanyCalled |  |  |
| PlatoonCaller |  |  |
| PlatoonCalled |  |  |
| SquadCaller |  |  |
| SquadCalled |  |  |
| Seed |  |  |
| TransmissionQuality |  |  |

#### SCR_GearboxHitZone

| Name | Value/Range | Description |
| --- | --- | --- |
| GearboxDamage |  |  |

#### SCR_HelicopterSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| AirSpeed |  |  |
| AltitudeAGL |  |  |
| climbRate |  |  |
| CollisionDM |  |  |
| DamageState |  |  |
| EngineAngle |  |  |
| Engine_01DamageState |  |  |
| Engine_02DamageState |  |  |
| EngineOn |  |  |
| EngineRPM |  |  |
| FireState |  |  |
| ImpactSurface |  |  |
| MainRotorRPM |  |  |
| MainRotorRPMScaled |  |  |
| PitchAngle |  |  |
| RollAngle |  |  |
| RotorMainDamageState |  |  |
| RotorTailDamageState |  |  |
| Speed |  |  |
| SpeedToCamera |  |  |
| suspension1 |  |  |
| suspension2 |  |  |
| suspension3 |  |  |
| TailRotorRPMScaled |  |  |
| TurnRate |  |  |

#### SCR_RadioComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Power |  |  |
| Channel_K |  |  |
| Channel_M |  |  |

#### SCR_SetTargetsModeUserAction

| Name | Value/Range | Description |
| --- | --- | --- |
| StartRound |  |  |

#### SCR_VehicleEngineOnCondition

| Name | Value/Range | Description |
| --- | --- | --- |
| engineOn |  |  |

#### SCR_VehicleSoundComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| RainIntensity |  |  |

#### SCR_WristwatchComponent

| Name | Value/Range | Description |
| --- | --- | --- |
| Hour |  |  |
| Minute |  |  |
| Second |  |  |
| Day |  |  |
