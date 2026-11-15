---
title: Arma Reforger:Enfusion Blender Tools: Rig Updater
url: https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools:_Rig_Updater
---

# Description

This tool replaces existing character rig with newer version. Everything located in **LOD0, Rig, Reference, IK Targets** or **Extra** collection will be deleted and replaced by version from newer blend file.

During upgrade process, following things will be performed:

- Replacement of old Rig with newer version
- Restoration of all NLA tracks assigned to old Rig
- Restoration of all bone constrains present in the Rig
- Updating of constrains present on all non Rig & Character objects which were previously pointing to Rig or Character

**Warning:** Be careful - updater tries to handle most of the migration steeps but it is still possible that it might encounter some issue. Proceed with caution and perform backup of your blend file before attempting to upgrade the rig!

# Usage

1. Download latest [Character_AnimationRig_RigUpdater.blend](https://github.com/BohemiaInteractive/Arma-Reforger-Samples/blob/main/SampleMod_NewCharacter/Assets/Characters/SampleCharacter/Character_AnimationRig_RigUpdater.blend)
1. Open blend file with rig which you want to update
1. In Enfusion Tools tab, navigate to **Animation Tools** and click on **Update Rig** button
1. Locate [Character_AnimationRig_RigUpdater.blend](https://github.com/BohemiaInteractive/Arma-Reforger-Samples/blob/main/SampleMod_NewCharacter/Assets/Characters/SampleCharacter/Character_AnimationRig_RigUpdater.blend)  and confirm selection by clicking on **Update rig** button
