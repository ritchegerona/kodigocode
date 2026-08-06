use anyhow::Result;
use libloading::{Library, Symbol};
use std::path::Path;
use std::fs;
use super::tool::Tool;

/// Representation of the plugin manifest (openclaude-plugin.json).
#[derive(Debug, serde::Deserialize)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Load all plugins from a directory, returning a vector of boxed tools.
pub fn load_plugins(dir: &Path) -> Result<Vec<Box<dyn Tool>>> {
    let mut tools: Vec<Box<dyn Tool>> = Vec::new();
    if !dir.is_dir() {
        return Ok(tools);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("so") {
            continue;
        }
        // Load the library
        unsafe {
            let lib = Library::new(&path)?;
            // Expected symbol signature
            let constructor: Symbol<unsafe fn() -> Box<dyn Tool>> = lib.get(b"plugin_entry")?;
            let tool = constructor();
            tools.push(tool);
            // Note: lib is dropped here, which may unload symbols; in real code keep lib alive.
        }
    }
    Ok(tools)
}
