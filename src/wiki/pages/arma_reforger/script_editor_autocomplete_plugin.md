---
title: Arma Reforger:Script Editor: Autocomplete Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Autocomplete_Plugin
---

- **name**: Autocomplete
- **editor**: script
- **descr**: A plugin to autocomplete/autofix the current line
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/ScriptEditor/SCR_AutocompletePlugin.c`
- **shortcut**: [Ctrl+Return]
The **Autocomplete** [Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor) plugin helps script authors by providing autocompletion and smart insertions for e.g common code structures, code snippets, boilerplate code etc. It also assists with attribute decorators, log level consistency, null and validity checks, and more.

## Features

- Script snippets on keyword (see Default Keywords below for a full list of default keywords)
- Instantiation help (see Autocomplete below)
- Safety checks: adds a null check after a cast (if not present), adds a validity check after `Resource.Load` (if not present)
- Attribute decorators: automatically suggests/adds appropriate `[Attribute(/* ... */)]` decorators for variables
- Log level consistency: ensures `Print` and `PrintFormat` method calls have explicit log levels
- User customisation: supports user-defined keywords/snippets and attribute decorators; defaults can be edited or reset.

## How to Use

In Script Editor, type a supported keyword or code pattern then press [Ctrl+Return] to execute the plugin.
The keyword gets replaced with the corresponding snippet, or the line will be auto-fixed/augmented as needed.
Many behaviours are configurable in the plugin settings via Workbench and custom keywords can be added.

## Default Keywords

| Keyword | Description | Result |
| --- | --- | --- |
| `if` | `if` structure | ```enforce<br>if (condition)<br>{<br>}<br>``` |
| `ife` | `if-else` structure | ```enforce<br>if (condition)<br>{<br>}<br>else<br>{<br>}<br>``` |
| `for` | `for` loop temporarily storing the iteratedclength | ```enforce<br>for (int i, count = arr.Count(); i < count; i++)<br>{<br>}<br>``` |
| `forr` | Reversed `for` loop (from end to start) | ```enforce<br>for (int i = arr.Count() - 1; i >= 0; i--)<br>{<br>}<br>``` |
| `foreach` | `foreach` loop (item type to be replaced) | ```enforce<br>foreach (SCR_Class item : items)<br>{<br>}<br>``` |
| `foreachi` | Indexed `foreach` loop | ```enforce<br>foreach (int i, SCR_Class item : items)<br>{<br>}<br>``` |
| `switch` | `switch-case` structure | ```enforce<br>switch (value)<br>{<br>case 0:<br>break;<br>default:<br>break;<br>}<br>``` |
| `while` | `while` structure | ```enforce<br>while (condition)<br>{<br>}<br>``` |
| `class` | `class` basic structure | ```enforce<br>class SCR_MyClass<br>{<br>protected string m_sValue = "Generated class";<br>//------------------------------------------------------------------------------------------------<br>//! constructor<br>void SCR_MyClass()<br>{<br>}<br>}<br>``` |
| `method`/`func` | Method/Function structure (indentation depending on keyword's indentation) | ```enforce<br>//------------------------------------------------------------------------------------------------<br>//! \param[in] parameter<br>protected void Method(int parameter)<br>{<br>}<br>``` |
| `ctor`/`dtor` | Current class's constructor/destructor<br>`IEntity` constructor if class name ends with `Entity` | ```enforce<br>//------------------------------------------------------------------------------------------------<br>// constructor<br>void SCR_CurrentClassName()<br>{<br>}<br>```<br>```enforce<br>//------------------------------------------------------------------------------------------------<br>// constructor<br>void SCR_CurrentClassNameEntity(IEntitySource src, IEntity parent)<br>{<br>}<br>```<br>```enforce<br>//------------------------------------------------------------------------------------------------<br>// destructor<br>void ~SCR_CurrentClassName()<br>{<br>}<br>``` |
| `findcomp` | Script component-finding structure with null check | ```enforce<br>SCR_ComponentClass component = SCR_ComponentClass.Cast(entity.FindComponent(SCR_ComponentClass));<br>if (!comp)<br>return;<br>``` |
| `print` | A simple `Print` line with log level | ```enforce<br>Print("fill", LogLevel.NORMAL);<br>``` |

## Autocompletion

| Type | Examples |
| --- | --- |
| Instantiation | ```enforce<br>SCR_Class instance =<br>// or<br>SCR_Class instance = new<br>// or<br>SCR_Class instance = new SCR_Class;<br>// becomes<br>SCR_Class instance = new SCR_Class();<br>```<br>```enforce<br>map<string, ref SCR_MyLongClassName> myMap =<br>// becomes<br>map<string, ref SCR_MyLongClassName> myMap = new map<string, ref SCR_MyLongClassName>();<br>```<br>```enforce<br>array<string> list = new array<string>();<br>// becomes<br>array<string> list = {};<br>``` |
| Attribute decorator creation | ```enforce<br>protected bool m_bShowWhenEmpty;<br>// becomes<br>[Attribute(defvalue: "1", desc: "Show When Empty")]<br>protected bool m_bShowWhenEmpty;<br>```<br>```enforce<br>protected SCR_EBoxSide m_eBoxSide;<br>// becomes<br>[Attribute(defvalue: "0", desc: "Side To Show", uiwidget: UIWidgets.ComboBox, enumType: SCR_EBoxSide)]<br>protected SCR_EBoxSide m_eSideToShow;<br>``` |
| Log level check | ```enforce<br>Print("Hello there");<br>// becomes<br>Print("Hello there", LogLevel.NORMAL);<br>```<br>```enforce<br>PrintFormat("General %1!", "Kenobi");<br>// or<br>PrintFormat("General %1!", "Kenobi", LogLevel.NORMAL);<br>// becomes<br>PrintFormat("General %1!", "Kenobi", level: LogLevel.NORMAL);<br>```<br>```enforce<br>PrintFormat("You are a %1 one", "bold", LogLevel.WARNING);<br>// becomes<br>PrintFormat("You are a %1 one", "bold", level: LogLevel.WARNING);<br>``` |
| Cast null check | ```enforce<br>SCR_Class casted = SCR_Class.Cast(instance);<br>Print(casted.m_Value);<br>// becomes<br>SCR_Class casted = SCR_Class.Cast(instance);<br>if (!casted)<br>return;<br>Print(casted.m_Value);<br>``` |
| Resource validity check | ```enforce<br>Resource resource = Resource.Load(resourceName);<br>BaseContainer baseContainer = resource.GetResource().ToBaseContainer();<br>// becomes<br>Resource resource = Resource.Load(resourceName);<br>if (!resource.IsValid())<br>return;<br>BaseContainer baseContainer = resource.GetResource().ToBaseContainer();<br>``` |

## Settings Overview

- Enable/Disable constructor/destructor/autoinstantiate/fix features
- Choose when instantiation assistance triggers (on "= new", "=" only or both)
- Configure default `Print`/`PrintFormat` log level
- Edit tool/user keyword pairs and attribute decorator presets
- All settings available via plugin configuration dialog.

## Customisation

- Custom keywords/snippets and attribute templates are supported via settings.
- Tool-provided (default) and user-defined lists are kept separate.

## See Also

- [Arma Reforger:Script Editor](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor)
- Script Editor Plugins
- [Arma Reforger:Script Editor: Fill From Template Plugin](https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Fill_From_Template_Plugin)
