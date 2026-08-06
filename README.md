# KodigoCode

A **Rust-based AI chat TUI** with multi-provider support, extensible tool system, model registry, session management, and dynamic plugin loading. Built on CodeWhale's architecture patterns.

The binary is `kc`.

## Overview

`kc` is a terminal-native AI chat client supporting 10+ LLM providers through OpenAI-compatible APIs. It features:

- **Full TUI** with syntax-highlighted markdown rendering and streaming responses
- **10+ providers**: OpenClaude, OpenAI, DeepSeek, Gemini, Groq, Mistral, Ollama, OpenRouter, LM Studio, NVIDIA NIM
- **Model registry** with alias resolution and family-based grouping
- **Pluggable tool system** with JSON schema descriptors, capability tagging, and timeout/duration support
- **Permission system**: tools declare whether they are read-only or mutating
- **Concurrent tool execution** with read/write locking per tool
- **Session snapshots** for undo/restore
- **Thread persistence** to disk with archive/unarchive
- **Dynamic `.so` plugin loading** via `libloading`

Built on patterns from [CodeWhale](https://github.com/Hmbown/CodeWhale): ToolRegistry dispatch, ToolError variants, JSON type extractors, ToolCapability tags, ToolCallRuntime concurrent execution, and model alias registry.

## Prerequisites

- **Rust** (stable 1.70+ recommended) – [rustup](https://rustup.rs/)
- **Cargo** (bundled with Rust)

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
make install        # Install globally
make uninstall      # Remove the binary
```

### Manual

```bash
cargo install --path .
cargo build                     # Debug build (target/debug/kc)
cargo build --release           # Release build (target/release/kc)
```

## Configuration

On first run, `kc` automatically creates the configuration file and plugins directory.

| Setting | Type | Default | Description |
|---|---|---|---|
| `max_context_tokens` | `usize` | 128000 | Max context window tokens |
| `log_level` | `String` | `"info"` | Log level |
| `plugins_dir` | `PathBuf` | `~/.config/kodigocode/plugins` | Plugin `.so` directory |
| `provider.provider` | `String` | `"deepseek"` | Default AI provider |
| `provider.model` | `String` | `"deepseek-chat"` | Default model |

**Config file location:** `~/.config/kodigocode/config.toml`

## Usage

```bash
kc              # Launch TUI chat (default)
kc version      # Print version
kc list         # List registered tools
kc run <tool> [args]  # Execute a tool
kc load-plugins <dir>  # Load .so plugins
kc discover-plugins <dir>  # Scan for .so files
```

### TUI Keybindings

| Key | Action |
|---|---|
| `/` | Open command palette (search providers/models) |
| `/model` | Quick model switch |
| `Enter` | Send message |
| `Ctrl+C` / `Esc` | Exit |
| `PageUp` / `PageDown` | Scroll history |

### Slash Commands

| Command | Description |
|---|---|
| `/model [name]` | List or switch model |
| `/setup key <provider> <key>` | Set API key |
| `/setup url <provider> <url>` | Override base URL |
| `/clear` | Clear chat history |
| `/undo` | Undo last turn |
| `/help` | Show help |

## Architecture

```
src/
├── main.rs              # Entry point, CLI dispatch
├── chat.rs              # TUI: ratatui rendering, streaming, palette
├── cli.rs               # Clap subcommands
├── config.rs            # TOML config load/save
├── tool.rs              # Tool trait, ToolRegistry, ToolError, ToolResult,
│                        #   ToolDescriptor, ToolCapability, type extractors,
│                        #   ToolCall, ToolCallRuntime, dispatch
├── model_registry.rs    # ModelFamily enum, ModelInfo, ModelRegistry, aliases
├── session.rs           # SessionStore, Thread, Snapshot, SessionSource
├── palette.rs           # Color theme (WHALE)
├── providers.rs         # 10+ provider definitions with models
├── ai/
│   ├── mod.rs           # AiProvider trait, StreamDelta, ToolDef
│   └── openai_compat.rs # OpenAI-compatible HTTP + SSE streaming
├── plugin/
│   └── mod.rs           # Dynamic .so plugin loading
└── tools/
    ├── git_tool.rs      # Git command proxy
    ├── fs.rs            # Filesystem read/write/create/delete/scan
    └── exec.rs          # Shell command execution
```

### Tool System (CodeWhale-inspired)

Every tool implements the `Tool` trait with both raw (`run`) and typed (`run_typed`) execution. Each tool carries a `ToolDescriptor` with:

- **JSON input/output schemas** for LLM function calling
- **Capability tags** (`ReadOnly`, `WritesFiles`, `ExecutesCode`, etc.)
- **Mutating flag** for permission gating
- **Timeout** and **parallel support** for concurrent execution

The `ToolRegistry::dispatch()` method validates permissions, acquires execution locks, and supports per-tool timeouts.

### Model Registry

`ModelRegistry` resolves user-requested model names through an alias map with a priority chain:
1. Exact match by model ID
2. Alias map lookup (case-insensitive)
3. Family default fallback
4. Global default

### Session & Undo

`SessionStore` manages threads with snapshot-based undo history. Each user turn pushes a snapshot; `/undo` pops to revert.

## Plugin System

Plugins are shared objects (`*.so`) exposing a `plugin_entry` symbol returning `Box<dyn Tool>`.

```bash
kc plugins ~/.config/kodigocode/plugins
kc discover-plugins /path/to/plugins
```

See [Plugin Development Guide](docs/plugin_development.md).

## Testing

```bash
cargo test
```

## Contributing

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Write code and tests.
4. Run `cargo test`.
5. Open a pull request.

See [Contributing Guide](docs/CONTRIBUTING.md).

## License

Apache-2.0. See `LICENSE` for details.

## Acknowledgements

- **CodeWhale** — architecture patterns for tool system, model registry, execution runtime
- **clap** — CLI parsing
- **ratatui** — terminal UI framework
- **syntect** — syntax highlighting
- **pulldown-cmark** — markdown rendering
- **tokio** — async runtime
- **libloading** — dynamic plugin loading