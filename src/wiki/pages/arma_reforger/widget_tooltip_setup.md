---
title: Arma Reforger:Widget Tooltip Setup
url: https://community.bistudio.com/wiki/Arma_Reforger:Widget_Tooltip_Setup
---

ScriptedWidgetTooltips is an Enfusion API for tooltips that is currently employed in core menus.
It is expanded in scripts by `SCR_ScriptedWidgetTooltip`, and allows setting up a customisable tooltip triggered by the mouse hovering a particular widget.

Most widgets in `.layout` files support it, which can be set in their **Behavior** section. Choose `SCR_ScriptedWidegtTooltip` as the Tooltip class.

Once the class is set, there are two fields to provide: a `.conf` file and a tag. The config file must be of `SCR_ScriptedWidgetTooltipPresets` type.

Similarly to Configurable Dialogs, the tooltip config provides modular options for tooltip's appearance and behaviour. Override the `Content` class to provide further customisation.

The tooltips are automatically displayed on mouse hover and gamepad focus.

`SCR_ScriptedWidgetTooltip` provides invokers (`enfusion://ScriptEditor/scripts/Game/UI/Components/WidgetLibrary/SCR_ScriptedWidgetTooltip.c;41`) for different stages of a tooltip's lifetime.

```enforce
// Invokers
// Reliance on static invokers is a bit of a bandaid solution but there is no other way for other scripts to access the tooltip class
// If possible, only bind on hover/focus gained and make sure to unbind on lost.
protected static ref ScriptInvokerTooltip m_OnTooltipShowInit;	// Called before creating the content widget
protected static ref ScriptInvokerTooltip m_OnTooltipShow;		// Called after creating the content widget
protected static ref ScriptInvokerTooltip m_OnTooltipHide;		// Called after removing the content widget
```

These are static, so make sure to check that the tooltip firing an invoker is the one your are actually interested in!
You can use the `IsValid()` method, which checks for tags and optionally for hoverWidget and config file.
