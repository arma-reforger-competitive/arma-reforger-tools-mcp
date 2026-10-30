# Commits

Commit messages follow [Conventional Commits 1.0.0](https://www.conventionalcommits.org/en/v1.0.0/). The plugin follows [Semantic Versioning 2.0.0](https://semver.org/spec/v2.0.0.html), and the commit type decides the next version bump. Below version 1.0.0, a breaking change bumps MINOR.

## Format

```
<type>(<scope>)!: <description>

<body>

<footers>
```

Only `<type>: <description>` is required. Example:

```
feat(game-files): suggest the closest real paths when a path is wrong
```

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
- One commit has one type. If a change fits two types, split it into two commits.

## Scope

The scope is the name of the place the change lives in:

- a Capability folder: `api-docs`, `wiki`, `game-files`, `workbench`
- a skill: `create-mod`, `port-status`
- `plugin` for the plugin manifest and server start-up

Leave the scope out when the change touches many places.

## Description

- Use the imperative: the description completes "Applying this commit will …".
- Start in lowercase, with no period at the end.
- Keep the whole first line at 72 characters or fewer.
- Say the effect, not the mechanism: `fix(wiki): find pages when the title has wrong capital letters`, not `fix(wiki): change string compare`.

## Body

Write the body for the **why**: the diff already shows the what. Add a body when the change reverses a decision, works around a bug in the game or Workbench, or looks wrong without context. Name the ADR when one applies. Wrap lines at 72 characters.

## Footers

- `Closes #N` when the commit finishes ticket N. `Refs #N` when it is one part of ticket N.
- `BREAKING CHANGE: <what the user must do>` when a tool, a tool input, a skill, or a setting is renamed or removed. Also add `!` before the colon.

## Writing the message

Use a heredoc, so the body and footers keep their line breaks:

```sh
git commit -F - <<'EOF'
feat(game-files): suggest the closest real paths when a path is wrong

Claude often guesses paths that do not exist. In the old project,
27 of 42 reads failed this way and Claude had to search again.

Closes #7
EOF
```

Fix a bad message only before you push it (`git commit --amend`). After the push, leave it.
