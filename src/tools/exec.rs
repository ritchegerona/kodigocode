use async_trait::async_trait;
use anyhow::Result;
use serde_json::Value;
use std::process::Stdio;
use tokio::process::Command;

use crate::tool::{Tool, ToolDescriptor, ToolError, ToolRegistry, ToolResult, ToolCapability, required_str};

pub struct ExecTool;

#[async_trait]
impl Tool for ExecTool {
    fn name(&self) -> &'static str {
        "exec"
    }
    fn description(&self) -> &'static str {
        "Execute a shell command and return its output"
    }

    fn descriptor(&self) -> ToolDescriptor {
        ToolDescriptor::new("exec", "Execute a shell command and return its output")
            .with_input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "command": {
                        "type": "string",
                        "description": "The shell command to execute"
                    },
                    "args": {
                        "type": "array",
                        "items": { "type": "string" },
                        "description": "Optional arguments to the command"
                    }
                },
                "required": ["command"]
            }))
            .with_timeout(300_000)
            .with_capability(ToolCapability::ExecutesCode)
            .mutating()
    }

    fn is_mutating(&self) -> bool {
        true
    }

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

    async fn run_typed(&self, input: &Value) -> std::result::Result<ToolResult, ToolError> {
        let command = required_str(input, "command")?;

        let mut cmd = Command::new(command);
        if let Some(args_val) = input.get("args") {
            if let Some(args_arr) = args_val.as_array() {
                let args: Vec<&str> = args_arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect();
                cmd.args(&args);
            }
        }

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        match cmd.output().await {
            Ok(output) => {
                let mut result = String::new();
                result.push_str(&String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    result.push_str("\n---STDERR---\n");
                    result.push_str(&String::from_utf8_lossy(&output.stderr));
                }
                Ok(ToolResult::success(result))
            }
            Err(e) => Err(ToolError::execution_failed(e.to_string())),
        }
    }
}

pub fn register_exec_tool(registry: &mut ToolRegistry) {
    registry.register(ExecTool);
}