---
title: Arma Reforger:Model Performance
url: https://community.bistudio.com/wiki/Arma_Reforger:Model_Performance
---

- ***In general***, try to keep the polycount as low as possible; of course, such decision depends on the asset type and its specific use case.
- ***In general***, the **vertex density** of different types of assets goes in this order of importance (descending, #1 = biggest density):
  1. weapon and gadget
  1. character and gear
  1. vehicle
  1. prop
  1. structure
  1. vegetation

**Important:** This does **not** mean that e.g a vehicle must always have less vertices than a weapon; it means that ***relative to its size***, a vehicle **in general** has less vertices.A knife (weapon) will definitely have less vertices compared to a haul truck (vehicle)...!

- ***In general***, create LODs using Bohemia Interactive's [Arma Reforger:Enfusion Blender Tools](https://community.bistudio.com/wiki/Arma_Reforger:Enfusion_Blender_Tools); don't hesitate fixing them too.
  - LOD0, LOD1 and the last LOD are usually created manually, intermediate LODs being creatable by tools.

**Warning:** If you import a heavily detailed 3D model as LOD0 without retouching it/creating smaller LODs,the game will keep this model in memory for as long as it will be in game's world.<br>Do not then be surprised by the mod's negative performance impact!

## Instance

Repetitive complex elements must be split to save on memory, allowing then to assemble them in Prefabs.

## Vanilla Values

As a point of reference, here are vanilla values for multiple common assets; be sure to aim for the general category's ballpark.

| Asset | Category | LOD 0 |  |  |  |  | Last LOD |  |  |  |  |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Asset | Category | **Vertices** | **Faces** | **Materials** | **Meshes** | **Bones** | **Vertices** | **Faces** | **Materials** | **Meshes** | **Bones** |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Weapons/Rifles/M16A2/M16A2_body.xob` | Weapon (Assault Rifle) | 42631 | 49842 | 7 | 7 | 12 | 1142 | 674 | 6 | 6 | 7 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Weapons/Rifles/AK74/AK74_body.xob` | Weapon (Assault Rifle) | 34614 | 41452 | 6 | 6 | 9 | 660 | 391 | 6 | 6 | 2 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Weapons/Handguns/M9/M9_body.xob` | Weapon (Handgun) | 14891 | 17515 | 2 | 2 | 6 | 213 | 114 | 1 | 1 | 2 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Weapons/Handguns/PM/PM_body.xob` | Weapon (Handgun) | 5888 | 7551 | 1 | 1 | 6 | 341 | 309 | 1 | 1 | 2 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Items/Equipment/Binoculars/Binoculars_B12/Binoculars_B12.xob` | Gadget | 4719 | 5774 | 1 | 1 | 0 | 56 | 34 | 1 | 1 | 0 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Items/Equipment/Binoculars/Binoculars_B8/Binoculars_B8.xob` | Gadget | 6919 | 8681 | 4 | 4 | 0 | 311 | 250 | 1 | 1 | 0 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Items/Equipment/Binoculars/Binoculars_M22/Binoculars_M22.xob` | Gadget | 7245 | 9965 | 1 | 1 | 0 | 107 | 76 | 1 | 1 | 0 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Basebody/Basebody_Male_Head_01.xob` | Character (Head) | 18932 | 34278 | 7 | 7 | 89 | 654 | 928 | 2 | 2 | 69 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Basebody/Basebody_Male_01.xob` | Character (Body) | 33181 | 57088 | 4 | 32 | 100 | 1230 | 1296 | 4 | 32 | 29 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/HeadGear/Helmet_M1/Helmet_M1_01.xob` | Gear (Helmet) | 7876 | 11342 | 1 | 1 | 1 | 156 | 228 | 1 | 1 | 1 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/HeadGear/Helmet_M1/Helmet_M1_Cover.xob` | Gear (Helmet) | 8391 | 12199 | 2 | 2 | 1 | 123 | 183 | 1 | 1 | 1 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/HeadGear/Helmet_SSh68/Helmet_SSh68_01.xob` | Gear (Helmet) | 2976 | 3779 | 1 | 1 | 1 | 51 | 70 | 1 | 1 | 1 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/HeadGear/Helmet_SSh68/Helmet_SSh68_01_cover.xob` | Gear (Helmet) | 3320 | 3673 | 1 | 1 | 1 | 50 | 48 | 1 | 1 | 1 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Uniforms/Jacket_BDU_M81/Jacket_BDU_M81.xob` | Gear (Top) | 6129 | 7950 | 1 | 1 | 49 | 237 | 208 | 1 | 1 | 45 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Uniforms/Jacket_KZS/Jacket_KZS.xob` | Gear (Top) | 6016 | 9968 | 1 | 1 | 35 | 252 | 278 | 1 | 1 | 34 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Uniforms/Jacket_M70/Jacket_M70_01.xob` | Gear (Top) | 11003 | 17090 | 1 | 1 | 48 | 284 | 205 | 1 | 1 | 10 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Uniforms/Pants_BDU_M81/Pants_BDU_M81.xob` | Gear (Legwear) | 4023 | 5554 | 1 | 1 | 20 | 117 | 100 | 1 | 1 | 10 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Uniforms/Pants_KZS/Pants_KZS.xob` | Gear (Legwear) | 2856 | 5092 | 1 | 1 | 16 | 176 | 220 | 1 | 1 | 20 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Uniforms/Pants_M70/Pants_M70_01.xob` | Gear (Legwear) | 6129 | 7950 | 1 | 1 | 19 | 237 | 208 | 1 | 1 | 10 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Footwear/CombatBoots_US_01/CombatBoots_US_01.xob` | Gear (Footwear) | 2176 | 3148 | 1 | 1 | 6 | 133 | 100 | 1 | 1 | 4 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Characters/Footwear/CombatBoots_Soviet_01/CombatBoots_Soviet_01.xob` | Gear (Footwear) | 2054 | 3140 | 1 | 1 | 9 | 122 | 100 | 1 | 1 | 4 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Vehicles/Wheeled/M151A2/m151a2_base.xob` | Vehicle | 34709 | 36022 | 12 | 12 | 45 | 131 | 70 | 1 | 1 | 0 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Vehicles/Wheeled/UAZ469/UAZ469_base.xob` | Vehicle | 32826 | 34755 | 13 | 13 | 37 | *2658* | *1448* | 4 | 4 | 5 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Structures/Military/Houses/Barracks_01/Barracks_01.xob` | Structure | 13455 | 11036 | 4 | 4 | 0 | 194 | 98 | 1 | 1 | 0 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Structures/Military/Houses/Barracks_E_02/Barracks_E_02.xob` | Structure | 95272 | 98776 | 15 | 15 | 0 | 178 | 94 | 1 | 1 | 0 |
| `enfusion://ResourceManager/~ArmaReforger:Assets/Structures/Military/Houses/Barracks_E_03/Barracks_E_03.xob` | Structure | 129523 | 119760 | 14 | 14 | 0 | 168 | 92 | 1 | 1 | 0 |
