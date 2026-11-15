---
title: Language Server Protocol
url: https://community.bistudio.com/wikidata/external-data/arma-reforger/EnfusionScriptAPIPublic/Page_LSP.html
---

| Pages |
| --- |
| Features |
| Visual Studio Code configuration |
| Neovim configuration |

[Language Server Protocol](https://microsoft.github.io/language-server-protocol/) (LSP) implementation in the Enfusion engine give external text editors ability to write Enforce Script with additional features. Current implementation is using TCP connection between the LSP server (Script Editor) and LSP client (your text editor). Following are configuration examples of favorite text editors and IDEs.

- Visual Studio Code
- Neovim

> The Language Server Protocol (LSP) is an open, JSON-RPC-based protocol for use between source code editors or integrated development environments (IDEs) and servers that provide "language intelligence tools": programming language-specific features like code completion, syntax highlighting and marking of warnings and errors, as well as refactoring routines. The goal of the protocol is to allow programming language support to be implemented and distributed independently of any given editor or IDE. In the early 2020s, LSP quickly became a "norm" for language intelligence tools providers. – [Language Server Protocol @ Wikipedia](https://en.wikipedia.org/wiki/Language_Server_Protocol)

## Motivation

- We want to offer possibility to use rich code editing features to an external editors when programming Enforce. Script Editor remains the main way how we expect to people to write the code, but we want to support 3rd party editors too, giving our communities freedom of choice.
- We want to experiment with Large Language Models and AI tools which can use LSP to generate correct Enforce script. External editor are also offering many different AI related capabilities which we would need to implement in the Script Editor in order to test or utilize them.

It is important to note that the goal is not to implement all the features LSP offers. We are just experimenting with it and we will see how far and what direction this goes.

## How to use it

**Remarks:** This following steps will change in the future. Clients will be able to boot up the LSP server automatically, no manual actions of running Workbench, opening Script Editor will be needed. All will happen in a background.

**Remarks:** Currently the Script Editor can compile code only when it is saved on the drive, which means that it is performing write operations to the files you are editing using external editor. This may have some unpredictable consequences in the editors itself.

1. Enable LSP in the Workbench Options
   1. Client connects to an IP:PORT of the Language Server using TCP communication
   1. Some features are assuming that the server and client are sharing a filesystem
1. Start editing script files in your favorite text editor with configured LSP
   1. You can connect LSP only to files you have physically on your drive
   1. Scripts must be in valid placement (in correct folders inside your project you are editing scripts for)
