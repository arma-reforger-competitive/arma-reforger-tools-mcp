---
title: Language Server Protocol: Visual Studio Code configuration
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_LSP_VSCode.html
---

## Installing an extension

**Remarks:** The extension is WIP. It will get proper name, metadata and distribution channel later.

1. From VS Code

| Action | Detail |
| --- | --- |
| Go to the Extensions view. |  |
| Select **Views and More Actions...** |  |
| Select **Install from VSIX...** |  |

1. From Command Line Run following command code --install-extension my-extension-0.0.1.vsix

## Enabled LSP Client

- Open file from your project
- Check selected language in down-right panel
  - If it doesn't compare with 'Enforce', manually select language.
- You can manually reload the client extension
  - CTRL + SHIFT + P → 'LSP Client restart'
  - Or by default Shortcut CRTL + ALT + D. [See this link, if you'll want change it](https://code.visualstudio.com/docs/configure/keybindings)

## Troubleshooting

### Incorrect port number

| Action | Detail |
| --- | --- |
| Go to extensions -> Click on gear behind 'lsp-enforcescript-extension' -> Go to Settings |  |
| Double check port number in extension and in Workbench options. |  |
