# wiki

Reads pages of the Arma Reforger wiki and the Enfusion engine pages from the Knowledge base.

## Tools

- `wiki_search`: finds pages by words, for example `server config` or `faction spawn points`. Each result gives the page title, the path of its best Section (for example `game > mods`), and a short text from it. Each page shows once. `limit` sets how many pages come back, from 1 to 10 (default 5). If no page matches, the answer says what to try next.
- `wiki_read`: returns the full text of one page by its title, for example `Arma Reforger:Mod Project Setup` or `Event System`. Capital letters do not matter, and underscores from a wiki URL count as spaces. If no page has the title, the answer lists the 3 closest real titles. An empty title gives an error with an example title. Pages longer than 100,000 characters are cut.
  - `section` returns only one Section: a heading name (`mods`) or a path (`game > password`). When a heading name ends more than one path, the answer is an error that lists the full paths. Paths that differ only in capital letters count as one path. When no Section has the name, the error lists all Sections of the page.

## Search

A Section is one heading with all the text and smaller headings under it (see `GLOSSARY.md`). `sections.rs` splits each page into Sections when the server starts.

`search.rs` ranks pages with BM25, written here with no search library (ADR 0005):

- Each Section is indexed with its heading path and its own text, without link URLs. A heading with no text of its own is left out: the paths of its smaller headings hold its name.
- Each page title is indexed once as its own text. A page scores its title plus its best Section, so the best Section is the one whose own text matches best.
- Words are whole words in lowercase. A code name stays one word, and its parts are words too: `missionHeader` also gives `mission` and `header`. There is no stemming.

## Data

`pages/` holds one Markdown file per page. A short header holds the title and the URL:

```markdown
---
title: Arma Reforger:Server Config
url: https://community.bistudio.com/wiki/Arma_Reforger:Server_Config
---
```

The path follows the URL in lowercase, for example `pages/arma_reforger/server_config.md`. The build puts every file into the server program (see `build.rs`), and the server checks the headers when it starts.

The pages come from two sources:

- **252 wiki pages**: every page in the Arma Reforger namespace of the [BI Community Wiki](https://community.bistudio.com/wiki/Category:Arma_Reforger), from the wiki export `source/export.xml`. A page with no text left after the conversion, such as an empty placeholder, is left out.
- **24 engine pages**: the pages that `EnfusionScriptAPI/html/pages.html` lists in the Workbench docs of Arma Reforger Tools. Each one has the same file on the wiki server as its URL. A page under another page gets the other page's title in front, for example `Replication: RplNode`.

## Update after a game update

Do not edit the files in `pages/` by hand. Rebuild them:

1. Set `REFORGER_WORKBENCH_DOCS` to the docs folder of Arma Reforger Tools, for example `/mnt/d/Games/steamapps/common/Arma Reforger Tools/Workbench/docs`.
2. Renew `source/export.xml`. The wiki answers 403 to scripts, so save it in a browser: `cargo xtask import --export-list` prints the steps and the page list.
3. Run `cargo xtask import`. It rewrites `pages/` and `../api-docs/classes.jsonl`, and changes only the files whose text changed. A second run changes no file.
4. Run `cargo test`. `tests/knowledge_base.rs` holds the page and class counts: change them when the game added or removed pages.

The converter is in `xtask/src/wikitext/`. A template that it does not know stops the import with the template name, so add the new template to `render` in `templates.rs`.
