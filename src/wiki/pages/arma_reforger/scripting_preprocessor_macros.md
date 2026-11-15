---
title: Arma Reforger:Scripting: Preprocessor Macros
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Preprocessor_Macros
---

Preprocessor macros provide helpful context information, especially useful in debug.

**Note:** See also [Arma Reforger:Scripting: Preprocessor Directives](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Preprocessor_Directives).

| Macro | Description | Example |
| --- | --- | --- |
| **__FILE__** | Is replaced by a string containing the current file's relative path. | `Print(__FILE__, LogLevel.NORMAL);`<br>`// ends as e.g`<br>`Print("scripts/WorkbenchGame/ScriptEditor/TAG_MyTestPlugin.c", LogLevel.NORMAL);`<br>`// this is absolutely valid`<br>`string absPath;`<br>`Workbench.GetAbsolutePath(__FILE__, absPath, true);`<br>`Print(absPath); // e.g "D:/MyMods/TAG_MyMod/scripts/WorkbenchGame/ScriptEditor/TAG_MyTestPlugin.c"` |
| **__LINE__** | Is replaced by a **string** containing the current file's line number. | `Print(__LINE__ + 2, LogLevel.NORMAL);`<br>`// ends as`<br>`Print("4" + 2, LogLevel.NORMAL); // "42"` |
