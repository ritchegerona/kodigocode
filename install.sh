#!/usr/bin/env bash
set -euo pipefail

# Installation script for KodigoCode project
# -------------------------------------------------
# 1. Ensure Bun is installed (https://bun.sh)
# 2. Install project dependencies
# 3. Build the project
# 4. Run initial tests
# -------------------------------------------------

# Check for bun
if ! command -v bun >/dev/null 2>&1; then
  echo "Bun is not installed. Please install it from https://bun.sh and re-run this script."
  exit 1
fi

# Install workspace dependencies
echo "Installing dependencies..."
bun install

# Build the project (if a build script is defined)
if grep -q '"build"' package.json; then
  echo "Running build script..."
  bun run build
fi

# Run tests
if grep -q '"test"' package.json; then
  echo "Running tests..."
  bun run test
fi

echo "Installation complete. You can now run the TUI with: bun run apps/tui/src/index.tsx"
