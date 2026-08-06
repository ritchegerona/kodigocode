use async_trait::async_trait;
use anyhow::{Result, anyhow};
use serde_json::Value;
use tokio::process::Command;

use crate::tool::{Tool, ToolDescriptor, ToolError, ToolRegistry, ToolResult, ToolCapability};

pub struct GitTool;

impl GitTool {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Tool for GitTool {
    fn name(&self) -> &'static str {
        "git"
    }

    fn description(&self) -> &'static str {
        "Execute git commands and return their output"
    }

    fn descriptor(&self) -> ToolDescriptor {
        ToolDescriptor::new("git", "Execute git commands and return their output")
            .with_input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "args": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Arguments to pass to git"
                    }
                },
                "required": ["args"]
            }))
            .with_capability(ToolCapability::ExecutesCode)
    }

    fn is_mutating(&self) -> bool {
        true
    }

    async fn run(&self, args: &[String]) -> Result<String> {
        let mut cmd = Command::new("git");
        for arg in args {
            cmd.arg(arg);
        }
        let output = cmd.output().await.map_err(|e| anyhow!(e))?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(anyhow!(
                "git command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }

    async fn run_typed(&self, input: &Value) -> std::result::Result<ToolResult, ToolError> {
        let args_val = input.get("args").ok_or_else(|| ToolError::missing_field("args"))?;
        let args_arr = args_val.as_array().ok_or_else(|| {
            ToolError::invalid_input("field 'args' must be an array of strings")
        })?;
        let args: Vec<String> = args_arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect();

        match self.run(&args).await {
            Ok(output) => Ok(ToolResult::success(output)),
            Err(e) => Err(ToolError::execution_failed(e.to_string())),
        }
    }
}

pub fn register_git_tool(registry: &mut ToolRegistry) {
    registry.register(GitTool::new());
}