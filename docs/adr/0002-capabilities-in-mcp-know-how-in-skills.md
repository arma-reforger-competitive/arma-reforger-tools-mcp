# Capabilities in MCP tools, know-how in skills

Something becomes an MCP tool only if it needs running code: a live connection (Workbench), fast search over large data (Knowledge base), binary files (.pak), or output that must be exactly right (new GUIDs, Enfusion text format, checks against the API). Everything an expert could do with a text editor and the docs is a skill: steps, rules, examples, and text-only template files that Claude copies and edits.

## Considered options

- **Everything as MCP tools** (the old design). It was rejected because none of the 12 old "create" tools were ever used. Claude wrote the files itself. What it lacked was knowledge, not file generation.
- **Skills and scripts only, no MCP.** It was rejected because it loses live Workbench control and fast search.
