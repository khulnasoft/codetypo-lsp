# Zed Settings

The Codetypo extension can be configured through a `.codetypo.toml` configuration file, which reference can be found [here](https://github.com/khulnasoft/codetypo/blob/rust/docs/reference.md).

Additionally, you can configure it in Zed's settings with the following:

```javascript
{
    "lsp": {
        "codetypo": {
            "initialization_options": {
                // Path to your codetypo config file, .codetypo.toml by default.
                "config": ".codetypo.toml",
                // Path to your codetypo-lsp executable, takes $PATH into account.
                "path": "codetypo-lsp",
                // Diagnostic severity within Zed. "Error" by default, can be:
                // "Error", "Hint", "Information", "Warning"
                "diagnosticSeverity": "Error",
                // Minimum logging level for the LSP, displayed in Zed's logs. "info" by default, can be:
                // "debug", "error", "info", "off", "trace", "warn"
                "logLevel": "info",
                // Traces the communication between ZED and the language server. Recommended for debugging only. "off" by default, can be:
                // "messages", "off", "verbose"
                "trace.server": "off"
            }
        }
    }
}
```

**WARNING**: When modifying your Codetypo configuration either in `.codetypo.toml` or `Cargo.toml` you will need to reload the workspace to take them into account.

You do not need to reload when editing Zed's `settings.json`.
