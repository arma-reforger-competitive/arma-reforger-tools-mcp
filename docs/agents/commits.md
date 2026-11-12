# Commits

The shared rules are in `~/www/home/docs/agents/commits.md`. This file adds what is different in this repo.

The plugin follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html), and the commit type decides the next version bump. Below version 1.0.0, a breaking change bumps MINOR.

## Types

| Type       | Use when                                                    | Bump  |
| ---------- | ----------------------------------------------------------- | ----- |
| `feat`     | A new tool, skill, or Capability that users can see         | MINOR |
| `fix`      | A bug that users could hit                                  | PATCH |
| `docs`     | README, `GLOSSARY.md`, ADRs, `docs/`, code comments         | none  |
| `refactor` | The code changes, the behaviour stays the same              | none  |
| `perf`     | Faster or smaller, the behaviour stays the same             | none  |
| `test`     | Tests only                                                  | none  |
| `build`    | `package.json`, `tsconfig`, build scripts, plugin packaging | none  |
| `ci`       | Workflows in `.github/`                                     | none  |
| `chore`    | Anything else, for example `.gitignore`                     | none  |
| `revert`   | Undoing an earlier commit                                   | none  |

- A plugin skill is behaviour that users get, so a new or changed skill is `feat` or `fix`, never `docs`.
- Updating the Knowledge base after a game update is `fix`: users got outdated answers.

## Scope

The scope is the name of the place the change lives in:

- a Capability folder: `api-docs`, `wiki`, `game-files`, `workbench`
- a skill: `create-mod`, `port-status`
- `plugin` for the plugin manifest and server start-up

## Breaking changes

A change is breaking when a tool, a tool input, a skill, or a setting is renamed or removed.

## Body

Add a body also when the change works around a bug in the game or Workbench.
