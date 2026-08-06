# Contributing Guide

We welcome contributions to **kodigocode**. This guide outlines how to get started, the workflow we use, and best practices to keep the project healthy.

## Prerequisites

- A recent stable Rust toolchain (1.70 or newer). Install via [rustup](https://rustup.rs/).
- Cargo (bundled with Rust).
- Git.

## Fork & Clone

1. Fork the repository on GitHub.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/<your-username>/kodigocode.git
   cd kodigocode
   ```

## Development Workflow

1. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. **Make changes** – add code, documentation, or tests.
3. **Run the test suite** to ensure nothing breaks:
   ```bash
   cargo test
   ```
4. **Run the formatter** (optional but encouraged):
   ```bash
   cargo fmt
   ```
5. **Commit** with a clear, concise message:
   ```bash
   git add .
   git commit -m "feat: brief description of change"
   ```
6. **Push** to your fork:
   ```bash
   git push origin feature/your-feature-name
   ```
7. **Open a Pull Request** on the upstream repository.

## Pull Request Checklist

- Adopt the conventional commit style used in the repository.
- Ensure `cargo test` passes on CI (the GitHub Actions workflow runs on every PR).
- Include or update tests for new functionality or bug fixes.
- Update documentation if you expose new CLI options or behaviour.
- Verify that `cargo clippy` reports no new warnings (run with `cargo clippy -- -D warnings`).

## Code Style

- Follow the existing code style: use `snake_case` for functions/variables, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants.
- Keep functions small and focused; extract reusable logic into helper functions when appropriate.
- Prefer `Result<T, anyhow::Error>` for error handling, mirroring the rest of the codebase.

## Testing

- Unit tests belong in the same module under a `#[cfg(test)]` block.
- Integration tests live in the `tests/` directory and are run with `cargo test`.
- Aim for coverage of new public APIs; the CI pipeline will flag failing tests.

## Issues & Bugs

- Search existing issues before opening a new one.
- When reporting a bug, include:
  - Steps to reproduce.
  - Expected vs. actual behavior.
  - Output of `kodigocode --version` and your OS.

## License

By contributing you agree that your contributions will be licensed under the Apache‑2.0 license, the same as the project.
