# Folders by Capability

Server code is grouped by Capability (one folder holds the logic, MCP tools, and tests for one job), not by kind of code (tools/, utils/, index/). In the old repo, understanding one feature meant opening four folders. With this layout, a Port adds one folder and removing a Capability deletes one folder.

Each Capability folder has a short README.md (what it does, which tools it gives Claude, where its data comes from), so GitHub shows the explanation when you open the folder. Tool names start with the first word of their folder name: `api-docs/` gives `api_search`, `game-files/` gives `game_find`.
