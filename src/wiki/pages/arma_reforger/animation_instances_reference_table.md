---
title: Arma Reforger:Animation Instances Reference Table
url: https://community.bistudio.com/wiki/Arma_Reforger:Animation_Instances_Reference_Table
---

Below tables provide reference of animations and [export profiles](https://community.bistudio.com/wiki/Arma_Reforger:Animation_Export_Profiles) which should be used when [exporting TXA animations from Blender](https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools:_Import/Export_Animation#Export_animations_from_Blender).

## Rifle

[**Sample New Weapon**](https://community.bistudio.com/wiki/Arma_Reforger:Weapon_Creation) ([download link](https://github.com/BohemiaInteractive/Arma-Reforger-Samples/blob/main/SampleMod_NewWeapon/Assets/Weapons/Rifles/SampleWeapon_01/Anims/SampleWeapon_01_Animations.blend)) is used here as a **reference**, so not all animations are present - like *Finger_trigger_in* or *Sight*.
In such a case, **animations of AK74** are provided to give some guidance on what should be filled in.

### Weapon Instance

| Action name | Description | Animation take used | Action used | Export profile | Export as |
| --- | --- | --- | --- | --- | --- |
| BoltPose | Poses of the bolt.<br>- First frame represents closed bolt<br>- Last frame represents open bolt | 4 | w_sampleweapon_fire | 724_Weapon_Bolt | w_rfl_sampleweapon_01_bolt_pose |
| Finger_trigger_in |  | 6 |  | 999_Empty |  |
| Finger_trigger_out |  | 6 |  | 999_Empty |  |
| Fire | Animation of weapon being fired - trigger is pressed and bolt is doing one full cycle | 3 | w_sampleweapon_fire | 724_Weapon_Bolt_Trigger | w_rfl_sampleweapon_01_fire |
| Idle | Empty animation | 1 |  | 999_Empty |  |
| Idle_finger | Empty animation | 1 |  | 999_Empty |  |
| IKOffset | No animation assigned | - | - | - | - |
| Reload_InsertMag | Animation of inserting magazine into the weapon. 722_Weapon_MagRelease profile can be used if there is animated weapon magazine release on the weapon | 72 | rfl_sampleweapon_erc_insert_mag | 999_Empty | w_rfl_sampleweapon_01_mag_insert |
| Reload_RemoveMag | Animation of removing magazine from the weapon. 722_Weapon_MagRelease profile can be used if there is animated weapon magazine release on the weapon | 59 | rfl_sampleweapon_erc_remove_mag | 999_Empty | w_rfl_sampleweapon_01_mag_remove |
| ReloadActionBolt | Animation of racking the bolt | 36 | rfl_sampleweapon_erc_bolt_rack | 999_Empty | w_rfl_sampleweapon_01_bolt_rack |
| Safety | Safety mode animation. First frame safety mode, second single mode and third one is used for full auto | 3 | w_sampleweapon_safety | 701_Rifle_Idle | w_rfl_sampleweapon_01_safety |
| Sight | Iron sight adjustment animation. Amount of frames is equal to amount of zeroing modes defined in the weapon | 11 |  | 723_Weapon_Sight |  |
| Switch_Mode_Off | Empty animation for switching safety switch off. This animation need to have same amount of frames as the one in player instance | 16 |  | 999_Empty |  |
| Switch_Mode_On | Empty animation for switching safety switch on. This animation need to have same amount of frames as the one in player instance | 16 |  | 999_Empty |  |
| Trigger | Animation of trigger when its pressed | 4 | w_sampleweapon_fire | 721_Weapon_Trigger | w_rfl_sampleweapon_01_trigger |

## Player Instance

| Action name | Description | Number of frames | Animation take used | Export profile | Exported as |
| --- | --- | --- | --- | --- | --- |
| BoltPose | - | - | - | - | - |
| Finger_trigger_in |  | 6 |  | 502_IK_RightHand |  |
| Finger_trigger_out |  | 6 |  | 502_IK_RightHand |  |
| Fire | Animation of weapon being fired - trigger is pressed and bolt is doing one full cycle | 3 |  | 401_RIndexABS |  |
| Idle | Empty animation | 1 | using same animation as **Safety** |  |  |
| Idle_finger | Empty animation | 1 |  | 502_IK_RightHand |  |
| IKOffset | IK offset animation | 1 | p_sampleweapon_ik | 101_FullBodyABS | p_rfl_sampleweapon_01_offset |
| Reload_InsertMag | Animation of inserting magazine into the weapon | 72 | rfl_sampleweapon_erc_insert_mag<br>rfl_sampleweapon_pne_insert_mag | 252_UpperbodyADD_ArmsABS | p_rfl_sampleweapon_01_erc_mag_insert<br>p_rfl_sampleweapon_01_pne_mag_insert |
| Reload_RemoveMag | Animation of removing magazine from the weapon | 59 | rfl_sampleweapon_erc_remove_mag<br>rfl_sampleweapon_pne_remove_mag | 252_UpperbodyADD_ArmsABS | p_rfl_sampleweapon_01_erc_mag_remove<br>p_rfl_sampleweapon_01_pne_mag_remove |
| ReloadActionBolt | Animation of racking the bolt | 36 | rfl_sampleweapon_erc_bolt_rack<br>rfl_sampleweapon_pne_bolt_rack | 252_UpperbodyADD_ArmsABS | p_rfl_sampleweapon_01_erc_bolt_rack<br>p_rfl_sampleweapon_01_pne_bolt_rack |
| Safety | Safety mode animation. First frame safety mode, second single mode and third one is used for full auto | 3 | p_sampleweapon_ik | 502_IK_RightHand | p_rfl_sampleweapon_01_safety |
| Sight | Iron sight adjustment animation. Amount of frames is equal to amount of zeroing modes defined in the weapon | 11 | *empty anim, need same length as Sight in weapon* | 999_Empty | - |
| Switch_Mode_Off | Empty animation for switching safety switch off. This animation need to have same amount of frames as the one in player instance | 16 | *it is possible to use same anim as Switch_Mode_On* | 262_ArmsADD_RHandABS | - |
| Switch_Mode_On | Empty animation for switching safety switch on. This animation need to have same amount of frames as the one in player instance | 16 | p_rfl_sampleweapon_01_modes_on | 262_ArmsADD_RHandABS | p_rfl_sampleweapon_01_modes_on |
| Trigger | Animation of finger when trigger is pressed | 4 | using same animation as **Fire** |  |  |
| Animation IK Pose | Inverted Kinematics pose defined inside **SCR_WeaponAttachmentsStorageComponent** in Item **Animation Attributes** sections under name **Animation IK Pose** | 1 | p_sampleweapon_ik | 501_IK | p_rfl_sampleweapon_01_ik |
