# Arma Reforger Tools MCP

> **Work in progress.** The plugin installs and has one tool, `wiki_read`. Follow the [roadmap](https://github.com/arma-reforger-competitive/arma-reforger-tools-mcp/issues/1) to see what is done and what comes next.

`reforger` is a Claude Code plugin that helps you learn, build, and fix Arma Reforger mods. It gives Claude two kinds of help:

- **MCP tools** for jobs that need code: searching the Enfusion script API and the Reforger wiki, and finding and reading the game's own files.
- **Skills** with Reforger know-how: steps, rules, examples, and templates for modding tasks.

This project rebuilds the older [enfusion-mcp](https://github.com/arma-reforger-competitive/arma-enfusion-mcp) step by step, one part at a time.

## Install

You need [Node.js](https://nodejs.org/) 24 or later and Claude Code.

```sh
git clone https://github.com/arma-reforger-competitive/arma-reforger-tools-mcp.git
cd arma-reforger-tools-mcp
npm install
claude plugin marketplace add ./
claude plugin install reforger@reforger-local
```

Claude Code runs the plugin from this folder, so keep the folder after the install. To check the install, run `claude mcp list`. The line `plugin:reforger:reforger` must show `Connected`.

## Tools

| Tool        | What it does                                      |
| ----------- | ------------------------------------------------- |
| `wiki_read` | Returns one Arma Reforger wiki page by its title. |

## Development

```sh
npm run check   # check the format, find code problems, and check the types
npm test        # run all tests
```

Both commands use [Vite+](https://viteplus.dev) (`vp`). To fix the format, run `npx vp check --fix`.

The server runs the TypeScript files in `src/` directly with Node, so there is no compile step. Each Capability has its own folder in `src/`.

## Learn more

- [`GLOSSARY.md`](GLOSSARY.md): the words this project uses
- [`docs/adr/`](docs/adr/): the design decisions and why we made them
- [`docs/build-log.md`](docs/build-log.md): what each Port changed

## Disclaimer

Unofficial. Not affiliated with Bohemia Interactive. Arma Reforger is a trademark of Bohemia Interactive a.s.
