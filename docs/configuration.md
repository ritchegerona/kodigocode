# Configuration Guide

`kodigocode` reads its runtime configuration from a TOML file located at:
```
$HOME/.config/openclaude/config.toml
```
If the file does not exist, default settings are used.

## Available Settings

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `max_context_tokens` | `usize` | `128000` | Maximum number of tokens the OpenClaude context window can hold.
| `log_level` | `String` | `"info"` | Logging level for the CLI (`error`, `warn`, `info`, `debug`, `trace`).
| `plugins_dir` | `PathBuf` | `$HOME/.config/openclaude/plugins` | Directory where shared‑object plugins (`*.so`) are loaded from when using the `load-plugins` sub‑command.

## Example `config.toml`

```toml
max_context_tokens = 200000
log_level = "debug"
plugins_dir = "/Users/username/.config/openclaude/plugins"
```

## Reloading Configuration

The configuration is read once at startup. To apply changes, restart the `kodigocode` binary.
