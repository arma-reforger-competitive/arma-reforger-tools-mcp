# reference-mods

Keeps the Reference mods in the Workbench `addons` folder. A Reference mod is a Workshop mod that the agent reads to learn how a feature works. The owner lists it in a mod repo, and it is never stored in git.

## Tools

- `reference_sync`: makes the `addons` folder of the Workbench folder in Documents match the lists. It has no inputs. It:
  - deletes each mod that is in the record and on no list, then removes it from the record. It needs no Workbench for this.
  - downloads each listed mod that is not on disk, through the Workbench bridge. For each one it asks the Handler script to start the download, then asks for its progress about every 2 seconds, up to 30 seconds in total. Then it answers. A second call starts no download twice: it asks for the progress of the downloads that the server started before. When Workbench does not answer, each missing mod is a failed line with the message of the bridge, for example "Workbench is not running". An ID that the Workshop does not know is a failed line, and the other mods continue.
  - adds a mod to the record only when its download is complete and its folder `<Name>_<ID>` is in `addons`.
  - never touches anything else in `addons`: projects, links, `saves`, and mods that the owner downloaded by hand stay. A listed mod that is on disk counts as present, also when it is not in the record.

  The answer has one line for each Reference mod and each deleted mod: name, ID, license name, and state. Example: `tvtLobby (6AA81EE549F50CB7), license MIT: present`. The license name comes from the record, so it shows without Workbench. A mod that is not in the record has license `unknown`.

  A setting that is missing or wrong, or a list line that is not valid, stops the call before any delete. The answer is an error that names the setting, or the file and the line.

  Tool labels: not read-only, destructive (it deletes folders), safe to call again, and open-world (it downloads from the Workshop).

## The list

The file `reference-mods.jsonl` in the root of a mod repo. One line is one Reference mod:

```json
{"id":"6AA81EE549F50CB7","name":"tvtLobby"}
```

- `id`: the Workshop ID, 16 hex digits. Capital letters do not matter.
- `name`: the mod name. It is not empty.
- No other fields. An empty line is skipped, so an empty file is valid.
- A mod repo with no list is skipped. A mod on two lists is one Reference mod. It stays until both lists drop it.

## The record

The Tool keeps its own record in `<Workbench Documents folder>/addons/.reference-mods.jsonl`. The dot keeps it from looking like a mod folder. One line for each mod that the Tool downloaded:

```json
{"id":"6AA81EE549F50CB7","folder":"tvtLobby_6AA81EE549F50CB7","version":"1.0.0","license":"MIT"}
```

The Tool deletes only folders that the record names. It writes the record through a temp file and a rename, so a stop in the middle never leaves half a file.

## Settings

Set them in the `env` of Claude Code's `settings.json`. A path works in Windows form and in WSL form. Restart Claude Code after a change.

- `REFORGER_MODS_PATH`: the folder of the mod repos, for example `D:\Mods`. The Tool reads `reference-mods.jsonl` in each direct subfolder.
- `REFORGER_WORKBENCH_DOCUMENTS_PATH`: the Workbench folder in Documents. Its `addons` folder is where the Tool downloads, deletes, and keeps the record. Without it, the Tool finds the folder the same way as the `workbench` bridge.
- `REFORGER_WORKBENCH_HOST` and `REFORGER_WORKBENCH_PORT`: the Workbench NET API, for the downloads (see `workbench`).

## How it works

`plan.rs` holds the pure logic: the parse of a list line and of the record, the plan, and the text of the answer. The plan takes the lists, the record, and the folder names in `addons`. It gives three groups: delete (in the record and on no list), download (listed and not on disk), and present (listed and on disk). A mod is on disk when `addons` has a folder `<Name>_<ID>` with a name that is not empty (`src/mod_folder.rs` parses the name). `sync.rs` is the thin outer layer: it reads the files, deletes the folders, writes the record, and calls the bridge. The server keeps in memory the downloads that it started and that have not ended. For such a mod, a later call asks only for the progress, also when its folder is on disk already.

## The Handler actions

`reference_sync` calls two actions of the Handler script, each with the parameter `id`:

- `reference_download`: starts the download of the mod `id`. When a download of that ID already runs, it starts nothing.
- `reference_progress`: answers `state` (`searching`, `downloading`, `complete`, or `failed`), and where they apply: `percent` (a whole number from 0 to 100), `name`, `mod_version`, `license`, and `reason`. The mod version is in `mod_version`, because the bridge uses `version` for the version of the Handler script.
