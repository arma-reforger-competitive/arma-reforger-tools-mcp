---
title: Arma Reforger:Enfusion Blender Tools: Model Quality Assurance
url: https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools:_Model_Quality_Assurance
---

## Description

Model Quality Assurance(MQA) feature provides tools to capture various issues on a model before it even gets into Enfusion. MQA isn't meant for Blender user only, it's highly recommended for Autodesk Apps users too.

## Features

The tool reports two types of issues: **Conventions**(naming) and **Mesh Topology**(various issues related to mesh data)

Reporting works while in **Object Mode** and report contains issues selected by user. If selected issue is not listed in the report, it means such issue hasn't been found.

Along with reporting, MQA also helps user to locate selected Mesh topology issue via **Validator** objects. Depending on the issue, the tool generates specific meshes to provide visual assistance in locating the problem.

### Report window

There are two ways of getting to the report window(Blender Info Log):

- once reporting is done just click on a bar at the bottom of Blender app(this message disappears after few seconds), this will open the report window

- you can simply change the editor type to **Info**

It's a known Blender issue that **Info** window updates once it gets focus by hovering mouse cursor over it. So in case information in the **Info** window is outdated after pressing the **Report!** button, just move mouse cursor into **Info** window.

## Issue Reporting

### Conventions

#### Non-Enfusion Data

Loops through all selected objects(only "MESH" object type)and reports if object name doesn't contain any of Enfusion supported:

- prefixes -> "ubx", "ucx", "usp", "ucs", "ucl", "utm", "com", "occ", "socket", "boxvol", "sphvol"
- suffixes -> "_lod0"(up to 15)

It helps to detect typos eg. "suckit_" instead of "socket_" as well as objects that may be left overs(references,helpers) hidden in collections or hidden under parent Empty(Armature), so they can be removed from file.

- *Non-Enfusion data:*

#### .xxx

Autodesk Apps allow user to name various objects by the same name eg. "house_lod0". If a model with such named objects is then imported into Blender, it renames each object as follows: "house_lod0.000", "house_lod0.001" etc. This may go under artist's radar and such named objects will be apparently skipped during FBX import in Enfusion.

- *Objects having '.xxx' suffix:*

**'usage' Property**

Loops through all selected collider objects and reports those that:

- don't have 'usage' property defined at all
  - *No 'usage' property on collider(s):*
- have 'usage' property defined, but string is empty
  - *Empty string in 'usage' property on collider(s):*
- have 'usage' property defined, but string is not from the predefined list of LayerPresets(defined in ArmaReforger.gproj)
  - *Wrong layer preset name in \'usage\' property on collider(s):*

### Mesh Topology

#### Short Edges

Loops through all selected objects and reports those their mesh contains at least one edge shorter than value of parameter **Length**.

- *Short edges:*

#### Small Faces

Loops through all selected objects and reports those their mesh contains at least one face with area smaller than value of parameter **Area**.

- *Small faces:*

#### UCX Collider

Loops through all selected **collider** objects and reports those their mesh contains:

- at least one Non-manifold edge
  - *Non-manifold edges on UCX:*
- at least one Non-planar face(ngon)
  - *Non-planar faces on UCX:*
- at least one Non-convex face
  - *Non-convex faces on UCX:*

#### UVs

Loops through all selected objects and reports those that:

- at least one vertex coordinates exceed range <32,32> in UV space
  - *Exceeding UV Coordinates:*
- at least one face has all of its vertices at (0,0), Autodesk Apps users can delete faces in UV Editor(impossible in Blender), in Blender these faces appear as unwrapped(look like single vertex at coord(0,0))
  - *Non-Unwrapped UV Faces:*
