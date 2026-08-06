use async_trait::async_trait;
use anyhow::{Result, anyhow};
use tokio::process::Command;
use crate::tool::{Tool, ToolRegistry};

/// Simple tool that proxies `git` commands.
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
}

/// Register the GitTool in a registry (convenience function).
pub fn register_git_tool(registry: &mut ToolRegistry) {
    registry.register(GitTool::new());
}
