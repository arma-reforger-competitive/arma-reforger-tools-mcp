---
title: Arma Reforger:Weapon Slots And Bones
url: https://community.bistudio.com/wiki/Arma_Reforger:Weapon_Slots_And_Bones
---

## Splitting Weapon

Weapon should be split into two parts - body and attachments - and all attachments can be connected to body via slots.

### Body

There are few guidelines when it comes to setting up weapon model:
- weapon itself has to be without magazine or any attachments
- the body has a "**slot**" points (dummy objects), into which attachments are being snapped in a prefab. We always name this point "**slot_description**".

**Note:** You can always check position of the existing weapons by loading them in [Resource Browser](https://community.bistudio.com/wiki/Arma_Reforger:Resource_Manager:_Model_Editor#Bones) model viewer and switching to **Bones** tab.

### Attachments

- everything, which is not core part of a weapon, like optics, magazines, grenade launchers, etc.
- each attachment has a "**snap**" point (dummy object), which is being snapped to a "slot" point on a weapon body in a prefab. We always name this point "**snap_description**".

## Slot/Snap points

When figuring out the position of slot points on weapons and snap points on attachments, following steps will help to avoid complications with possible badly modeled mesh or badly placed slot and snap points:

1. Before a production of new assets starts, create rough placeholders for all attachments and set their snap points following the rules below
1. When modeling the weapon and setting up the slot point, try as early as possible to attach all possible attachments exactly **"snap point on slot point"** (even just placeholders if final mesh is not possible) and see if all fits.
1. If some attach mechanism is the same for more weapons, the slot point must be on the the same spot in relation to the attach mechanism on these weapons (so all relevant attachments always fit in).

### Conventions

#### Slots

| What | Naming | Notes | Example |
| --- | --- | --- | --- |
| Magazine well | `slot_magazine` | Used in **MuzzleComponent**<br>- Should be setup according to the magazine<br>- in a place, where front side of the magazine touches the bottom side of the weapon body |  |
| Optics mount ***(non-standard rail)*** | `slot_optics` | Used in **AttachmentSlotComponent**<br>- node should be placed on contact surfaces<br>- for AK dovetail mount - on center of a rounded niche on outer side of the weapon body attach mechanism |  |
| Muzzle | `slot_barrel_muzzle` | Used in **AttachmentSlotComponent**<br>- suppressors, compensator, etc.<br>- node for slot should be positioned on the end of the muzzle thread |  |
| Top rail |  | Used in **AttachmentSlotComponent**<br>- should be mainly used for Picatinny rail or any other universal mounting solution | Slot placement (from [Sample New Weapon](https://github.com/BohemiaInteractive/Arma-Reforger-Samples/blob/main/SampleMod_NewWeapon/Assets/Weapons/Rifles/SampleWeapon_01/SampleWeapon_01.blend))<br>Example of mount - Picatinny rail (+dimensions) |
| Bottom rail |  | Used in **AttachmentSlotComponent**<br>- should be mainly used for Picatinny rail or any other universal mounting solution | Slot placement (from [Sample New Weapon](https://github.com/BohemiaInteractive/Arma-Reforger-Samples/blob/main/SampleMod_NewWeapon/Assets/Weapons/Rifles/SampleWeapon_01/SampleWeapon_01.blend))<br>Example of mount - Picatinny rail (+dimensions) |
| Side rail |  | Used in **AttachmentSlotComponent**<br>- should be mainly used for Picatinny rail or any other universal mounting solution | Slot placement (from [Sample New Weapon](https://github.com/BohemiaInteractive/Arma-Reforger-Samples/blob/main/SampleMod_NewWeapon/Assets/Weapons/Rifles/SampleWeapon_01/SampleWeapon_01.blend))<br>Example of mount - Picatinny rail (+dimensions) |
| Under-barrel | `slot_underbarrel` | Used in **AttachmentSlotComponent**<br>- to be used with underbarrel devices such as UGL or grip |  |
| Bayonet mount | `slot_bayonet` | Used in **AttachmentSlotComponent** |  |
| Flash-light | `slot_flashlight` | Used in **AttachmentSlotComponent** |  |

#### Attachments

| What | Naming | Notes | Example |
| --- | --- | --- | --- |
| Magazine | `snap_weapon` | - snap is located on front upper edge, in the center of the magazine |  |
| Optics | `snap_weapon` |  |  |
| Muzzle device | `snap_weapon` | - model is centered on the end of the thread |  |
| Under-barrel | `snap_weapon` | - UGLs often use specialised mounts, so it needs to be fitted according to weapon |  |
| Bayonet | `snap_weapon` |  |  |
| Bipod | `snap_weapon` |  |  |
| Flashlight | `snap_weapon` |  |  |
| Laser | `snap_weapon` |  |  |
| Barrel | `snap_weapon` | machineguns |  |

#### Simulation

Slots used by various weapon related components

| What | Naming | Notes | Example |
| --- | --- | --- | --- |
| Eye (aiming) | `eye` | Used by **SightsComponent**<br>- for each muzzle (if needed - e.g. integrated UGL yes, double-barrel shotgun no) |  |
| Barrel (firing dir) - Position | `barrel_chamber` | Used by **MuzzleComponent**<br>- for each muzzle |  |
| Barrel (firing dir) - Direction | `barrel_muzzle` | Used by **MuzzleComponent**<br>- for each muzzle |  |

#### Bones

Below names are compatible with existing [animation export profiles](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Export_Profiles)

| What | Naming | Notes |
| --- | --- | --- |
| Magazine release | `w_mag_release` |  |
| Bipod legs | `w_bipodleg`<br>`w_bipodleg_left`<br>`w_bipodleg_right` |  |
| Trigger | `w_trigger` |  |
| Safety | `w_safety` | Use only in case weapon has separate safety lever. If not, use firemode selector |
| Firemode selector | `w_fire_mode` |  |
| Buttstock | `w_buttstock` |  |
| Charging handle | `w_ch_handle` |  |
| Ejection port | `w_ejection_port` |  |
| Bolt | `w_bolt` |  |
| Bolt release | `w_bolt_release` |  |
| Slide | `w_slide` | pistols |
| Hammer / Striker | `w_hammer / w_striker` |  |
| Cylinder | `w_cylinder` | revolvers |
| Rear sight | `w_rear_sight` |  |
| Front sight | `w_front_sight` |  |
| Barrel | `w_barrel` | single - recoil operated guns, multiple - rotary cannons |
