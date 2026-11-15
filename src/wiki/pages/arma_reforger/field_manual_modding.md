---
title: Arma Reforger:Field Manual Modding
url: https://community.bistudio.com/wiki/Arma_Reforger:Field_Manual_Modding
---

The **Field Manual** is the helpful in-game manual available at all time from either the main menu or the pause menu.

Here we will see how to add our own entries.

## Config Modding

**Note:**
- the actual structure is: Root Config > Categories > SubCategories > Entries > Pieces.
- the structure itself can support an infinite amount of levels, but the UI does not.
- a Category and a SubCategory are the same `SCR_FieldManualConfigCategory` object - they only differ in their hierarchy level.

### Root Config

The Field Manual's root config can be found here: FieldManualConfigRoot.conf (`enfusion://ResourceManager/~ArmaReforger:Configs/FieldManual/FieldManualConfigRoot.conf`) (in `Data\Configs\FieldManual`).

It holds:
- the Field Manual's main title
- the tab names
- various layouts
- categories (other `.conf`)
- tile backgrounds from which it will select randomly

### Add a Category

A category holds:
- its title
- sub-categories

**Note:**
- A sub-category is identical to a category (`SCR_FieldManualConfigCategory`) - they only differ in hierarchy level.
- Even if first-level categories can hold *entries*, they should not for the time being (they will be ignored as the UI does not support them).
- Even if second-level categories can hold *categories*, they should not for the time being (they will be ignored as the UI does not support them).

| Variable | Description |
| --- | --- |
| Enabled | This allows to enable/disable (show/hide) the category. |
| Title | This is the category's title - it will also be shown in the config entry name |
| Categories | Sub-categories to this category |
| Entries | Sub-category entries to display (the "tiles" and their content) |
| Category Layout | FieldManual_MenuCategory.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/MenuParts/FieldManual_MenuCategory.layout`) by default |
| Sub Category Layout | FieldManual_MenuSubCategory.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/MenuParts/FieldManual_MenuSubCategory.layout`) by default |

### Add a Sub-Category

Repeat the same operation as above in a Category instead of the config root.

### Add an Entry

The following entry types are available:
- No `SCR_FieldManualConfigEntry` - the parent type, not to be used directly
- Yes `SCR_FieldManualConfigEntry_Standard` - the standard type, full width page
- Yes `SCR_FieldManualConfigEntry_Weapon` - an unused weapon type with an automatically-filled weapon statistics' side panel (with rate of fire, weight, etc)

| Variable | Description |
| --- | --- |
| Enabled | This allows to enable/disable (show/hide) the entry. |
| Id | This ID is an `EFieldManualEntryId` enum constant - required only if this entry is to be opened from the outside (e.g a hint) through `SCR_FieldManualUI.Open(enumValue);` |
| Title | This is the entry's title, both for the tile and for the opened entry - it will also be shown in the config entry name |
| Image | The tile content. Recommended definition: 400×300. |
| Content | A list of `SCR_FieldManualPiece` used to define and setup the entry's content. |
| Weapon Entity Path | `SCR_FieldManualConfigEntry_Weapon` only: the path to the weapon's prefab (e.g Rifle_AK74.et (`enfusion://ResourceManager/~ArmaReforger:Prefabs/Weapons/Rifles/AK74/Rifle_AK74.et`)) |
| Layout | - FieldManual_Entry_Standard.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Entries/FieldManual_Entry_Standard.layout`) by default for `SCR_FieldManualConfigEntry_Standard`<br>- FieldManual_Entry_Weapon.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Entries/FieldManual_Entry_Weapon.layout`) by default for `SCR_FieldManualConfigEntry_Weapon` |

### Add Content

An Entry's content is made out of multiple pieces. All pieces are vertically set, one after another - there are no "image to the left, scroll to the right" elements.

The available piece types are as follow:

#### SCR_FieldManualPiece

`SCR_FieldManualPiece` - the parent type, not to be used directly.

#### SCR_FieldManualPiece_ConfigEntry

`SCR_FieldManualPiece_ConfigEntry`.

| Variable | Description |
| --- | --- |
| Display Name | The config entry text |
| Config Path | the `.conf`/`.et` file to read |
| Entry Path | the config path, in format `path/to/entry` (case-sensitive!) |
| Value Format | A specific format can be used along with `%1` |
| Layout | FieldManual_Piece_ConfigEntry.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_ConfigEntry.layout`) by default |
| Decimal Move | Move the comma left (negative) or right (positive), range -9..+9, default 0 |
| Fixed Decimals | Define the amount of number's decimals, range -1..5, default -1 |

#### SCR_FieldManualPiece_ConfigEntryList

SCR_FieldManualPiece_ConfigEntryList - a list of SCR_FieldManualPiece_ConfigEntry.

| Variable | Description |
| --- | --- |
| Config Entries | A list of `SCR_FieldManualPiece_ConfigEntry` - see SCR_FieldManualPiece_ConfigEntry above |
| Layout | FieldManual_Piece_ConfigEntryList.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_ConfigEntryList.layout`) by default |

