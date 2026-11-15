---
title: Arma Reforger:World Editor: Terrain Creation Tool
url: https://community.bistudio.com/wiki/Arma_Reforger:World_Editor:_Terrain_Creation_Tool
---

**Important:** Before anything: **save frequently**. After height map importation, normal maps, terrain texture and layer maps import/generation as they are **not** saved to file after the operation.

**Note:** For more advices and tips, see [Arma Reforger:Terrain Tutorial](https://community.bistudio.com/wiki/Arma_Reforger:Terrain_Tutorial).

## Manage

### Height Map

The height map is terrain altitude's information. World Editor can work with both `.asc` and 16-bit `.png` height map files.
The section offers options to import, export, and rebuild the height map.

#### Import Height Map

This option allows to import a height map from an external source or a World Editor export.

It allows to **invert in X and Z axis**, importing the terrain "from right to left" (a hill being West of the terrain would be East) or "from bottom to top" (South would be North).
Ticking both corresponds to a 180° terrain rotation.

The **resampling heights** option allows to override the terrain's altitude range by linear interpolation. It is required for png import as the png format only holds *relative*

**Example**

| Original (-10 to +10m range) | -10 | -5 | 0 | 1 | 5 | 10 |
| --- | --- | --- | --- | --- | --- | --- |
| **Imported (0 to +100m range)** | 0 | 25 | 50 | 55 | 75 | 100 |

#### Export Height Map

The **Base**/**Modified** option targets the terrain's height map - either the base terrain one, or the one modified by e.g road generators and other entities that edit terrain through script.

#### Rebuild Height Map

This button allows to rebuild the height map manually in the event it is not synchronised properly with the actual data (road, building placements).

#### Bake Selection

This "bakes" the selected entities' terrain modification into the base height map.

For example: if a building is placed on a slope, it flattens the terrain by default; this flattening follows the building and is destroyed with the building.
With this button the building's flattening is stored into the height map and will remain even if that building is moved or deleted.

#### Generate Normal Map

Allows to (re)generate the terrain's normal map. Normal map interpolation settings (minimum and maximum angle for vertex interpolation) are only available when the Interpolation Mode option is set to `Auto`.

### Satellite Map

#### Import satellite map

This imports a satellite map for the terrain, used for long-distance terrain texture.

### Surface Map

#### Fix block border

This is to be used if hard edges between textures of neighbouring terrain blocks appear.

#### Change surface map size

#### Rebuild terrain materials

**Rebuild terrain materials** must be run if a layer map reimport goes wrong - in the event of wrong or missing clutter or when terrain material tracing does not work properly.

### Options

### Tile Map

The Tile Map is an overhead map of the terrain tiles.

#### Minimap Controls

Hold [RMB] to drag the map around.

##### Scale To Fit

Fit the minimap to the view widget.

##### Center on Camera

Move the minimap so the camera position is centered.

##### Claim Tiles

Lock tiles so only your account can work on them.

##### Release Tiles

Release tiles so everyone can work on them.

##### Cogwheel

**Set Background Image**
Set the minimap's texture.

**Note:** See also [2D Map Creation](https://community.bistudio.com/wiki/Arma_Reforger:2D_Map_Creation).

**Clear Background Image**
Remove the minimap's texture.

## Sculpt

### Common Values

**Strength**: strength of the effect - range 0..500

**Height** (in **Flatten** mode): wanted terrain's absolute height

**Radius**: radius (in metres) range 0..200

**Falloff**: the "cone/cylinder" slope ratio (see below) - range 0..100

**Angle**: the effect angle - range 0..360

### Common Controls

| Controls | Effect |
| --- | --- |
| [Shift+Mouse up]<br><br>[Shift+Mouse down] | Set strength/height |
| [Ctrl+Mouse up]<br><br>[Ctrl+Mouse down] | Set radius |
| [Ctrl+Shift+Mouse up]<br><br>[Ctrl+Shift+Mouse down] | Set falloff |
| [Ctrl+Shift+Mousewheel] | Set angle |

### Sculpt

[LMB] raises terrain according to Strength.

[Alt+LMB] lowers the terrain.

### Flatten

Sets the terrain to a defined Height. **Space** picks the height of the current terrain position under the cursor.

### Smooth

Softens edges.

### Noise

Generates Perlin-based terrain noise.

## Paint

A specific terrain tool used for painting surface materials on terrain ground.

- Paint tool is used for painting surface materials on a terrain ground with an adjustable brush. All paint brush properties can be set in the same tab.
- Information where each surface material is painted is stored as a mask inside game files upon saving a project. However, it is possible to both import and export each surface's mask in a .png file.
- As of Arma Reforger v0.9.5 there is a limit of 5 surfaces per block for a terrain.

| Feature | Screenshot | Description |
| --- | --- | --- |
| **Basic Brush Settings** |  | **Show diagnostics for block under cursor** – this option enables a simple surface layer list per block tab in World Editor viewport when using Paint tool and hovering over terrain.<br>Information about block under cursor will be shown. Useful as an overview to manage surfaces in multiple blocks efficiently while painting.<br>**Note:** See the Sculpt section above for more information.<br>**Strength**<br>range: 0..40<br>Strength of the brush. [Shift+MouseUp]/[Shift+MouseDown] to adjust.<br>**Radius**<br>range: 0..200<br>Radius of the brush. [Ctrl+MouseUp]/[Ctrl+MouseDown] to adjust.<br>**Falloff**<br>range: 0..100<br>Fadeout of the brush. [Ctrl+Shift+MouseUp]/[Ctrl+Shift+MouseDown] to adjust.<br>**Angle**<br>range: 0..360<br>Rotation of the brush – useful for rectangular or user brushes. |
| **Brush Shape and Type** |  | - **Brush shape** – Linear/Smooth/Sphere/Inverted Sphere – these options define the style of painting, icons next to these tabs work as an indication what each option provides.<br>- **Brush type** – Round/Rectangle/User. User brushes are loaded from root/Brushes (i.e. root/Brushes/blot.png). |
| **Surface Layer List** |  | - Surface layer list – A simple drag & drop list of surface materials. These can be dropped from either Workbench or World Editor's Resource Browser.<br>- The very first surface layer is always used as default and while it can not be removed or imported, it can be changed to a different one.<br>In this scenario, Grass_03 is used as a default surface. When masks are stored in game files, they are all merged together. Any information that is remaining from the resulting merged mask is used as default layer. |
| **Basic Painting** |  |  |
| **Extra Surface Layer Operations** | <br> | - **Change layer's material...** – Change surface material to a one not included in the list.<br>- **Clear surface layer** – selected surface layer's mask will be cleared with default surface layer.<br>- **Fill surface layer (clear other)** – selected surface layer will fill the entire terrain ground, replacing every other surface layer.<br>- **Remove surface layer** – removes surface layer from the list.<br>- **Priority surface mask import** – will automatically import mask of a selected surface layer.<br>- **Priority surface mask import...** – will import a specific surface layer mask.<br>- **Export surface mask** – will automatically export mask of a selected surface layer.<br>- **Export surface mask...** – will export a specific surface layer mask.<br>- A simple batch import and export feature for surface masks is also available – these options work with masks from all surfaces at once.<br>*All surface masks are in a .png format and for the best results a predetermined sizing should be followed – this information can be found in the Info & Diags tab (section below).* |

## Info & Diags

This tab contains useful info about blocks and their surface layers. These diagnostics also allow either manual merging or replacing of surface layers in specific blocks.

| Step | Screenshot | Description |
| --- | --- | --- |
| **Block and Pixel Under Cursor Diagnostics** |  | - In the lower part of the **Info & Diags** tab, there is an extra information about blocks containing painted surface layers.<br>- While in **Info & Diags** tab, you can press Ctrl+X in World Editor's viewport to visualize surface layers painted on the terrain ground. Diagnostic colors are set in each surface layer.<br>**Block**<br>- **Live (under cursor)** – diagnostics will describe the block actually under cursor in World Editor's viewport. If the tile is clicked then this checkbox becomes unticked.<br>- **Camera auto-align** – when a block is selected, camera will automatically align to it.<br>- **Coordinates** – it is possible to input specific tile coordinates and have camera automatically moved to their location. Below is a list of specific pixel coordinates of height map, satellite texture and surface mask. As each has a different size, the coordinates are different. Listed coordinates are Top-left coordinates/Bottom-right coordinates.<br>- Percentage list determines the mask coverage of a surface layer in a selected block. These percentages work with surface layer mask blending as well, meaning that the total value can be up to 500%.<br>**Pixel Under Cursor**<br>- **Coordinates** – same as above.<br>- The Pixel Surface statistics list shows surface layers per pixel. The range is set from 0 to 255 (black - white) and always add up to **255**. It is possible to determine exactly which pixel contains what surface layers through this data. |
| **Basic Surface Diagnostics** |  | - **Merge** – merges selected surface layer with already existing one.<br>- **Replace** – replaces selected surface layer with a layer not present in selected block.<br>- A 3x3 grid is an indicator of selected block and its neighbours. The orientation is based on cardinal directions.<br>- **Green** – there is a free slot for that surface layer.<br>- **Yellow** – selected block.<br>- **Red** – the limit of 5 surface layers is reached in that block. |
| **Surface Layer Merge or Replace Operations** |  | 1. Select a block in which you want a surface layer merged or replace.<br>1. Select specific surface in the percentage list by right-clicking on it.<br>1. Pick between either *Merge* or *Replace* options.<br>1. If *Replace* is further expanded, pick a surface layer which you want to replace selected one.<br>1. If there is a red square in the 3x3 grid, the 5 surface layer limit is reached in that block and it needs to be processed by further merging surface layers there to empty the slot. |
