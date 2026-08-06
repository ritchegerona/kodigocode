# kodigocode

A lightweight, Rust‑based **OpenClaude** command‑line interface with interactive REPL, dynamic plugin loading, and automatic configuration setup.

## Overview

`kodigocode` is a CLI wrapper around the OpenClaude toolset. It provides a simple way to execute OpenClaude tools, load custom plugins, run an interactive session, and inspect the version of the binary.

The project demonstrates:
- Clap‑based argument parsing.
- Dynamic plugin loading via `libloading`.
- Automatic configuration setup (TOML) on first run.
- Interactive REPL for tool execution.
- Structured logging with timestamps.

## Prerequisites

- **Rust** (stable 1.70+ recommended) – install with [rustup](https://rustup.rs/).
- **Cargo** (bundled with Rust).
- An internet connection for fetching dependencies.

## Quick Start

```bash
git clone https://github.com/ritchegerona/kodigocode.git
cd kodigocode
make install
```

Or manually:

```bash
cargo install --path .
```

The binary is placed in `~/.cargo/bin`. Ensure that directory is on your `$PATH`.

## Installation

### Using Makefile

```bash
make build          # Debug build
make release        # Optimised release build
make install        # Install globally (runs cargo install --path .)
make uninstall      # Remove the binary (runs cargo uninstall kodigocode)
```

### Manual

```bash
cargo install --path .          # Build and install globally
cargo build                     # Debug build (target/debug/kodigocode)
cargo build --release           # Release build (target/release/kodigocode)
```

## Uninstallation

```bash
make uninstall
```

Or manually:

```bash
cargo uninstall kodigocode
```

If `cargo uninstall` isn't available, remove the binary manually from `~/.cargo/bin/kodigocode`.

## Configuration

On first run, `kodigocode` automatically creates the configuration file and plugins directory. No manual setup is required.

| Setting | Type | Default | Description |
|---|---|---|---|
| `max_context_tokens` | `usize` | 128000 | Maximum context window tokens |
| `log_level` | `String` | `"info"` | Log level: `error`, `warn`, `info`, `debug`, `trace` |
| `plugins_dir` | `PathBuf` | `~/.config/openclaude/plugins` | Directory containing `.so` plugin files |

**Config file location:** `~/.config/openclaude/config.toml`

A `config.toml` with defaults is generated for you on first launch:

```toml
max_context_tokens = 128000
log_level = "info"
plugins_dir = "/home/username/.config/openclaude/plugins"
```

To reload configuration, restart the `kodigocode` binary.

## Usage

```
kodigocode <SUBCOMMAND>

Subcommands:
  version            Print version information
  run <tool> [args]  Execute a tool with optional arguments
  list               List all registered tools
  load-plugins <dir> Load plugins from a directory
  discover-plugins <dir>  Scan a directory for .so plugin files
  interactive        Start an interactive REPL session
  help               Show help
```

### Subcommands

**`version`**
```bash
kodigocode version
# openclaude 0.1.0
```

**`run <tool> [args]`**
```bash
kodigocode run git status
# On branch main
```

**`list`**
```
#  20 * Output example
kodigo list
# NAME  and other
# SOURCE  # DESCRIPTION
# ...out
  git                   built-in  Execute git commands
```

**`load-plugins <dir>`**
```
A terminal command only for now, among other
```

---

<details>
sem">
Ensure information flows follow the usage.

---

### Interactive Mode

```
kodigocode interactive
```

Inside REPL:

| Command | Description |
|---|---|
| `help` / `?` | Show available commands |
| `list` | List registered tools |
| `run <tool> [args]` | Execute a tool |
| `exit` / `quit` | Exit interactive mode |

## Plugin System

Plugins are shared objects (`*.so`) that expose a `plugin_entry` symbol returning a `Box<dyn Tool>`. Write a plugin, compile it as `cdylib`, and place the `.so` in the configured `plugins_dir`.

### Loading & Discovering Plugins

```bash
kodigocode load-plugins ~/.config/openclaude/plugins
kodigocode discover-plugins /path/to/plugins
```

See [Plugin Development Guide](docs/plugin_development.md) for writing custom plugins.

## Testing

```bash
cargo test
```

## Contributing

Contributions are welcome!
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Write code and tests.
4. Run `cargo test`.
5. Open a pull request.

See [Contributing Guide](docs/CONTRIBUTING.md) for details.

## License

Licensed under the Apache‑2.0 License. See `LICENSE` for details.

## Acknowledgements

- **clap** – command‑line argument parsing.
- **anyhow**, **thiserror** – error handling.
- **log**, **env_logger** – logging infrastructure.
- **serde**, **toml** – configuration parsing.
- **tokio** – async runtime.
- **libloading** – dynamic plugin loading.
- **dirs** – locating user configuration directories.