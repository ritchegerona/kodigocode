use async_trait::async_trait;
use anyhow::Result;
use serde_json::Value;
use std::fs;
use std::path::Path;

use crate::tool::{Tool, ToolDescriptor, ToolError, ToolRegistry, ToolResult, ToolCapability, required_str};

pub struct FsTool;

#[async_trait]
impl Tool for FsTool {
    fn name(&self) -> &'static str {
        "fs"
    }
    fn description(&self) -> &'static str {
        "Filesystem operations: read, write, create, delete, rename, move, scan"
    }

    fn descriptor(&self) -> ToolDescriptor {
        ToolDescriptor::new("fs", "Filesystem operations: read, write, create, delete, rename, move, scan")
            .with_input_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["read", "write", "create", "delete", "rename", "move", "scan"],
                        "description": "The filesystem action to perform"
                    },
                    "path": {
                        "type": "string",
                        "description": "Target path"
                    },
                    "data": {
                        "type": "string",
                        "description": "Data for write operations"
                    },
                    "dst": {
                        "type": "string",
                        "description": "Destination path for rename/move"
                    }
                },
                "required": ["action", "path"]
            }))
            .with_capability(ToolCapability::WritesFiles)
            .mutating()
    }

    fn is_mutating(&self) -> bool {
        true
    }

    async fn run(&self, args: &[String]) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: fs <subcommand> [args...]".into());
        }
        match args[0].as_str() {
            "read" => {
                let path = args
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("read requires <path>"))?;
                let content = fs::read_to_string(path)?;
                Ok(content)
            }
            "write" => {
                let path = args
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("write requires <path>"))?;
                let data = args
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("write requires <data>"))?;
                fs::write(path, data)?;
                Ok(format!("Wrote {} bytes to {}", data.len(), path))
            }
            "create" => {
                let path = args
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("create requires <path>"))?;
                if Path::new(path).exists() {
                    return Err(anyhow::anyhow!("File already exists"));
                }
                fs::write(path, "")?;
                Ok(format!("Created empty file {}", path))
            }
            "delete" => {
                let path = args
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("delete requires <path>"))?;
                fs::remove_file(path)?;
                Ok(format!("Deleted {}", path))
            }
            "rename" => {
                let src = args
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("rename requires <src>"))?;
                let dst = args
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("rename requires <dst>"))?;
                fs::rename(src, dst)?;
                Ok(format!("Renamed {} to {}", src, dst))
            }
            "move" => {
                let src = args
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("move requires <src>"))?;
                let dst = args
                    .get(2)
                    .ok_or_else(|| anyhow::anyhow!("move requires <dst>"))?;
                fs::rename(src, dst)?;
                Ok(format!("Moved {} to {}", src, dst))
            }
            "scan" => {
                let dir = args
                    .get(1)
                    .ok_or_else(|| anyhow::anyhow!("scan requires <dir>"))?;
                let mut entries = vec![];
                for entry in walkdir::WalkDir::new(dir) {
                    let entry = entry?;
                    entries.push(entry.path().display().to_string());
                }
                Ok(entries.join("\n"))
            }
            _ => Ok(format!("Unsupported fs subcommand: {}", args[0])),
        }
    }

    async fn run_typed(&self, input: &Value) -> std::result::Result<ToolResult, ToolError> {
        let action = required_str(input, "action")?;
        let path = required_str(input, "path")?;

        let result = match action {
            "read" => match fs::read_to_string(path) {
                Ok(content) => ToolResult::success(content),
                Err(e) => return Err(ToolError::execution_failed(e.to_string())),
            },
            "write" => {
                let data = required_str(input, "data")?;
                match fs::write(path, data) {
                    Ok(()) => ToolResult::success(format!("Wrote {} bytes to {}", data.len(), path)),
                    Err(e) => return Err(ToolError::execution_failed(e.to_string())),
                }
            }
            "create" => {
                if Path::new(path).exists() {
                    return Err(ToolError::invalid_input("File already exists"));
                }
                match fs::write(path, "") {
                    Ok(()) => ToolResult::success(format!("Created empty file {}", path)),
                    Err(e) => return Err(ToolError::execution_failed(e.to_string())),
                }
            }
            "delete" => match fs::remove_file(path) {
                Ok(()) => ToolResult::success(format!("Deleted {}", path)),
                Err(e) => return Err(ToolError::execution_failed(e.to_string())),
            },
            "rename" | "move" => {
                let dst = required_str(input, "dst")?;
                match fs::rename(path, dst) {
                    Ok(()) => {
                        ToolResult::success(format!("Moved {} to {}", path, dst))
                    },
                    Err(e) => return Err(ToolError::execution_failed(e.to_string())),
                }
            },
            "scan" => {
                let mut entries = vec![];
                for entry_result in walkdir::WalkDir::new(path) {
                    match entry_result {
                        Ok(entry) => entries.push(entry.path().display().to_string()),
                        Err(e) => entries.push(format!("error: {}", e)),
                    }
                }
                ToolResult::success(entries.join("\n"))
            },
            _ => return Err(ToolError::invalid_input(format!("Unsupported fs action: {}", action))),
        };
        Ok(result)
    }
}

pub fn register_fs_tool(registry: &mut ToolRegistry) {
    registry.register(FsTool);
}