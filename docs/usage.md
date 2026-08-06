# Usage Guide

## Command syntax

```bash
kodigocode <SUBCOMMAND> [OPTIONS]
```

## Sub‑commands

- `version`
  - Prints the CLI version.

- `run <tool> [args]`
  - Executes an OpenClaude tool. Replace `<tool>` with the tool name and provide any tool‑specific arguments after it.
  - Example:
    ```bash
    kodigocode run my_tool --option value
    ```

- `load-plugins <dir>`
  - Dynamically loads plugins from the specified directory. The directory should contain compiled shared objects (`*.so`).
  - Example:
    ```bash
    kodigocode load-plugins ./plugins
    ```

## Help

Run `kodigocode --help` or `kodigocode <SUBCOMMAND> --help` to see detailed help for each command.
