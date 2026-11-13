# Folders by Capability

Server code is grouped by Capability (one folder holds the logic, MCP tools, and tests for one job), not by kind of code (tools/, utils/, index/). In the old repo, understanding one feature meant opening four folders. With this layout, a Port adds one folder and removing a Capability deletes one folder.

Each Capability folder has a short README.md (what it does, which tools it gives Claude, where its data comes from), so GitHub shows the explanation when you open the folder. Tool names start with the first word of their folder name: `api-docs/` gives `api_search`, `game-files/` gives `game_find`.

These folders are the plugin architecture of the server. A Capability is added or removed when the server is built, with one line in `server.rs`. Nothing is loaded while the server runs, and there is no setting to turn a Capability on or off. Each Capability folder makes only one item public: the function that gives its Tools to `server.rs`. Everything else stays private, so the compiler stops one Capability from using the inside of another. Code that two or more Capabilities need goes in its own folder that gives no Tools, for example the .pak reader.
