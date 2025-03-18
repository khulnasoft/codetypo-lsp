# VS Code Settings

This extension contributes the following settings:

- `codetypo.config`: Custom config. Used together with a config file found in the workspace or its parents, taking precedence for settings declared in both. Equivalent to the codetypo `--config` [cli argument](https://github.com/khulnasoft/codetypo/blob/rust/docs/reference.md).
- `codetypo.diagnosticSeverity`: How codetypo are rendered in the editor, can be one of an Error, Warning, Information or Hint.
- `codetypo.logLevel`: Logging level of the language server. Logs appear in the _Output -> Codetypo_ pane.
- `codetypo.path`: Path to the `codetypo-lsp` binary. If empty the bundled binary will be used.
- `codetypo.trace.server`: Traces the communication between VS Code and the language server. Recommended for debugging only.

To disable `codetypo` per workspace, see [disable this extension](https://code.visualstudio.com/docs/editor/extension-marketplace#_disable-an-extension).

## VS Code Commands

| Command           | Description         |
| ----------------- | ------------------- |
| Codetypo: Restart | Restart the server. |
