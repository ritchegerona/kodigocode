# KodigoCode

A modular AI‑software‑engineering assistant built with **TypeScript** and **Bun**. It provides a lightweight CLI, an interactive TUI, and a multi‑agent architecture for coding, testing, documentation, security, and Git operations.

## Installation

### 1️⃣ Install Bun (>= 1.0.0)
```sh
curl -fsSL https://bun.sh/install | bash
```

### 2️⃣ Verify Bun installation
```sh
bun --version   # should print something like 1.x.x
```

### 3️⃣ Clone the repository (or update an existing checkout)
```sh
# Fresh install
git clone https://github.com/ritchegerona/kodigocode.git
cd kodigocode

# If you already have the repo, fetch the latest changes:
# (run inside the repo directory)
git fetch origin && git checkout master && git pull origin master
```

### 4️⃣ (Optional) Uninstall Bun
```sh
# Remove Bun binaries and the ~/.bun directory
rm -rf "$HOME/.bun"
# Remove the PATH entry added to your shell profile (e.g., ~/.zshrc or ~/.bashrc)
sed -i '' '/\.bun\/bin/d' ~/.zshrc   # adjust the file name for your shell
```

### 5️⃣ Install project dependencies
```sh
export PATH="$HOME/.bun/bin:$PATH"   # make sure Bun is on your PATH for this session
bun install
```

### 6️⃣ Build the workspace (compile all packages)
```sh
bun run build
```

After these steps you can run the CLI or TUI as described below.


## Quick Start

```sh
# Install Bun (if not already installed)
curl -fsSL https://bun.sh/install | bash

# Install dependencies
bun install

# Build the workspace (compile all packages)
bun run build

# Run the CLI
bun run apps/cli/src/index.tsx --help
```

## Usage

Below is a quick walkthrough of the most common workflows.

### 1️⃣ Run the CLI

```sh
# Show available commands
bun run apps/cli/src/index.tsx --help

# Get a quick status of the repository
bun run apps/cli/src/index.tsx git status

# Run the test suite
bun run apps/cli/src/index.tsx test

# Generate a short documentation excerpt
bun run apps/cli/src/index.tsx doc

# Perform a placeholder security audit
bun run apps/cli/src/index.tsx security

# Search for a symbol (e.g., a function name) across the code base
bun run apps/cli/src/index.tsx symbol-search fetchData
```

### 2️⃣ Use the interactive TUI

```sh
bun run apps/tui/src/index.tsx
```

The TUI will render a file‑tree on the left and a Git diff view on the right. Use the arrow keys to navigate the tree; press `q` to quit.

### 3️⃣ Extend functionality with plugins

1. Create a new folder under `plugins/` (e.g., `plugins/myplugin`).
2. Export a `plugin` object that implements the `KodigoPlugin` interface.
3. The plugin will be loaded automatically on `MasterAgent` start and can register new commands via `context.registerCommand`.

For a concrete example, see the built‑in `plugins/hello` plugin.

That’s it! You can now combine the CLI and TUI to iteratively develop, test, and inspect your code base.

## Advanced Features (Roadmap)

The following capabilities are either already available or planned for upcoming releases:

- **Interactive chat** (`chat` command) – converse with the LLM directly from the CLI.
- **Tool calling** – the LLM can request execution of built‑in tools (fs, git, search, etc.).
- **Multi‑file editing** – agents can generate and apply changes across multiple files with a diff preview.
- **MCP support** – Model‑Client‑Plugin protocol for external model services.
- **Multi‑model support** – switch between OpenAI, Anthropic, Gemini, Ollama, etc.
- **Plugin marketplace** – discover, install, enable, and manage plugins from the TUI or CLI.
- **Multi‑agent workflows** – chain agents (e.g., `chat → code → test → git commit`).
- **Project memory** – persistent storage of chat history, symbol indexes, and user preferences.
- **Visual TUI dashboard** – integrated panes for file tree, editor, diff, chat, and git status.


## Commands

The CLI exposes a set of top‑level commands managed by the **MasterAgent**:

- `help` – list all available commands.
- `code` – run the **CodingAgent** (placeholder for code generation).
- `test` – run the **TestingAgent** (executes the Vitest suite).
- `doc` – run the **DocAgent** (outputs a short README excerpt).
- `security` – run the **SecurityAgent** (placeholder security audit).
- `git <action>` – run the **GitAgent** (`status`, `diff`, `add`, `commit`).
- `symbol-search <query>` – fast export‑symbol lookup across the workspace.
- `hello` – sample plugin command (provided by the `hello` plugin).

## TUI (Terminal UI)

```sh
bun run apps/tui/src/index.tsx
```

The TUI displays:

- A live **file tree** explorer.
- A **Git diff** viewer.
- Real‑time status messages from the MasterAgent.

## Project Layout

```
kodigoCode/
├─ apps/                # Executables (CLI, TUI, daemon)
├─ packages/            # Core libraries (tools, models, agents, etc.)
│   ├─ core/           # Logger, config loader, command registry
│   ├─ agents/         # Multi‑agent implementations (coding, testing, ...)
│   ├─ tools/          # Filesystem, shell, git, search, symbol‑search tools
│   ├─ models/         # LLM provider abstractions
│   ├─ terminal/       # Ink UI components (FileTree, DiffViewer, Spinner)
│   └─ …                # Additional shared packages
├─ templates/           # Code templates for scaffolding
├─ docs/                # Documentation and design docs
├─ examples/            # Sample projects demonstrating usage
└─ tests/               # Vitest test suite
```

## Development

- `bun run test` – runs the Vitest suite.
- `bun run lint` – placeholder for linting (e.g., ESLint).
- Add new packages under `packages/` and reference them via the workspace.
- Extend the **MasterAgent** to register additional agents or plugin commands.

## License

MIT
