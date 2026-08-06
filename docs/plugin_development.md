# Plugin Development Guide

`kodigocode` supports extending its functionality via dynamic plugins written in Rust. A plugin is a shared library (`*.so`) that implements the `Tool` trait defined in `src/plugin/mod.rs`.

## Steps to Create a Plugin

1. **Create a new library crate**
   ```bash
   cargo new --lib my_plugin
   cd my_plugin
   ```
2. **Add dependencies** in `Cargo.toml`:
   ```toml
   [dependencies]
   openclaude = { path = ".." } # use the local crate as a dependency
   libloading = "0.8"
   ```
3. **Implement the `Tool` trait**
   ```rust
   use openclaude::plugin::Tool;
   use anyhow::Result;

   pub struct MyTool;

   impl Tool for MyTool {
       fn name(&self) -> &'static str { "my_tool" }
       fn description(&self) -> &'static str { "A custom tool example" }
       fn run(&self, args: &[String]) -> Result<()> {
           println!("Running my_tool with args: {:?}", args);
           Ok(())
       }
   }
   ```
4. **Expose the entry point**
   Add the following to `src/lib.rs` (or another file) and mark it with `#[no_mangle]`:
   ```rust
   #[no_mangle]
   pub unsafe fn plugin_entry() -> Box<dyn Tool> {
       Box::new(MyTool)
   }
   ```
5. **Configure crate type**
   In `Cargo.toml` set the crate type to `cdylib`:
   ```toml
   [lib]
   crate-type = ["cdylib"]
   ```
6. **Build the plugin**
   ```bash
   cargo build --release
   ```
   The compiled shared object will be located at `target/release/libmy_plugin.so`.
7. **Deploy the plugin**
   Copy the `.so` file to the plugins directory used by `kodigocode` (default: `$HOME/.config/openclaude/plugins`).
   ```bash
   cp target/release/libmy_plugin.so $HOME/.config/openclaude/plugins/
   ```
8. **Load the plugin**
   ```bash
   kodigocode load-plugins $HOME/.config/openclaude/plugins
   ```
   The CLI will now list `my_tool` as an available tool.

## Tips & Gotchas

- **Symbol name must be exactly `plugin_entry`** and the function must be `extern "C"` compatible; the `#[no_mangle]` attribute ensures this.
- The plugin binary must be compiled for the same target architecture as the host CLI.
- Keep plugin dependencies minimal to avoid version conflicts.
- Errors during loading are reported to stderr; ensure the shared object is readable.

## Testing Plugins

Create integration tests in your plugin crate that call `MyTool::run` directly. The host CLI can also be used to verify the plugin works:
```bash
kodigocode load-plugins ./path/to/plugin_dir
kodigocode run my_tool --example-arg
```

Happy hacking!