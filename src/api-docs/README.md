# api-docs

The Enfusion and Arma Reforger script API classes, for the `api_search` Tool that comes later.

## Data

`classes.jsonl` holds one class per line, 9,119 classes in total:

- 963 engine classes (`"source": "enfusion"`) from `EnfusionScriptAPI/html` in the Workbench docs of Arma Reforger Tools.
- 8,156 game classes (`"source": "arma"`) from `ArmaReforgerScriptAPIPublic/html`.

Each class has its name, its brief and description, its parent and children, its group, its source file, its methods, properties, and enums, and the URL of its page on the wiki server. A method has no list of parameters: they are all in its `signature`. Empty fields are left out.

`cargo xtask import` rebuilds the file. See `src/wiki/README.md` for the steps.
