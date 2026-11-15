---
title: Menu System
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_Menu.html
---

## Description

MenuManager handles creation, opening, closing and updating of game menus and dialogs. It connects UI layouts (the visual representation), script classes (the functionality) and action contexts (for input handling) into a single system. It provides a stack-based system where menus can overlay each other and dialogs are prioritized.

Each menu is an instance of MenuBase or its subclass. A menu preset connects three things together: the UI layout that defines how the menu looks, the script class that implements its behavior, and the action context that handles its input.

Key classes:

- MenuManager - manages the lifecycle of all menus and dialogs.
- MenuBase - base class for all menus, provides widget access, callbacks and menu item handling.

## Configuration

The list of available menus is described in a menu configuration file which is referenced in the project settings under **MenuManagerSettings** section.

Each MenuPreset has the following properties:

- **Layout** - menu UI layout resource path.
- **Class** - script class implementing the menu.
- **ActionContext** - **Context** name active while the menu is open.
- **MenuItems** - array of MenuItem objects binding widgets to actions.
- **PreloadCount** - number of pre-loaded instances. Pre-loaded menus are created in advance to avoid runtime allocation.
- **Persistent** - when enabled, the menu is not destroyed after close and can be reused.

Example configuration:

```
MenuManager {
 MenuPresets {
  MenuPreset MAIN_MENU {
   Layout "UI/Layouts/Menus/MyMenu.layout"
   Class "MyMenu"
   MenuItems {
    MenuItem confirm {
     WidgetName "ConfirmButton"
     Action "MenuSelect"
    }
    MenuItem cancel {
     WidgetName "CancelButton"
     Action "MenuBack"
    }
   }
  }
 }
}
```

## Menu Presets

Menus are identified by the **ScriptMenuPresetEnum** enum. The base enum is empty and is meant to be inherited and extended by each game project.

**Remarks:** Enum entry names must match the preset identifier names in the configuration file, as they are linked by name. For example, **MyGameMenus.MAIN_MENU** corresponds to the **MenuPreset MAIN_MENU** in the configuration above.

To register a custom enum, override the Game.GetMenuPreset method in your game class and return the inherited variant.

Example:

```
enum MyGameMenus : ScriptMenuPresetEnum
{
    MAIN_MENU,
}
 
class MyGame : Game
{
    override typename GetMenuPreset()
    {
        return MyGameMenus;
    }
}
```

## Menu Lifecycle

MenuBase provides a set of callbacks that are invoked in a specific order during the lifetime of a menu.

Opening sequence:

1. OnMenuInit - called during MenuManager config load.
1. OnMenuOpen - called when the menu is being opened.
1. OnMenuShow - called when the menu becomes visible.
1. OnMenuFocusGained - called when the menu gains focus.
1. OnMenuOpened - called after the menu is fully opened and ready.

While open:

- OnMenuUpdate - called each frame while the menu is open.
- OnMenuItem - called on menu item interaction.

Closing sequence:

1. OnMenuFocusLost - called when the menu loses focus or another menu overlaps it.
1. OnMenuHide - called when the menu becomes hidden.
1. OnMenuClose - called when the menu is closed.

## Dialogs

Dialogs are special menus opened via MenuManager.OpenDialog. They are stacked by priority using DialogPriority:

- INFORMATIVE - low priority informational dialogs.
- WARNING - warning dialogs.
- CRITICAL - critical dialogs that demand immediate attention.

Dialog results are represented by DialogResult.

## Menu Items

Menu items connect widgets to callbacks and input actions. They can be configured in the menu preset or added at runtime using MenuBase.AddMenuItem.

Use MenuBase.BindItem to register a callback for a menu item. The MenuBindAttribute attribute can be used to bind a method to a menu item declaratively.

Example:

```
class MyMenu: MenuBase
{
    [MenuBindAttribute("confirm")]
    void OnConfirm()
    {
        Print("Confirmed!");
        Close();
    }
 
    [MenuBindAttribute("cancel")]
    void OnCancel()
    {
        Close();
    }
 
    override void OnMenuOpen()
    {
        SetLabel("title", "My Menu");
    }
}
 
void OpenMyMenu()
{
    MenuManager manager = g_Game.GetMenuManager();
    MenuBase menu = manager.OpenMenu(MyGameMenus.MAIN_MENU);
}
```
