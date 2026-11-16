---
title: Arma Reforger:Scripting: Preprocessor Directives
url: https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Preprocessor_Directives
---

Preprocessor directives allow to determine preprocessor behaviour, e.g ignoring blocks of code depending on certain conditions.

For use cases, see e.g [Scripting Temporary Feature](https://community.bistudio.com/wiki/Arma_Reforger:Scripting_Temporary_Feature).

**Note:** See also [Arma Reforger:Scripting: Preprocessor Macros](https://community.bistudio.com/wiki/Arma_Reforger:Scripting:_Preprocessor_Macros).

| Directive | Description | Example |
| --- | --- | --- |
| **#define** | Define a flag. A flag is either defined or not.<br>**Note:** A flag can be determined outside of the code; see [Startup Parameters - scrDefine](https://community.bistudio.com/wiki/Arma_Reforger:Startup_Parameters#scrDefine). | `#define MY_FLAG` |
| **#ifdef** | Open a preprocessor scope that is considered if the provided flag is defined. The scope must be ended by `#endif` (see below). | `#define MY_FLAG`<br>`#ifdef MY_FLAG`<br>`Print("Flag is defined");`<br>`#endif` |
| **#ifndef** | Open a preprocessor scope that is considered if the provided flag is **not** defined. The scope must be ended by `#endif` (see below). | `#define MY_FLAG`<br>`#ifndef MY_FLAG`<br>`Print("Flag is not defined");`<br>`#endif` |
| **#else** | Add a preprocessor scope that is of the opposite condition of the current `#ifdef`/`#ifndef`. The scope must be ended by `#endif` (see below). | `#ifdef MY_FLAG`<br>`Print("Flag is defined");`<br>`#else`<br>`Print("Flag is not defined");`<br>`#endif` |
| **#endif** | Close a preprocessor scope - see #ifdef and #ifndef above. | See `#ifdef` and `#ifndef` above. |
| **#include** | Include another file. The effect is as if the other file's content was copy-pasted at this exact `#include` location. | `// FileToInclude.c`<br>`protected static const string MY_PRINT = "Hello there";`<br>`class SCR_ScriptedClass`<br>`{`<br>`#include "scripts/Game/FileToInclude.c"`<br>`void ShowMessage()`<br>`{`<br>`Print(MY_PRINT);`<br>`}`<br>`}` |
