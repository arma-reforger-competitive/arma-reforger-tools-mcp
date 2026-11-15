---
title: Arma Reforger:Weapon Creation/Asset Preparation
url: https://community.bistudio.com/wiki/Arma_Reforger:Weapon_Creation/Asset_Preparation
---

## Prepare the Mesh

This tutorial will try to cover procedure for plugging in new weapon into Enfusion Workbench.
While tutorial might be more focused on Blender users, other software users shouldn't be worried since most of the things are working very similar in other programs.

### Object Orientation

One of the most important thing to begin with is making sure that your **model is properly orientated**. As per [Arma Reforger:FBX Import](https://community.bistudio.com/wiki/Arma_Reforger:FBX_Import#Alignment) page:

**Note:** Everything must be oriented as pointing along/towards **the Y+ axis in Blender and 3dsMax** and **along the Z+ axis in Maya**.

That means, when you are **importing A3 weapon you need to rotate it by 90 degrees to the left.**

### Object Cutting

Most likely model that you might already have few bits already present in the mesh.
Since Enfusion allows you to assemble weapon from multiple parts, we will split our mesh into multiple separate objects to achieve much higher customization of weapon.

In this case, sample contains two parts which could be potentially moved to separate files, letting you to have 3 variants in total quite easily.
As marked on above video, grip & iron sights were moved to separate FBX files.
Magazine was already separated for that particular mesh so only remaining thing to do is adding slot points for those accessories.

### Object Naming

When it comes to naming, there are few important rules to keep in mind.

1. **_LODx** suffix is used to indicate **Level of Details**
1. **UBX_, UCX_, USP_, UCS_, UCL_, UTM_** prefixes are used to mark **Colliders**
1. **OCC_** prefix is used for **Geometry Occluders**

Beside that, there are also some additional guidelines regarding naming of the objects - **[slot/snap points naming convention](https://community.bistudio.com/wiki/Arma_Reforger:Weapon_Slots_And_Bones)** - itself which don't have effect on how mesh is processed by engine (like those mentioned above) but are there to **have consistency**.
It's also worth to note that some of base weapon prefabs are using some of those names **by default** (they can be of course changed but it's strongly advised to follow those rules nevertheless).

### Add Slots/Snap Points

If you had experience with Arma 3 modding, then whole concept of having slots & snap points should be fairly familiar to you.
Those dummy objects are serving as *memory points -* there is one major difference though - you no longer have to place two points to make an axis.
Instead, rotation of dummy object is used.

In Blender, you can use one of the **empty objects** like **Plain Axis** to create those helper points.

*Please notice the Plain Axis gizmo - this is the orientation of the model. Make sure that your empty object is **properly aligned***.

**Note:** One easy method to have slot & snap points correctly aligned is to create first slot points, when the mesh is still in one piece, and then copy paste mesh & empty socket to new empty scene.
After that only thing left is to rename ***slot_XX*** to ***snap_XX***.

On **Sample Weapon**, the following slots were created:

- **slot_ironsight_front & slot_ironsight_rear -** slots for picattinny mounted ironsights
- **slot_magazine -** slot for magazine well
- **slot_optics -** slot for top mounted optics
- **slot_underbarrel -** slot for bottom mounted accessories like bipod

Additional, following empty objects were created for various components:

- **snap_hand_right -** **IK target** for right hand when using **weapon deployment** feature
- **snap_hand_left -**  **IK target** for left hand when using **weapon deployment** feature
- **barrel_chamber & barrel_muzzle -** those points are used in **MuzzleComponent** to determine location & direction where bullet is spawned
- **eye -** point for aiming down sight view. Used in **SightsComponent**

### Colliders & Material Names

Colliders are special type of objects which are used to calculate various kinds of collisions - be it physic simulation or tracing of bullet penetration.
There are few rules regarding those colliders and most of them listed [FBX Import - Colliders usage](https://community.bistudio.com/wiki/Arma_Reforger:FBX_Import#Collider_usage) page.

**Weapon requires at least one collider with** **Weapon [layer preset](https://community.bistudio.com/wiki/Arma_Reforger:Collision_Layer)** - if you don't have it, then **weapon actions like equip will be missing from it**.
If your geometry is simple enough, it is possible to just use one collider for both weapon & fire geometry collision.
Otherwise it might be necessary to have two colliders:

- One for **Weapon** collision - should be very simple collider (i.e. convex)
- Another for **FireGeo** collision - can be more detailed, trimesh can be used to provide best experience available.

When importing asset from previous Arma game - like in this example - you are most likely going to have already convex components ready from **Geometry, Fire Geometry, View Geometry** or **Geometry Physx LODs** and in this case automatically **Fire Geometry LOD from Arma 3 were used as FireGeo** and **mesh from Geometry LOD** was used for **Weapon** layer.

Now you might ask how to assign **Layer Presets**.
If you are using Blender, there is small handy tool - [Objects Tool](https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools:_Objects_Tools) - which is part of [Enfusion Blender Tools](https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools) to assist you with assigning correct game materials & layer presets on colliders.
Otherwise, take a look at [FBX Import](https://community.bistudio.com/wiki/Arma_Reforger:FBX_Import#Setting_Layer_Preset_on_colliders_.28usage_parameter.29) page where there are instructions how to set that parameter in 3DS Max or Maya.

**Note:** You can use [Model Quality Assurance](https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools#Model_Quality_Assurance) to verify if your colliders are convex by **checking UCX Collider** option.

### Skeleton Setup & Mesh Rigging

#### Skeleton Creation

Whether it's new model or mesh imported from P3D, most likely it will be necessary to prepare skeleton. In **Blender** skeletons are called **[Armatures](https://docs.blender.org/manual/en/latest/animation/armatures/index.html)** and their creation process is quite straightforward.
Starting with [creation of **Armature itself**](https://docs.blender.org/manual/en/latest/animation/armatures/introduction.html#your-first-armature), this can be done by selecting **Armature** option from **Add** menu in top section of the viewport while being in **Object** mode.

**Important:** It is recommended to keep your armature called **Armature** since in such scenario, you will avoid artificial bone in skeleton hierarchy when mesh is imported in Workbench.
While it might be not causing any issue on rifles, it is basically a **necessary thing when importing character related gear**.

Once it is created, you might notice that root bone is quite large and might wish to resize it - to do so, switch to **Edit Mode** and select that **Bone** and then reduce its size to some reasonable level.

**Important:** Armature should be located at **0,0,0 point and use 1.0 scale**.
Otherwise you might encounter some problems when trying to animate it in Blender at later stage.

Next step will be rotation of this bone towards the **front of the weapon** and then renaming it to **w_root** - this bone will serve us a **root bone** of this mesh.

#### Bones Creation

After basics of armature are done, it is time to add bones for all animated parts on the weapon. While in **Edit Mode** and with **Armature** selected, it is possible to add new bones through either:

- **Add → Single Bone** option in top bar of viewport
- By [duplicating bone](https://docs.blender.org/manual/en/latest/animation/armatures/bones/editing/duplicate.html) already present in the armature - like **w_root**

If you choose the first option, then it will be necessary to resize & rotate the new bone to the desired size. Duplication doesn't have this problem, so it was chosen to create the fire mode switch bone - **w_fire_mode**. It is also important to keep the hierarchy of bones in order, and in this case it means that all bones responsible for moving parts of the weapon - **like w_fire_mode** - should be **parented to w_root**. This can be done selecting bone in **Edit Mode and then changing Parent property in Relations section of Bone properties** tab.

*Please note that GIF above is Using Blender [Industry Compatible Keymap](https://docs.blender.org/manual/en/latest/interface/keymap/industry_compatible.html) *

**Note:** Consider setting bones visibility to **In Front** - this will help with placement of the bones.<br>

There are also few thing to keep in mind when creating bones and below is list of those things:

- Bones will serve as axis for any further motion so try to place them in **spots which would result in reasonable motion**, like center of **fire selector pivot point** or **center of bolt**.
- **Keep Y+** orientation of the bones - this might be especially handy is some action is using center of bone for actions
- While it's not necessary, it is recommended to use vanilla **[naming convention for bones](https://community.bistudio.com/wiki/Arma_Reforger:Weapon_Slots_And_Bones#Bones)** - this will be especially useful when dealing with animation export, since vanilla animation export profiles expects such bone names.
  - *It is still possible to use custom names but this might require custom export profiles - more about it will mentioned in the chapter covering weapon animation*
With this knowledge in mind, it should be possible to create rest of the bones like **w_trigger**, **w_charging_handle** and **w_bolt**.

#### Mesh Skinning

Skinning of the mesh depends on the software that you are using and if you don't know how to do it in software of your choice, it is recommended **to search for some tutorials on the web what skinning of bones is**.
**Note:** If you **imported model from P3D** via Enfusion Blender Tools then most likely you will have some vertex groups already - in this case you can try to **rename** those so they **match skeleton bone names**.
**Recommended read**: Official Blender documentation - **[Assigning a Vertex Group](https://docs.blender.org/manual/en/latest/modeling/meshes/properties/vertex_groups/assigning_vertex_group.html)**.
If you are using **Blender**, here you can find short instruction how to quickly setup skinning on object. In the example below, **w_bolt** bone will be set:

1. Switch to **Object Mode**
1. Select object which you want to skin - in this it is **body_02_LOD0**
1. In **[Modifiers Properties](https://docs.blender.org/manual/en/latest/modeling/modifiers/introduction.html#interface)** tab, add **[Armature](https://docs.blender.org/manual/en/latest/modeling/modifiers/deform/armature.html)** modifier via **Add Modifier** button
1. In **Object** property of **Armature** **modifier** select **Armature** (skeleton) which should influence this object - in this case it is **Armature**
1. In **Object Data Properties** expand [**Vertex Group** panel](https://docs.blender.org/manual/en/latest/modeling/meshes/properties/vertex_groups/vertex_groups.html)
1. Switch to **[Edit Mode](https://docs.blender.org/manual/en/latest/editors/3dview/modes.html)** and then activate **face** **selection mode**
1. Select faces which should belong to bolt
1. In **Vertex Group section, click on** plus button to **[Add Vertex Group](https://docs.blender.org/manual/en/latest/modeling/meshes/properties/vertex_groups/assigning_vertex_group.html#creating-vertex-groups)**
1. Double click with **Left Mouse Button** on it and change name of that new vertex group from **Group** to **w_bolt**
1. Click on **Assign** button in **Vertex Groups** section (assuming you still have selected bolt faces in viewport) - this will assign your current selection in viewport to **w_bolt** vertex group with full influence (*influence is controlled by **[Weight](https://docs.blender.org/manual/en/latest/modeling/meshes/properties/vertex_groups/vertex_weights.html)** property*)

At this stage **w_bolt** bone should be successfully rigged but that is not the end of the process!

**Important:** Below section applies to any 3D software!

When creating skinned objects, **Enfusion Workbench** expects that the whole object is skinned to some bone.

**Important:** Otherwise importer will try to "fix" it by skinning remaining faces to some root bone and in console log you will see below message:

```
RESOURCES (W): Missing some mesh skinning weights (Object_LOD0). Weighting them to root
```

In Blender realms, this means that any object which **Armature** modifier, must be fully skinned to some existing bone through vertex groups.
In this case it means that all faces, **beside those which belongs to** **w_bolt** vertex group, on **body_02_LOD0** object has to be skinned to **w_root** bone.
In this case it was done by selecting faces belonging to **w_bolt** and then [inverting the selection *via* the [Ctrl+I](https://docs.blender.org/manual/en/latest/interface/keymap/industry_compatible.html#selection) shortcut].
After that, new vertex group called **w_root** was created and current selection was assigned to it via **Assign** button.

#### Colliders Rigging

If you have some animated collider please keep in mind that **only trimesh colliders can be skinned**. In all other cases you have to use **100% weight**.

In Blender it gets even bit more tricky and **Object Relations** (in ***Object tab***) have to be used if you want connect non trimesh collider to some skeleton bone.

**Note:** It is also recommended to parent slots on the weapon to the **Armature** - it is not necessary to parent it to w_root bone though. This should make it easier to snap magazine in reload sequence for instance

## FBX Export Settings

Most of the general rules can be found on **[FBX Import page](https://community.bistudio.com/wiki/Arma_Reforger:FBX_Import#FBX_Export)**.
In principle, when exporting from i.e. 3DS Max, you have to make sure that you are exporting in **binary format in version 2014/2015**.
Furthermore, **Triangulation** & **Preserve Edge Orientation should be turned off**.

**Blender**-wise, there are 3 most important things to keep in mind when exporting FBX:

| **1. Object Types**:<br>For animated object like weapon you need to have checked on at least:<br>**Empty** - which handles all snap points<br>**Armature** - exports skeleton of your weapon<br>**Mesh** - self explanatory |  |
| --- | --- |
| **2. Custom Properties**:<br>Without this option all custom properties like **LayerPresets** would be lost! |  |
| **3. Leaf bones**:<br>Leaf bones are completely unnecessary in Enfusion and it's better to have that option **turned off** |  |

## Textures Preparation

When importing textures of weapon from previous Real Virtuality games like Arma 3 there is no real automated or simple method of conversion spec-gloss textures to PBR ([Physicial Based Rendering](https://en.wikipedia.org/wiki/Physically_based_rendering)) Metal Rough ones - current industry standard. Therefore in most cases it's much easier to do textures from scratch in i.e. Substance Painter.

There are tons of materials on the internet how to create proper PBR texture and it's highly recommend to search for it via some popular search engines. In case of Substance Painter, it is worth to take a look at [Substance Painter PBR guide](https://substance3d.adobe.com/tutorials/courses/the-pbr-guide-part-1).

If you still want to try convert **RV spec gloss textures to PBR**, you can try to follow this [Converting A2/A3/Dayz textures to PBR for Reforger](https://www.mod-fusion.com/post/converting-a2-a3-dayz-textures-to-pbr-for-reforger) tutorial.
**Recommended read:** [Textures](https://community.bistudio.com/wiki/Arma_Reforger:Textures)**.

## Model Import & Registration

Once mesh was successfully prepared and all selections, sockets & snap points are in place, it's time to try our asset in game.
Majority of the process is already pretty well described on the [Arma Reforger:FBX Import#Import process in the Workbench](https://community.bistudio.com/wiki/Arma_Reforger:FBX_Import#Import_process_in_the_Workbench) page.

In principle, all you have to do is click with right mouse button on your FBX files and select **Register and Import** option from the context menu.

### Colliders & Materials Check

After initial import was done it's time to make sure that materials & colliders are using proper materials & colliders.
By default, Enfusion will try to assign material based on the name of the assigned texture in mesh.
If it fails to find such texture, new dummy material (see area marked in orange on screen below) will be created next to the FBX model.

There are 2 typical errors when it comes to collider configuration:

**Important:**
- Make sure that you have "usage" property defined in collider object properties:
- Make sure that the correct material is assigned to all colliders:

More info can be found on [Arma Reforger:FBX Import](https://community.bistudio.com/wiki/Arma_Reforger:FBX_Import).

### Skeleton & Hierarchy

Next we will take care of bones - in this case, magazine object has only empty objects (snap point) and to import them, checking of **Export Scene Hierarchy in Miscellaneous section of Import Settings tab should be enough**.

In case of **skinned assets like rifles**, **Export Skinning** option should be used instead.
It might be also good to know that **Export Scene Hierarchy is** not necessary when **Export Skinning** option is selected.

Below process of setting **Export Scene Scene Hierarchy on magazine** is showcased and then analogical process of using of **Export Skinning** on **SampleWeapon_01.xob** is presented.

If for some reason you don't see bones icon on **SampleWeapon_01.xob even after checking Export Skinning** and reimporting resource, make sure that you have properly skinned your model in 3D software of your choice.

**Note:** Changes made in **Import Settings** tab are only applied to model after **manually reimporting model** via **Reimport resource (PC)** button.

## Texture Import

In principle, you can use same procedures as the ones described on [Weapon Modding](https://community.bistudio.com/wiki/Arma_Reforger:Weapon_Modding#From_Scratch) page.
**Recommended read: [Textures](https://community.bistudio.com/wiki/Arma_Reforger:Textures).

By default, **Workbench** should already **create some of the materials based on their material name in FBX**, so in case of SampleWeapon_01.xob (`enfusion://ResourceManager/~SampleMod_NewWeapon:Assets/Weapons/Rifles/SampleWeapon_01/SampleWeapon_01.xob`), there should be already some emats which needs to be properly configured.

- **SampleWeapon_01_Camo1.emat** material with two textures
  - **SampleWeapon_01_Camo1_BCR -** Base color + Roughness
  - **SampleWeapon_01_Camo1_NMO -** Normal map
- **SampleWeapon_01_Camo2.emat** also with two textures
  - **SampleWeapon_01_Camo2_BCR -** Base color + Roughness
  - **SampleWeapon_01_Camo2_NMO -** Normal map

Since in this example, magazine is sharing textures with rifle itself, it is necessary to adjust textures over there.
To do so, double click on **XOB file of magazine (`enfusion://ResourceManager/~SampleMod_NewWeapon:Assets/Weapons/Magazines/SampleWeapon_01/Magazine_65x39c_SampleWeapon_01_30rnd.xob`)** to open it in a new **Resource Manager** tab.
After that, it is possible to set up materials which were previously created for the weapon itself.

Materials can be assigned in 2 ways:

- Drag and dropping desired material on material icon in **Materials** tab
  - This action will automatically reimport model with selected material
- Changing **Material Assign** in **Visual section** of **Import Settings**
  - It will be necessary to click on **Reimport resource (PC)** button after applying changes

**Note:** See the next step: **[Prefab Configuration](https://community.bistudio.com/wiki/Arma_Reforger:Weapon_Creation/Prefab_Configuration)**.
