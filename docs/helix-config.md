# Helix config

In your [languages.toml](https://docs.helix-editor.com/languages.html) file configure the `codetypo` language server as follows:

```toml
[language-server.codetypo]
# codetypo-lsp must be on your PATH, or otherwise change this to an absolute path to codetypo-lsp
command = "codetypo-lsp"
# Logging level of the language server. Defaults to error.
# Run with helix -v to output LSP logs to the editor log (:log-open)
environment = {"RUST_LOG" = "error"}
# Custom config. Used together with a config file found in the workspace or its parents,
# taking precedence for settings declared in both. Equivalent to the codetypo `--config` cli argument.
config.config = "~/code/codetypo-lsp/crates/codetypo-lsp/tests/codetypo.toml"
# How codetypo are rendered in the editor, can be one of an Error, Warning, Info or Hint.
# Defaults to Warning.
config.diagnosticSeverity = "Warning"
```

Then add `codetypo` to one or more languages, as the last entry to avoid taking precedence, eg:

```toml
[[language]]
name = "rust"
language-servers = ["rust-analyzer", "codetypo"]
```

Currently it doesn't seem possible to add codetypo to all languages at once, see [this discussion](https://github.com/helix-editor/helix/discussions/8850).
