---
title: Arma Reforger:Enfusion Blender Tools: Import/Export ASC Elevation
url: https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools:_Import/Export_ASC_Elevation
---

This feature allows to import an [ESRI asc grid](https://en.wikipedia.org/wiki/Esri_grid) format into Blender as regular grid mesh as well as to export regular grid mesh back to ASC file (and import it in World Editor).

**Note:** This tool is a very simplistic implementation that imports the whole file at once; therefore, depending on grid resolution, operations may take some time.

## Import ASC File

In the Enfusion Blender Tools menu, use **Import > ASC elevation**.

Once the import is done, `Min/Max elevation` and `Elevation range` information is printed out in the Info log.

## Export ASC File

Select the terrain object first, then in the BT menu use **Export > Terrain to ASC**.

Once the export is done, *Min/Max elevation* and *Elevation range* info is printed out in Info editor.

## Elevation Modification

The elevation of an imported terrain mesh can be modified in various ways in Blender, e.g via Edit/Sculpt mode or by using procedural approaches.

Terrain mesh is represented as a regular grid and must remain as such after any modifications are done to mesh, otherwise elevation changes can appear in a slightly shifted position.

### Edit Mode

In **Edit** mode, the safest way to prevent such issue is to only shift vertices in the vertical axis by dragging the gizmo's blue arrow which locks movement to Z (vertical) axis.

### Sculpt Mode

In **Sculpt** mode, user can prevent vertices shifting in horizontal plane by locking the X and Y axis in the following menu:
