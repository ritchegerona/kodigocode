use anyhow::Result;
use libloading::{Library, Symbol};
use serde_json::Value;
use std::fs;
use std::path::Path;

use super::tool::{Tool, ToolDescriptor, ToolError, ToolResult};

pub struct PluginTool {
    inner: Box<dyn Tool>,
}

impl PluginTool {
    pub fn new(tool: Box<dyn Tool>) -> Self {
        Self { inner: tool }
    }
}

#[async_trait::async_trait]
impl Tool for PluginTool {
    fn name(&self) -> &'static str {
        self.inner.name()
    }

    fn description(&self) -> &'static str {
        self.inner.description()
    }

    fn source(&self) -> &'static str {
        "plugin"
    }

    fn descriptor(&self) -> ToolDescriptor {
        let mut desc = self.inner.descriptor();
        desc.name = self.inner.name().to_string();
        desc
    }

    fn is_mutating(&self) -> bool {
        self.inner.is_mutating()
    }

    async fn run(&self, args: &[String]) -> Result<String> {
        self.inner.run(args).await
    }

    async fn run_typed(&self, input: &Value) -> std::result::Result<ToolResult, ToolError> {
        self.inner.run_typed(input).await
    }
}

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
        unsafe {
            let lib = Library::new(&path)?;
            let constructor: Symbol<unsafe fn() -> Box<dyn Tool>> =
                lib.get(b"plugin_entry")?;
            let raw_tool = constructor();
            let wrapped = PluginTool::new(raw_tool);
            tools.push(Box::new(wrapped));
        }
    }
    Ok(tools)
}

pub fn discover_plugins(dir: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    if !dir.is_dir() {
        return Ok(names);
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("so") {
            names.push(entry.file_name().to_string_lossy().to_string());
        }
    }
    names.sort();
    Ok(names)
}
