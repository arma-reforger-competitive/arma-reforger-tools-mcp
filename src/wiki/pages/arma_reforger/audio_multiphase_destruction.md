---
title: Arma Reforger:Audio: Multiphase Destruction
url: https://community.bistudio.com/wiki/Arma_Reforger:Audio:_Multiphase_Destruction
---

The **M**ulti**p**hase **D**estruction (MPD) system handles environmental destruction (e.g., breaking signs, crumbling walls, etc.).
Each MPD entity transitions through "damage phases", altering its model, spawning debris and particle effects, or emitting sounds.
- For efficiency, sounds are managed by the [Arma Reforger:Audio: SCR_SoundManagerModule](https://community.bistudio.com/wiki/Arma_Reforger:Audio:_SCR_SoundManagerModule) system, not individual entities. Entities reference enums in `SCR_DestructionMultiPhaseComponent` for sound definitions.
- Sounds are triggered:
  1. When an entity's damage phase changes (using `SCR_EMaterialSoundTypeBreak`).
  1. When spawned debris collides with the environment or other entities (using `SCR_EMaterialSoundTypeDebris`).
- When triggering a sound:
  1. The enum string (e.g., "METAL_HEAVY") combines with "SOUND_MPD_" to form the sound name (e.g., "SOUND_MPD_METAL_HEAVY").
  1. The system plays the sound from the `.acp` file linked to the MPDestructionManager.

**Warning:** In order for Multiphase Destruction to work, a Multiphase Destruction Manager must be present within the world.

## Design

- All sound events are defined in one `.acp`: Sounds\Destruction\Multiphase\Destruction_Multiphase.acp (`enfusion://ResourceManager/~ArmaReforger:Sounds/Destruction/Multiphase/Destruction_Multiphase.acp`)
- A "manager" needs to be present in the gameworld: Prefabs\MP\MPDestructionManager.et (`enfusion://ResourceManager/~ArmaReforger:Prefabs/MP/MPDestructionManager.et`)
- Available sound types are defined via enums (`SCR_EMaterialSoundTypeDebris` and `SCR_EMaterialSoundTypeBreak`)

## MPD Enum Overview

Sounds use enum values. Pick one of the following two to mod:
- `SCR_EMaterialSoundTypeBreak` if the sound is supposed to play when the entity changes its damage phase.<br>In this case, the enum must start with "BREAK_", e.g. "BREAK_GLASSBOTTLE";
- `SCR_EMaterialSoundTypeDebris` if the sound is supposed to play when debris collides with the environment

Add the desired name to the correct enum through modding - make sure to choose a descriptive one. Example:

```enforce
modded enum SCR_EMaterialSoundTypeBreak
{
	BREAK_GLASSBOTTLE,
}
```

| Break Sounds | Debris Sounds |
| --- | --- |
| `SCR_EMaterialSoundTypeBreak` | `SCR_EMaterialSoundTypeDebris` |
| Defined via the "Material Sound Type" entry in the "Unsorted" category .<br>These sounds will play once when the entity breaks OR changes its damage phase (PhasesToDestroyed Signal).<br>Example: A wooden fence breaking and playing a snapping/splintering sound. | Defined in the "Small Debris" classes of the destructible entity. This sound will trigger when the spawned debris collides with something else and a certain impact threshold is exceeded.<br>Example: Broken off wooden fence plank falls over and impacts with the ground. |
| \| Enum Value \| Description \|<br>\| --- \| --- \|<br>\| NONE \| No sound, default \|<br>\| BREAK_GLASS \| Small glass objects, e.g. a glass or bottle \|<br>\| BREAK_GLASS_PANE \| Larger glass objects, e.g. car or building windows \|<br>\| BREAK_GROUNDRADAR \| Dedicated sound for breaking ground radars \|<br>\| BREAK_MATRESS \| Soft thump, used for soft, cloth-like sounds \|<br>\| BREAK_METAL \| General metal break sound \|<br>\| BREAK_METAL_GENERATOR \| When a generator breaks down \|<br>\| BREAK_METAL_NETFENCE \| Metal-ish sound with elements of netfence rattling \|<br>\| BREAK_METAL_POLE \| Resonant pole break sound \|<br>\| BREAK_PIANO \| Dedicated sound for breaking pianos \|<br>\| BREAK_PLASTIC \| Sharp smaller-scale plastic cracking \|<br>\| BREAK_ROCK \| Break sound for larger rock/stone/asphalt objects, e.g. a massive wall \|<br>\| BREAK_SANDBAG \| Soft and plastic-y \|<br>\| BREAK_TENT \| Dedicated sound for tents \|<br>\| BREAK_WATERHYDRANT \| Dedicated sound for waterhydrants \|<br>\| BREAK_WOOD_SOLID \| Universal wooden break sound \| | \| Enum Name \| Description \|<br>\| --- \| --- \|<br>\| NONE \| No sound, default \|<br>\| BELL_SMALL \| Dedicated sound for small bells \|<br>\| GLASS \| Small glass objects, e.g. bottles \|<br>\| MATRESS \| Dedicated sound for matresses and pillows \|<br>\| METAL_HEAVY \| Heavy metal impacts \|<br>\| METAL_LIGHT \| Light, crisp metal impacts \|<br>\| METAL_NETFENCE \| Metal-y impacts with netfence rattling \|<br>\| METAL_POLE \| Resonant, medium-sized metal \|<br>\| PLASTIC_HOLLOW \| E.g. a tube or food container \|<br>\| PLASTIC_SOLID \| E.g. an old telephone \|<br>\| ROCK \| Concrete chunks \|<br>\| ROCK_SMALL \| More brick-like, slightly hollow \|<br>\| SANDBAG \| Soft and plastic-y \|<br>\| WOOD_PLANK_SMALL \| For small, wooden objects, e.g. fence boards \|<br>\| WOOD_PLANK_LARG \| For large(r) wooden objects, such as smaller tree trunks \| |

## Signals

Not all signals are available for each type of sound.

| Signal Name | Description | Available for Break Sounds | Available for Impact Sounds |
| --- | --- | --- | --- |
| PhasesToDestroyed | An integer indicating how damaged the entity is.<br>-   0 = Total destruction<br>-   1 = Broken<br>- > 1 = Anything else | Yes | No |
| EntitySize | The entity's mass, usually used in order to "scale" sounds. | Yes | Yes |
| CollisionDV | A value reflecting the change in speed of an entity upon contact (how fast was it before, how much was it slowed down?) | No | Yes |

## Debugging

If any issues arise, the first step should always be to make sure an MPDestructionManager entity exists in the world.
Check if the `SCR_DestructionMultiPhaseComponent` is actually enabled (if it is an Entity with integrated MPD functionality, make sure "Enabled" is checked in the "Unsorted" tab).
Entities with integrated MPD functionality will always use the values stored in their Prefabs - meaning a per-instance change will not work. A Prefab change also requires a reload of the current world.

### Diag Menu

If a `DestructionMPDestructionManager.et` is present in the world, a [Arma Reforger:Diag Menu](https://community.bistudio.com/wiki/Arma_Reforger:Diag_Menu) entry can be found:

```
Diag Menu \ Sounds \ Show MPD Impulse Values
```

Upon colliding, the debris entity will then display the impulse data in the format `<impulse value>/<impulse threshold>/<mass>`.

### Sound Definition Check

In the main Workbench Window (**not** World Editor), the "Multiphase Destruction Soundless Prefab Search" tool can be accessed via Plugins → Prefabs.
The tool will go through all prefabs and list in the debug console log those which do not have a "Break" or "Impact" sound defined.
