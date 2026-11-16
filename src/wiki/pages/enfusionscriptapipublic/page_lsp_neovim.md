---
title: Language Server Protocol: Neovim configuration
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_LSP_Neovim.html
---

It is assumed that you are already familiar with [Neovim](https://neovim.io/), have the editor installed (in a recent version) and you know basics about the editor configuration.

## Create configuration

1. Create *enforce-ls.lua* [LSP configuration](https://neovim.io/doc/user/lsp.html#lsp-config) file inside the "*$MYVIMRC/lsp*" directory and enter following code snipped into it. More details about the [*vim.lsp.Config*](https://neovim.io/doc/user/lsp.html#vim.lsp.Config) structure and [*vim.lsp.rpc.connect*](https://neovim.io/doc/user/lsp.html#vim.lsp.rpc.connect()) function. You can print value of the environment variable *$MYVIMRC* using following command *:lua =os.getenv("MYVIMRC")*. The configuration will attach client to *c* and *enforce* [file types](https://neovim.io/doc/user/filetype.html) and create TCP connection to the Language Server running at the specified port. Client LSP capabilities are provided in the structure too, including directories where it tries to decide on a [workspace root](https://neovim.io/doc/user/lsp.html#lsp-root_markers). There is also a commented out code which assigns *enforce* file type to all **.c* files. You need to run '*:filetype detect*' for already opened *c* files to update it. **enforce-ls.lua**
```
-- Change file type for all *.c files to C++ for better highlighting
-- You can remove this section when you use Enforce tree-sitter
vim.filetype.add({
    pattern = {
        [".*/scripts/.*/.*%.c"] = 'cpp'
    }
})
 
---@type vim.lsp.Config
return {
    cmd = vim.lsp.rpc.connect('127.0.0.1', 5776),
    capabilities = vim.lsp.protocol.make_client_capabilities(),
    filetypes = { 'c', 'cpp', 'enforce' },
    root_markers = { 'scripts' },
}
```
1. (optional) Start *enforce-ls* LSP client by entering following command into the *"$MYVIMRC/init.lua"* file. **init.lua**
```
vim.lsp.enable("enforce-ls")
```
1. (optional) It is recommended to tweak default Neovim *init.lua* file a little bit, you can use slightly modified Suckless NeoVim Config. If you feel more adventurous you can use a more advanced, but still very simple, [kickstart.nvim](https://github.com/nvim-lua/kickstart.nvim/blob/master/init.lua). Both works very well. The later is a bit nicer and feature-rich, but it is harder to understand (it is using and installing various plugins) and it also requires few dependencies to be installed on your PC so do not forgot to read the [readme](https://github.com/nvim-lua/kickstart.nvim/blob/master/README.md) file.

## Start LSP client

1. Start the client using following command, in case you do not have it enabled in the "*init.lua*" file.
```
:lua vim.lsp.enable("enforce-ls")
# In case you are using Neovim 0.12+ you can run following instead
:lsp enable enforce-ls
```
1. Check the LSP status by running
```
:checkhealth vim.lsp
```

## Enforce tree-sitter

This is an optional step where we define custom "enforce" file type in Neovim and use [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) for code highlighting and folding. Without this step, we used "cpp" filetype as Enforce is syntactically close. Of course many things are different between these two languages so the highlighting wasn't perfect. It can be improved by [enforce tree-sitter implementation](https://github.com/simonvic/tree-sitter-enforce). The installation requires few steps and has C/C++ compiler as a prerequisite. This step is also completely unrelated and independent to the LSP, in other words you do not need to have LSP in order for the tree-sitter to work. But these two can complement each other nicely.

1. Install the [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter), you can use do it by creating "*treesitter.lua*" inside "*$MYVIMRC/plugins*" directory. It is assumed you use Neovim 0.12.x, otherwise you will need to use different package manager or install the plugin manually. Also make sure you are compiling for correct architecture, on MS Windows it means using "x64 Native Tools Command Prompt" when compiling using Microsoft C/C++ compiler.
```
if vim.version().major >= 0 and vim.version().minor >= 12 then
    vim.pack.add({
        'https://github.com/nvim-treesitter/nvim-treesitter',
    }, {
        load = true
    })
 
    require('nvim-treesitter.configs').setup({
        ensure_installed = { "enforce" },
        highlight = {
            enable = true,
            additional_vim_regex_highlighting = false,
        },
        indent = {
            enable = true,
        }
    })
 
    vim.filetype.add({
     pattern = {
         [".*/scripts/.*/.*%.c"] = "enforce",
     },
    })
end
```
1. Run *:lua checkhealth nvim-treesitter* and make sure you have all its need to compile parsers and that the *enforce* parser was installed
1. Check *enforce* parser in *:lua checkhealth vim.treesitter*
1. Run *:TSModuleInfo* and check that *highlight* and *indent* module is enabled for *enforce*
1. Open Enforce script file and check correct highlighting and folding, make sure that the file type was properly set
   1. Make sure you removed file type settings from *enforce-ls.lua* file
1. Folding requires expression method, for example
```
vim.opt.foldmethod = "expr"                     -- Use expression for folding
vim.opt.foldexpr = "nvim_treesitter#foldexpr()" -- Use treesitter for folding
vim.opt.foldlevel = 4                           -- Start with all folds open
```

## Tips and Tricks

Assuming that you successfully configured the LSP in the Neovim.

1. It is [Diagnostic](https://neovim.io/doc/user/diagnostic.html) which is showing you information about state of your file in relation to the LSP.
   1. You can use *]d* / *[d* to jump to the [next](https://neovim.io/doc/user/tagsrch.html#%5Dd-default)/[previous](https://neovim.io/doc/user/tagsrch.html#%5Bd-default) diagnostic in your buffer.
   1. You can use *CTRL+W CTRL+D* to [show diagnostic information under your cursor](https://neovim.io/doc/user/diagnostic.html#vim.diagnostic.open_float()).
   1. You can use *grt* for [Go to Type Definition](https://neovim.io/doc/user/lsp.html#vim.lsp.buf.type_definition()).
   1. You can use *gri* for [Go to Implementation](https://neovim.io/doc/user/lsp.html#vim.lsp.buf.implementation()).
   1. You can use *CTRL+]* for [Go to Definition](https://neovim.io/doc/user/tagsrch.html#CTRL-%5D), see [tagfunc defaults](https://neovim.io/doc/user/lsp.html#_defaults).
   1. You can use *CTRL + T* to jump back, as all Go to functions are pushing tags into the [Tag stack](https://neovim.io/doc/user/tagsrch.html#_2.-tag-stack).
   1. Too see all the diagnostics even in the not opened files use the [Quickfix](https://neovim.io/doc/user/quickfix.html).
      1. Run '*:lua vim.diagnostics.setqflist()*' to create the Quickfix list or update it.
      1. You can use '*:copen*' to open the Quickfix list in case you close it (for example by '*:cclose*').
   1. You can provide you own [diagnostics configuration](https://neovim.io/doc/user/diagnostic.html#vim.diagnostic.config()) to change [properties](https://neovim.io/doc/user/diagnostic.html#vim.diagnostic.Opts) like highlights, signs, severity levels and many more. For example, you can place following code into you *init.lua* to display diagnostics as virtual text, change the [signs](https://neovim.io/doc/user/diagnostic.html#_signs) or update diagnostics in the Insert mode (they are updated on [InsertLeave](https://neovim.io/doc/user/autocmd.html#InsertLeave) by default). **init.lua**
```
vim.diagnostic.config({
  virtual_text = true,
  signs = {
    text = {
      [vim.diagnostic.severity.ERROR] = '✘',
      [vim.diagnostic.severity.WARN] = '▲',
      [vim.diagnostic.severity.HINT] = '⚑',
      [vim.diagnostic.severity.INFO] = '»',
    },
  },
  update_in_insert = true,
})
```
1. [Omni completion](https://neovim.io/doc/user/options.html#%27omnifunc%27) is invoke-able by *CTRL+X CTRL+O* (O as Omni) in Insert mode (some other editor calls this Intellisense, but that is a trademark)
   1. Make sure you that following command "*:set omnifunc?*" prints "*omnifunc=v:lua.vim.lsp.omnifun*" see [Why isn't completion working?](https://neovim.io/doc/user/lsp.html#lsp-faq)
   1. Otherwise run "*set omnifunc=v:lua.vim.lsp.omnifunc*".
1. [*K*](https://neovim.io/doc/user/various.html#K) in Normal mode can be used to show the "man" page (*:help find-manpage*) of the command below the cursor.
   1. In our case it should display information which is returned from the LSP using the [*vim.lsp.bufer.hover()*](https://neovim.io/doc/user/lsp.html#vim.lsp.buf.hover()) function.
   1. This feature is not implemented yet.
1. [*gq*](https://neovim.io/doc/user/change.html#gq) operator can be used to format the code
   1. *"formatexpr"* should be automatically set to [vim.lsp.formatexpr()](https://neovim.io/doc/user/lsp.html#vim.lsp.formatexpr()) function.
   1. This feature is not implemented yet.
1. To troubleshoot the LSP you need to increase the the log level in the [lsp-log](https://neovim.io/doc/user/lsp.html#lsp-log).
   1. Check you current log level in the health page using *":checkhealth vim.lsp"*
   1. Set TRACE log level by running *":lua vim.lsp.log.set_level('trace')"*
   1. Check the log file in the user data directory. The file Location can be be taken from *":lua =vim.lsp.log.get_filename()"*
1. To debug Neovim Lua scripts
   1. You need to install [nvim-dap](https://github.com/mfussenegger/nvim-dap) (Debug Adapter Protocol for Neovim) and [osv](https://github.com/jbyuki/one-small-step-for-vimkind) (Lua adapter for Neovim Lua lanaguage).
   1. Installation and configuration is available at the [osv readme file](https://github.com/jbyuki/one-small-step-for-vimkind). Check mainly the osv keymaps, quickstart as well as nvim-dap usage.
   1. Optionally you can install UI extensions like [nvim-dap-ui](https://github.com/rcarriga/nvim-dap-ui) or [nvim-dap-view](https://github.com/igorlfs/nvim-dap-view).
1. [Quick reference](https://neovim.io/doc/user/quickref.html) is very useful page when you want to find keybinding to some Neovim action.

### Cheat Sheet - 100 Neovim Commands for Beginners

#### Basic Navigation

- h, j, k, l - Move cursor left, down, up, right (the core movement keys in Neovim)
- w - Jump to start of next word (punctuation considered as words)
- W - Jump to start of next WORD (space-separated words)
- b - Jump to start of previous word
- B - Jump to start of previous WORD
- e - Jump to end of word
- E - Jump to end of WORD
- 0 - Jump to start of line (first column)
- ^ - Jump to first non-blank character of line
- $ - Jump to end of line
- gg - Go to first line of document
- G - Go to last line of document
- {number}G - Go to specific line number
- { - Jump to previous paragraph/code block
- } - Jump to next paragraph/code block
- Ctrl-u - Move up half a screen
- Ctrl-d - Move down half a screen
- Ctrl-b - Move up one full screen
- Ctrl-f - Move down one full screen
- zz - Center cursor on screen (current line becomes middle line)
- zt - Position cursor at top of screen
- zb - Position cursor at bottom of screen

#### Editing Commands

- i - Enter insert mode before cursor (for inserting text)
- I - Enter insert mode at beginning of line
- a - Enter insert mode after cursor (append)
- A - Enter insert mode at end of line
- o - Insert new line below current line and enter insert mode
- O - Insert new line above current line and enter insert mode
- r - Replace a single character under cursor (without entering insert mode)
- R - Enter replace mode (overwriting existing text)
- x - Delete character under cursor
- X - Delete character before cursor
- dd - Delete entire line (and store in register)
- {number}dd - Delete multiple lines
- D - Delete from cursor to end of line
- yy or Y - Yank (copy) entire line
- {number}yy - Yank multiple lines
- y$ - Yank from cursor to end of line
- p - Paste after cursor
- P - Paste before cursor
- u - Undo last change
- Ctrl-r - Redo (undo the undo)
- ~ - Switch case of character under cursor
- >> - Indent line
- << - Unindent line
- . - Repeat last command (powerful for repetitive edits)
- cc or C - Change entire line (delete line and enter insert mode)
- cw - Change word (delete word and enter insert mode)
- c$ or C - Change to end of line
- J - Join current line with the next line

#### Search and Replace

- /pattern - Search forward for pattern
- ?pattern - Search backward for pattern
- n - Repeat search in same direction
- N - Repeat search in opposite direction
- * - Search forward for word under cursor
- # - Search backward for word under cursor
- :s/old/new/g - Replace all occurrences of 'old' with 'new' throughout file
- :s/old/new/gc - Replace all occurrences with confirmations
- :s/old/new/g - Replace all occurrences on current line
- :noh - Clear search highlighting
- gd - Go to local definition of word under cursor
- gD - Go to global definition of word under cursor

#### Visual Mode

- v - Enter character-wise visual mode (select characters)
- V - Enter line-wise visual mode (select entire lines)
- Ctrl-v - Enter block-wise visual mode (select rectangular blocks)
- gv - Reselect previous visual selection
- o - In visual mode: Move to other end of selection
- O - In visual block mode: Move to other corner of block
- aw - Select a word (in visual mode)
- ab - Select a block with () (in visual mode)
- aB - Select a block with {} (in visual mode)
- at - Select a block with HTML/XML tags (in visual mode)

#### File Operations

- :e filename - Edit a file (create if doesn't exist)
- :w - Write (save) the file
- :w filename - Write to specified filename (save as)
- :q - Quit (fails if unsaved changes)
- :q! - Quit without saving (discard changes)
- :wq or :x - Write and quit
- :saveas filename - Save file as filename
- :r filename - Insert contents of file below cursor
- :r !command - Insert output of shell command below cursor

#### Working with Windows and Tabs

- :split or :sp - Split window horizontally
- :vsplit or :vs - Split window vertically
- Ctrl-w h/j/k/l - Navigate between windows (left/down/up/right)
- Ctrl-w +/- - Increase/decrease window height
- Ctrl-w </> - Increase/decrease window width
- Ctrl-w = - Make all windows equal size
- Ctrl-w o - Make current window the only one
- :tabnew - Create new tab
- gt - Go to next tab
- gT - Go to previous tab
- :tabclose - Close current tab
- :tabonly - Close all other tabs

#### Buffer Management

- :ls - List all buffers
- :b number - Switch to buffer by number
- :bn - Next buffer
- :bp - Previous buffer
- :bd - Delete buffer (close file)
- :bufdo command - Execute command on all buffers
- :e # - Edit the alternate file (usually the previously edited file)

#### Marks and Jumps

- m{a-z} - Set mark at current position (lowercase for file-local)
- m{A-Z} - Set mark at current position (uppercase for global)
- '{mark} - Jump to line of mark
- `{mark} - Jump to position of mark
- Ctrl-o - Jump to older position in jump list
- Ctrl-i - Jump to newer position in jump list
- '. - Jump to position of last change
- `. - Jump to exact position of last change

#### Text Objects and Motions

- ci( - Change inside parentheses
- di&quot; - Delete inside double quotes
- yi] - Yank inside square brackets
- va{ - Visually select around curly braces (including the braces)
- dap - Delete around paragraph
- cit - Change inside HTML/XML tag
- diw - Delete inside word
- daw - Delete around word (including spaces)
- dab - Delete around block (parentheses)
- daB - Delete around block (curly braces)

#### Fold Commands

- zf - Create fold (in visual mode)
- zo - Open fold under cursor
- zc - Close fold under cursor
- za - Toggle fold under cursor
- zR - Open all folds
- zM - Close all folds
- zj - Move to next fold
- zk - Move to previous fold

#### Neovim-Specific Features

- :terminal or :term - Open integrated terminal
- Ctrl-\ Ctrl-n - Exit terminal mode to normal mode
- :checkhealth - Run Neovim's diagnostic tool
- :highlight - Show current highlight groups
- :Tutor - Start Neovim's built-in tutorial
- :help nvim-features - View Neovim's specific features

#### Advanced Features

- q{a-z} - Record macro into register
- @{a-z} - Play macro from register
- @@ - Repeat last played macro
- g& - Repeat last substitution on all lines
- :norm cmd - Execute normal mode command on selected lines
- gf - Go to file under cursor
- Ctrl-a - Increment number under cursor
- Ctrl-x - Decrement number under cursor
- :sort - Sort selected lines
- !motion command - Filter text through external command

1. Useful articles and videos about the Neovim and LSP configuration
   1. [The Only Video You Need to Get Started with Neovim](https://www.youtube.com/watch?v=m8C0Cq9Uv9o)
   1. [Configuring Neovim 0.11 LSP from scratch](https://blog.diovani.com/technology/2025/06/13/configuring-neovim-011-lsp.html)
   1. [A guide on Neovim's LSP client](https://vonheikemen.github.io/devlog/tools/neovim-lsp-client-guide/)
