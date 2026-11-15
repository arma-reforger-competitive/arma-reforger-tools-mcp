---
title: Arma Reforger:Game Master: Image Generation Tutorial
url: https://community.bistudio.com/wiki/Arma_Reforger:Game_Master:_Image_Generation_Tutorial
---

Preview Images are special types of automatically generated images used in for instance **Game Master Asset Browser**.

Scripts described in this tutorial will attempt to pick most appropriate background and camera settings for selected prefab and then, it will replace **existing PNG** file next to **already existing EDDS** file defined in **Image** property located inside editable entity component (depending on the type of the asset, it can be **SCR_EditableEntityComponent** , **SCR_EditableCharacterComponent**, **SCR_EditableVehicleComponent** or **SCR_EditableGroupComponent**). By default, automatically generated Preview Images are located in **UI/Textures/EditorPreviews/Auto ** folder.

**Important:** Therefore, it is **required to run [Create/Update Selected Editable Prefabs plugin](https://community.bistudio.com/wiki/Arma_Reforger:Game_Master:_Editable_Entities_Configuration)** or manually add that component to the asset if it is missing (f.e. when doing some character or vehicle prefab which doesn't inherit from existing base prefabs) before trying to generate the images. **Create/Update Selected Editable Prefabs** plugin will take care of creating appropriate Editable Component, creating editable variant (when it is necessary) and will generate placeholder preview images which can be later replaced via this script.

In case some non standard asset need an image, then it is possible to create custom world with user selected camera parameters.

# Generating Preview Images

## Load World

In World Editor, load [enfusion://WorldEditor/worlds/Editor/Slots/AssetImages/Eden_AssetImages.ent;5896.25,225.667,7097.26;-34.9951,53.0787,0 Worlds/Editor/Slots/AssetImages/Eden_AssetImages.ent]

## Adjust Screen Resolution

Select **SCR_EditorImageGeneratorEntity** in default layer. Doing so will show current and target resolution in top left corner. Readjust the viewport to make values match.

**Note:** If the viewport cannot be made narrow enough, force it to 4:3 resolution in *Camera > View Aspect Ratio > 4:3*

**Warning:**
- The default value of 400x300 should be maintained, as otherwise image stretching or resolution issues might appear in the .edds file
- If you have problems with generating pictures even though **Current resolution** matches **Target resolution** then make sure you are using **100% display scale** in Windows display settings.

## Select Prefabs

In World Editor's resource browser, select prefabs (*.et) you want to capture.

Don't worry about selecting extra files like prefabs of parts of *.conf files, they will simply be ignored.

## Play

Play the world. The system will spawn all selected prefabs one by one and take a screenshot of each of them.

**Note:** The system will spawn the entities on locations with matching labels. If the location does not match, or no location can be found, check the labels in the *EditableEntityComponent* of the prefab you are trying to capture images of.

Note that this component only exists on Editable versions of prefabs, with the exception of *Characters*, *Groups*, and *Vehicles*. Because of this it is important to select the right prefab type when taking images.

**Important:** When prompted, in **Script Authorization Required** window click **Yes to All**, in order to give necessary privileges to preview images generator plugin.
Alternatively it is possible to use [scriptAuthorizeAll](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#scriptAuthorizeAll) startup parameter.

# Configuring Custom World

In case you would like to create custom world for preview images, following steps have to performed.

**Note:** In case of troubles, [enfusion://WorldEditor/~worlds/Editor/Slots/AssetImages/Eden_AssetImages.ent *worlds\Editor\Slots\AssetImages\Eden_AssetImages.ent*] can be used as an example how prepare world for **Preview Images** generation.

## Create Manager

Insert **SCR_EditorImageGeneratorEntity** in the world.

## Create a Position

Insert **SCR_EditorImagePositionEntity** in the world.

- Set to entities of which labels it belongs, as well as time and weather used when the position is activated.
  - A position is selected when all its labels are part of the queried prefab. Any extra labels the prefab may have are not taken into consideration.
  - Example: A position marked as **VEHICLE, TRUCK** will be picked for prefab with **VEHICLE, TRUCK**, US, GREEN, but not for **VEHICLE**, CAR, US, GREEN
- Optionally set preview mesh used to represent the position.
- For groups, place entities of the same type as children of the position entity. Each of these sub-positions will serve as a slot for individual soldier.
- For characters, or characters in groups, animation poses can be configured
  - Select the right source by configuring the *Graph*, *Instance*, and *Start Node*
  - Set the ID to set up the desired pose
  - Force weapon type if the character might not hold this weapon by default. *E.g. an AT Soldier*
- It is recommended to give the image position a name which helps identifying the position in the hierarchy.

## Create a Camera

Insert **SCR_CameraBase** as a child of the position entity and add **Hierarchy** component to it.

- Use *Set entity to view orientation* plugin to move the camera to where World Editor's camera is. This will copy only position and rotation, not FOV!
- Optionally add **SCR_PostProcessCameraComponent** with custom post-process effects.
- Enable *Show Debug View Cone* to see where the camera is pointed to even in World Editor.

## Test the Position

Play the world with desired position entity selected. This will activate only this position, while ignoring all the others.

You still need compatible prefabs selected in **World Editor's** resource browser.

## Repeat

Return back to create position paragraph and place as many positions as you need to cover all prefabs.
