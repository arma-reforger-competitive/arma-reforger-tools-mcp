---
title: Arma Reforger:Script Editor: Basic Code Formatter Plugin
url: https://community.bistudio.com/wiki/Arma_Reforger:Script_Editor:_Basic_Code_Formatter_Plugin
---

- **name**: Basic Code Formatter
- **editor**: script
- **descr**: A plugin to help formatting code and following [conventions](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Conventions) and [good practices](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Best_Practices)
- **shortcut**: * [Ctrl+Shift+K]
- [Ctrl+Alt+Shift+K] (forced)
- **file**: `enfusion://ScriptEditor/scripts/WorkbenchGame/ScriptEditor/SCR_BasicCodeFormatterPlugin.c`
**Basic Code Formatter** is a plugin that helps formatting code to Bohemia Interactive standards as well as warns for bad practice.

It features:

- General space formatting
- Line end trimming
- Indentation fix from spaces to tabs
- Method separator fix
- Auto line end at the end of the file
- Scripting prefix check
- Batch processing (all addon scripts at once)
- The option to only formats modified lines (if using SVN/Git)
- Bad practice warnings
- A demo mode to practice in read-only.

## Usage

It is triggered by [Ctrl+Shift+K]; depending on the selected options, it will either process the current file or the selected addon's files.

**Note:** [Ctrl+Alt+Shift+K] can be used to force formatting of all the current file's lines.

## Features

### Modification

It can:
- trim line ends (trailing spaces and tabs)
- fix indentation (turn four spaces into a tab, remove extra spaces)
- auto-format method separators - all it takes is `//---` three dashes
- do general formatting:
  - remove semicolons `;` after a class-closing bracket
  - `if(` / `while(` / `foreach(`, etc
  - double spaces, double semicolons, capital NULL, etc<br>
**Note:** See `SCR_BasicCodeFormatterPlugin.GeneralFormatting()`.

- add a final line return to the file

**Note:** The plugin does not format comment or string content.

### Warnings

It warns about:
- wrong syntax (e.g `new ref`)
- improvable syntax (e.g `array<string> strings = new array<string>();` can be reduced to `array<string> strings = {};`)
- multiple empty lines
- usage of the `auto` and `autoptr` keywords
- divisions that could be multiplications (e.g `value / 2` is `value * 0.5`), as multiplications are usually cheaper in term of CPU
- unbracketed loops (`if`, `for`, `foreach`, `while`)
- `if` one-liners - always put the "then" part on a new line
- badly named variables and constants (e.g `string m_iValue` where the prefix for string is s - see [Values - String](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Values#String))
- `ScriptInvoker` direct usage (see [Arma Reforger:ScriptInvoker Usage](https://community.bistudio.com/wiki/Arma_Reforger:ScriptInvoker_Usage))
- a forgotten `Print()` call (a Print/PrintFormat without log level is considered a temporary debug one)
- non-tag-prefixed classes/enums (e.g `EMyEnum` vs `TAG_EMyEnum`)
- *some* spelling mistakes in comments (e.g "solider", "overriden", "dammage"...)

## Example

| Before | After |
| --- | --- |
| `class abc:Managed				// must warn for non-prefixed class + must space around ':'`<br>`{`<br>`[Attribute()];				// must remove the semicolon`<br>`protected ScriptInvokerVoid Event_OnSomething;`<br>`protected static string m_sValue;`<br>`// must fix the separator (below)`<br>`//---`<br>`protected void Method()`<br>`{							// must replace 5 spaces by one tab`<br>`if(true ) return;		// must reformat the if and warn about one-liner`<br>`// must warn about two empty lines (below)`<br>`// must remove trailing empty space (below)`<br>`}`<br>`// must remove the final semicolon below`<br>`};`<br>`// must add a line return (below)` | `class abc : Managed				// must warn for non-prefixed class + must space around ':'`<br>`{`<br>`[Attribute()]				// must remove the semicolon`<br>`protected ScriptInvokerVoid Event_OnSomething;`<br>`protected static string m_sValue;`<br>`// must fix the separator (below)`<br>`//------------------------------------------------------------------------------------------------`<br>`protected void Method()`<br>`{							// must replace 5 spaces by one tab`<br>`if (true) return;		// must reformat the if and warn about one-liner`<br>`// must warn about two empty lines (below)`<br>`// must remove trailing empty space (below)`<br>`}`<br>`// must remove the final semicolon below`<br>`}`<br>`// must add a line return (below)` |
| **Changelog** |  |
| `SCRIPT       : - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -`<br>`SCRIPT       : SCR_BasicCodeFormatterPluginExample.c - 7 lines changed, 1× line trimming, 1× indent fix(es) at line(s) 11, 5 formattings, 1 end spaces trimming, 1 4-spaces indent -> tabs replaced, 1 space(s) in indentation removed, 1 method separators fixed, added final newline (read: 3 ms, format: 83 ms, diff: 81 ms - total: 167 ms)`<br>`SCRIPT       : Checking all 20 lines (100% of the file)`<br>`SCRIPT       : 1× multiple consecutive empty lines found at line(s) 14 - leave only one`<br>`SCRIPT       : 2× badly-named variables found at line(s) 5-6 - use proper prefixes: m_s for ResourceName/string, m_v for vectors, NO m_p, CASED_CONSTS, etc`<br>`SCRIPT       : 1× non-prefixed class/enum found at line(s) 1 - classes and enums should be prefixed; see the settings to setup accepted prefixes (current SCR_)` |  |
