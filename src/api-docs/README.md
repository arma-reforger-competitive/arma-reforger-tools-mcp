# api-docs

Searches the Enfusion engine and Arma Reforger script API classes from the Knowledge base.

## Tools

- `api_search`: finds classes and methods by the full name or a part of the name, for example `SCR_BaseGameMode`, `GetFocusFOVScale`, or `CharacterCamera`. Capital letters do not matter. Give more names with spaces between them, for example `PlayerLimit MaxPlayers`.
  - A full class name gives the class page: the source and group, the classes it inherits from (`CameraHandlerComponent > ScriptComponent > GenericComponent`), up to 10 subclasses, the brief and description, all methods with their signatures, the properties, the source file, and the docs URL.
  - Other matches come as a list: each class with its parent and brief, each method as `Class.Method` with its signature and the first paragraph of its description. A full name comes first, then names that start with the query, then names that have it inside. Classes come before methods, and shorter names before longer ones.
  - `limit` sets how many results the list shows, from 1 to 50 (default 10). A longer list is cut, and the first line says how many match and how to narrow the search.
  - `type` searches only classes (`"class"`) or only methods (`"method"`).
  - When no name has the query in it, the list shows the names that differ by at most 2 letters, for example `ScriptComponent` for `ScriptCompnent`. When none is that close, the answer says what to try next.
  - An empty query gives an error with an example.

## Data

`classes.jsonl` holds one class per line, 9,119 classes in total:

- 963 engine classes (`"source": "enfusion"`) from `EnfusionScriptAPI/html` in the Workbench docs of Arma Reforger Tools.
- 8,156 game classes (`"source": "arma"`) from `ArmaReforgerScriptAPIPublic/html`.

Each class has its name, its brief and description, its parent and children, its group, its source file, its methods, and its properties, and the URL of its page on the wiki server. A method has no list of parameters: they are all in its `signature`. Empty fields are left out.

The build puts the file into the server program, and the server reads every line when it starts. A line that is not a valid class stops the server with the line number.

`cargo xtask import` rebuilds the file. See `src/wiki/README.md` for the steps.
