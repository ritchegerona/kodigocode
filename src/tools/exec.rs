use async_trait::async_trait;
use anyhow::Result;
use std::process::Stdio;
use tokio::process::Command;
use crate::tool::Tool;

/// Executes a command in a PTY-like fashion, streaming stdout/stderr.
pub struct ExecTool;

#[async_trait]
impl Tool for ExecTool {
    fn name(&self) -> &'static str { "exec" }
    fn description(&self) -> &'static str { "Execute a shell command and return its output" }
    async fn run(&self, args: &[String]) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: exec <command> [args...]".into());
        }
        let mut cmd = Command::new(&args[0]);
        if args.len() > 1 {
            cmd.args(&args[1..]);
        }
        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        let output = cmd.output().await?;
        let mut result = String::new();
        result.push_str(&String::from_utf8_lossy(&output.stdout));
        if !output.stderr.is_empty() {
            result.push_str("\n---STDERR---\n");
            result.push_str(&String::from_utf8_lossy(&output.stderr));
        }
        Ok(result)
    }
}

pub fn register_exec_tool(registry: &mut crate::tool::ToolRegistry) {
    registry.register(ExecTool);
}
