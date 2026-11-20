# guid

Makes GUIDs for new mod projects. A mod project file (`.gproj`) has a GUID: the ID of the mod. Other mods name this ID in their dependencies, and the Workshop uses it as the mod's ID, so it must be unique among all mods. The create-mod skill calls this Tool.

## Tools

- `guid_generate`: returns one new GUID as plain text: 16 hex characters in upper case, for example `6156F2F771D5D73D`. It has no inputs.
  - It makes the GUID from 8 random bytes of the operating system (`getrandom`), so each call gives a different GUID.
  - When the operating system gives no random bytes, the answer is an error that says to call again.

  Tool labels: read-only, not destructive, not safe to call again (each call gives a new GUID), and not open-world.

Resources such as Prefabs also have GUIDs, in their `.meta` files. Workbench makes those itself, so this Tool does not.