#### SCR_FieldManualPiece_Header

`SCR_FieldManualPiece_Header` - a text "title".

| Variable | Description |
| --- | --- |
| Image Path | Can be an `.edds` file or an `.imageset` |
| Image Set Name | If an image set has been used in **Image Path**, set the image name here |
| Text | The header's text |
| Layout | FieldManual_Piece_Header.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_Header.layout`) by default |

#### SCR_FieldManualPiece_Image

`SCR_FieldManualPiece_Image` - a simple image.

| Variable | Description |
| --- | --- |
| Image Path | Can be an `.edds` file or an `.imageset` |
| Image Set Name | If an image set has been used in **Image Path**, set the image name here |
| Image Color | Can be used to colour an image with a colour filter |
| Caption | The text associated to the image; be wary as some layouts cannot take a long text |
| Layout | FieldManual_Piece_Image.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_Image.layout`) by default |

#### SCR_FieldManualPiece_ImageGallery

`SCR_FieldManualPiece_ImageGallery` - a gallery of `SCR_FieldManualPiece_Image`.

| Variable | Description |
| --- | --- |
| Type | One of `SCR_EFieldManual_ImageGalleryType`, can be either `ICONS_VERTICAL`, `ICONS_LIST`, `GALLERY_HORIZONTAL` or `GALLERY_VERTICAL` |
| Text | The text displayed along the gallery |
| Images | A list of `SCR_FieldManualPiece_Image` - see SCR_FieldManualPiece_Image above |
| Layout | `enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_ImageGallery.layout` by default |

#### SCR_FieldManualPiece_Keybind

`SCR_FieldManualPiece_Keybind` - a keybind information.

| Variable | Description |
| --- | --- |
| Keybind | The keybind in format `<action name="ActionName"/>` |
| Description | The keybind's description |
| Input Display Condition | One of `SCR_EInputTypeCondition`, can be either `ALL_INPUTS`, `KEYBOARD_ONLY` or `GAMEPAD_ONLY`:<br>- `ALL_INPUTS`: the keybind is always displayed<br>- `KEYBOARD_ONLY`: the keybind is only displayed if keyboard/mouse are currently used<br>- `GAMEPAD_ONLY`: the keybind is only displayed if gamepad is currently used<br>**Note:** The UI will automatically show/hide the appropriate keybinds when controls change. |
| Layout | FieldManual_Piece_Keybind.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_Keybind.layout`) by default |

#### SCR_FieldManualPiece_KeybindList

`SCR_FieldManualPiece_KeybindList` - a list of `SCR_FieldManualPiece_Keybind`.

| Variable | Description |
| --- | --- |
| Keybinds | A list of `SCR_FieldManualPiece_Keybind` keybinds - see SCR_FieldManualPiece_Keybind above |
| Layout | FieldManual_Piece_KeybindList.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_KeybindList.layout`) by default |

#### SCR_FieldManualPiece_LineBreak

`SCR_FieldManualPiece_LineBreak` an empty space, used to separate elements where a bigger gap is needed.

No variables.

#### SCR_FieldManualPiece_Separator

`SCR_FieldManualPiece_Separator` - a Reforger-themed horizontal separator

| Variable | Description |
| --- | --- |
| Padding Top | Top padding, range 0..100, default 20 |
| Width Percentage | Total parent's width, range 0..100, default 75 |
| Thickness | "Height" of the separator, range 0..10, default 2 |
| Padding Bottom | Bottom padding, range 0..100, default 20 |
| Layout | FieldManual_Piece_Separator.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_Separator.layout`) by default |

#### SCR_FieldManualPiece_Text

`SCR_FieldManualPiece_Text` - a block of text.

| Variable | Description |
| --- | --- |
| Text | The paragraph's content |
| Layout | FieldManual_Piece_Text.layout (`enfusion://ResourceManager/~ArmaReforger:UI/layouts/Menus/FieldManual/Pieces/FieldManual_Piece_Text.layout`) by default |

## Script Modding

The concerned classes are:
- **`SCR_FieldManualUI`** for the UI management
- `SCR_FieldManualConfigRoot` for the root config
- `SCR_FieldManualConfigCategory` for categories
- `SCR_FieldManualConfigEntry` for entries
  - `SCR_FieldManualConfigEntry_Standard`
  - `SCR_FieldManualConfigEntry_Weapon`
- `SCR_FieldManualPiece` and children for the content pieces
- `EFieldManualEntryId` for the entry ID

### Open Field Manual

```enforce
EFieldManualEntryId entryId = EFieldManualEntryId.CONFLICT_OVERVIEW;
SCR_FieldManualUI.Open(entryId);
```
