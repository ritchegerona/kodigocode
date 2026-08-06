# Installation Guide

## Prerequisites

- **Rust** (stable 1.70+). Install via [rustup](https://rustup.rs/).
- **Cargo** (bundled with Rust).
- An internet connection for fetching crates.

## Steps

1. **Clone the repository**
   ```bash
   git clone https://github.com/ritchegerona/kodigocode.git
   cd kodigocode
   ```
2. **Build and install the binary globally**
   ```bash
   cargo install --path .
   ```
   This compiles the project and places the `kodigocode` executable in `~/.cargo/bin`.

3. **Verify the installation**
   ```bash
   kodigocode --help
   ```
   You should see the list of available sub‑commands.

### Optional: Local Build

If you prefer not to install globally, you can build locally:

```bash
cargo build          # Debug build (target/debug/kodigocode)
cargo build --release   # Optimised release build (target/release/kodigocode)
```

Run the binary directly:
```bash
./target/debug/kodigocode version
```