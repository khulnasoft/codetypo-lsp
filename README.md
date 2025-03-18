# codetypo-lsp

[![ci](https://github.com/khulnasoft/codetypo-lsp/actions/workflows/ci.yml/badge.svg?event=push)](https://github.com/khulnasoft/codetypo-lsp/actions/workflows/ci.yml)
[![release](https://github.com/khulnasoft/codetypo-lsp/actions/workflows/release.yml/badge.svg?event=release)](https://github.com/khulnasoft/codetypo-lsp/actions/workflows/release.yml)
![downloads](https://img.shields.io/github/downloads/khulnasoft/codetypo-lsp/total)

> **Source code spell checker for Visual Studio Code and LSP clients**

[codetypo](https://github.com/khulnasoft/codetypo) is a low false-positive source code spell checker. This project exposes `codetypo` via a Language Server Protocol (LSP) server and Visual Studio Code extension to provide a fast, low memory, in-editor spell checker.

## Install

- Vscode: Install [Codetypo spell checker](https://marketplace.visualstudio.com/items?itemName=khulnasoft.codetypo-vscode) from the VSCode Marketplace.
- VSCodium: Install [Codetypo spell checker](https://open-vsx.org/extension/khulnasoft/codetypo-vscode) from the Open VSX Registry.
- Neovim: Install `codetypo-lsp` using [mason](https://mason-registry.dev/registry/list#codetypo-lsp) or download `codetypo-lsp` from the [releases page](https://github.com/khulnasoft/codetypo-lsp/releases).
- Helix: Download `codetypo-lsp` from the [releases page](https://github.com/khulnasoft/codetypo-lsp/releases) and place it on your PATH.
- Zed: Install [Codetypo](https://zed.dev/extensions?query=codetypo) from Zed's extension marketplace.
- Other clients: Download `codetypo-lsp` from the [releases page](https://github.com/khulnasoft/codetypo-lsp/releases).

For configuration see:

- [VS Code Settings](docs/vscode-settings.md)
- [Neovim LSP config](docs/neovim-lsp-config.md)
- [Helix config](docs/helix-config.md)
- [Zed config](docs/zed-config.md)

## Features

<!-- markdownlint-disable-file MD033 -->

- Identify misspellings and provide a Quick Fix with suggested corrections:

    <img width="373" alt="Diagnostics example with Quick Fix" src="https://user-images.githubusercontent.com/125105/232224205-eb9c6123-0d38-4d60-ac93-0990016453e0.png">

## Usage

Once installed `codetypo` will automatically execute when you open or edit any file.

## Config file support

Supports [config fields](https://github.com/khulnasoft/codetypo/blob/master/docs/reference.md) in `codetypo.toml`, `_codetypo.toml`, or `.codetypo.toml`, except:

- `files.ignore*` - have no effect.
- `default.check-filename` - file names are never spell checked.
- `default.check-file` - files are always checked.
- `*.binary` - binary files are always checked.

Config files will be read from the workspace folder or its parents. If there is no workspace folder, then no config file will be read and the codetypo defaults will be used.

## Caveats

- File names are not spell checked.
- Server must be restarted after changing the config files (ie: codetypo.toml).

## Why aren't my misspellings being corrected?

To minimise false-positives `codetypo` only suggests corrections for known misspellings, rather than unknown words like a traditional spell-checker. For more info see [Why was ... not corrected?](https://github.com/khulnasoft/codetypo?tab=readme-ov-file#why-was--not-corrected).

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) to get started.
