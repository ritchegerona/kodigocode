# KodigoCode

A modular AI‑software‑engineering assistant built with **TypeScript** and **Bun**. It provides a lightweight CLI, an interactive TUI, and a multi‑agent architecture for coding, testing, documentation, security, and Git operations.

## Installation

```sh
# 1. Install Bun (>= 1.0.0) – the runtime and package manager used by KodigoCode
curl -fsSL https://bun.sh/install | bash

# 2. Verify Bun is available
bun --version

# 3. Clone the repository (if you haven't already)
git clone https://github.com/ritchegerona/kodigocode.git
cd kodigocode

# 4. Install project dependencies using Bun
bun install
```

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
