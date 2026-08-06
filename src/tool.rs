use async_trait::async_trait;
use anyhow::Result;

/// Abstraction for a pluggable tool.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Human readable name of the tool (e.g., "git").
    fn name(&self) -> &'static str;
    /// Run the tool with arguments, returning stdout (or error).
    async fn run(&self, args: &[String]) -> Result<String>;
}

/// Simple registry that holds boxed tools indexed by name.
pub struct ToolRegistry {
    tools: std::collections::HashMap<String, Box<dyn Tool>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: std::collections::HashMap::new() }
    }

    /// Register a tool implementation. Overwrites any existing entry with same name.
    pub fn register<T: Tool + 'static>(&mut self, tool: T) {
        self.tools.insert(tool.name().to_string(), Box::new(tool));
    }

    /// Register a pre‑boxed tool (e.g., from a plugin).
    pub fn register_boxed(&mut self, tool: Box<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    /// Retrieve a tool by name.
    pub fn get(&self, name: &str) -> Option<&Box<dyn Tool>> {
        self.tools.get(name)
    }
}
