---
title: Language Server Protocol: Features
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_LSP_Features.html
---

**Remarks:** List of currently supported features and their description.

| Feature | Description |
| --- | --- |
| Open/Close files | Possibility to manipulate with .c files. |
| Save files | LSP provides file saving, not a client. |
| Changes in file | Changes being made in the editor are synchronized with the Script Editor. Mainly for debugging purposes. |
| Diagnostic | Client gets information about errors and warnings in working project. For correct diagnostic, it's necessary to save all *.c files. After saving, client receives complete diagnostic information. Without saving, client receives diagnostics about Syntax And Lexical errors during script validation. |
| Word completion | Word completion isn't only about completion keywords and basic types. It also gets info from classes, structures, enumerations, and etc. for more effective code writing. |
| Signature Help for methods | The signature help request is sent from the client to the server to request signature information at a given cursor position. |
| Hover | The hover request is sent from the client to the server to request hover information at a given cursor position. The server returns symbol information including its signature and Doxygen documentation comment. Response content format (plaintext or markdown) is negotiated based on client capabilities. |
| Signature Help | The signature help request provides parameter hints when calling a method or function. The server returns a list of matching overloads with their parameter names, and highlights the active parameter based on cursor position. |

## Goto methods

Goto Type Definition – Client receives information about location of type of the found symbol.

Goto Definition – Client receives information about location of the found symbol.

### Go to Table

This table shows what a client will receive from LSP if it selects action.

Legend: (X) Do nothing

|  | Class | Method | Function | Variable | Const Variable |
| --- | --- | --- | --- | --- | --- |
| Type Definition | Go to class definition | (X) | (X) | Go to class definition of selected symbol | **Inside Class or Method:** Go to class definition of selected symbol. **Inside Function:** (X) |
| Definition | Go to class definition | Go to method definition | Go to function definition | Go to variable definition | **Inside Class or Method:** Go to class definition of selected symbol. **Inside Function:** (X) |
| Implementation | Go to class definition, if implementation of selected class is missing; Go to class implementation; Client receives list of implementations of selected class, if there is more than 1 implementation | Go to method definition, if override of selected method is missing; Go to method override; Client receives list of overridden methods of selected method, if there more more than 1 override | Go to function definition | Go to variable definition | **Inside Class or Method:** Go to class definition of selected symbol. **Inside Function:** (X) |
