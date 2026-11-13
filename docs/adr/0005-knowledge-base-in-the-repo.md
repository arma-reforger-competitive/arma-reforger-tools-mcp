# Knowledge base in the repo

The Knowledge base is committed to the repo as source files, so the plugin works right after install and git shows what changed after each game update:

- Wiki pages: one Markdown file per page, with the title and URL in a short header. The path follows the wiki URL in lowercase, for example `src/wiki/pages/arma_reforger/server_config.md`.
- The raw wiki source: the MediaWiki export `src/wiki/source/export.xml`.
- Engine pages: the related pages of the engine API docs in the Workbench install, as Markdown files like the wiki pages.
- API classes: one JSONL file, one class per line. Fields that repeat other fields (`params` repeats `signature`) and empty fields are removed, which takes the data from 52 MB to about 21 MB.

The build puts these files into the server program. The server searches them with its own BM25 ranking over Sections, without a search library.

The import tool `cargo xtask import` rebuilds all files from the Workbench install and from `export.xml`. The wiki blocks scripts (it answers 403), so after a game update the owner saves `export.xml` from the wiki's Special:Export page by hand.

## Considered options

- **One big JSON file per data set.** A diff after a game update is hard to read, and nobody can open one page alone.
- **Tantivy or SQLite FTS5 for search.** Both are too big for about 9,000 records. SQLite also needs C code built for Windows, and its data file cannot be diffed.
- **Download the Knowledge base from a GitHub Release.** Still possible later. No Tool needs to change for that.
