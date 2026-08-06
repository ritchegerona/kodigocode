# KodigoCode

A modular AI‑software‑engineering assistant built with **TypeScript** and **Bun**.

## Quick Start

```sh
# Install Bun (if not already installed)
curl -fsSL https://bun.sh/install | bash

# Install dependencies
bun install

# Build the workspace
bun run build

# Run the CLI
bun run apps/cli/src/index.ts
```

## Project Layout

```
kodigocode/
├─ apps/          # Executables (CLI, TUI, daemon)
├─ packages/      # Core libraries (tools, models, …)
├─ templates/    # Code templates
├─ docs/          # Documentation
├─ examples/      # Sample projects
└─ tests/         # Vitest test suite
```

## Development

- `bun run test` – runs the Vitest suite.
- `bun run lint` – (placeholder for linting).
- Add new packages under `packages/` and reference them via the workspace.

## License

MIT
