# kodigocode

A lightweight, Rust‑based **OpenClaude** command‑line interface.

## Overview

`kodigocode` is a small CLI wrapper around the OpenClaude toolset. It provides a simple way to execute OpenClaude tools, load custom plugins, and inspect the version of the binary.

The project demonstrates:
- Clap‑based argument parsing.
- Dynamic plugin loading via `libloading`.
- A minimal configuration system (TOML) for runtime options.

## Prerequisites

- **Rust** (stable 1.70+ recommended) – install with `rustup`.
- **Cargo** (bundled with Rust).
- An internet connection for fetching dependencies.

## Installation

```bash
# Clone the repository
git clone https://github.com/ritchegerona/kodigocode.git
cd kodigocode

# Build and install the binary globally
cargo install --path .
```

The binary will be placed in Cargo’s bin directory (usually `~/.cargo/bin`). Ensure that directory is on your `$PATH`.

## Building from source (without installing)

```bash
cargo build          # Debug build
cargo build --release   # Optimised release build
```

The resulting binary can be found at `target/debug/kodigocode` or `target/release/kodigocode`.

## Usage

```text
kodigocode <SUBCOMMAND>

Subcommands:
  version            Print version information
  run <tool> [args] Execute a tool with optional arguments
  load-plugins <dir> Load plugins from a directory
```

### Examples

- Show version:
  ```bash
  kodigocode version
  ```
- Run a tool (replace `<tool>` with the desired tool name):
  ```bash
  kodigocode run my_tool --option value
  ```
- Load plugins from `./plugins` directory:
  ```bash
  kodigocode load-plugins ./plugins
  ```

## Configuration

The CLI reads configuration from `$HOME/.config/openclaude/config.toml`. The configuration file is optional; defaults are used when it does not exist.

Sample `config.toml`:

```toml
max_context_tokens = 128000
log_level = "info"
plugins_dir = "/Users/username/.config/openclaude/plugins"
```

## Plugin System

Plugins are shared objects (`*.so`) that expose a `plugin_entry` symbol returning a boxed implementation of the `Tool` trait. To develop a plugin, implement the `Tool` trait in a separate crate, compile as a `cdylib`, and place the resulting `.so` file in a directory referenced by `plugins_dir`.

## Contributing

Contributions are welcome! Please follow these steps:
1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Make your changes and ensure they compile.
4. Add or update tests under `tests/`.
5. Run the test suite: `cargo test`.
6. Open a pull request.

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
