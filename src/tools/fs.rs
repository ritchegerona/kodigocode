use async_trait::async_trait;
use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::tool::Tool;

/// Simple filesystem tool exposing basic file operations.
pub struct FsTool;

#[async_trait]
impl Tool for FsTool {
    fn name(&self) -> &'static str { "fs" }
    fn description(&self) -> &'static str { "Filesystem operations: read, write, create, delete, rename, move, scan" }
    async fn run(&self, args: &[String]) -> Result<String> {
        if args.is_empty() {
            return Ok("Usage: fs <subcommand> [args...]".into());
        }
        match args[0].as_str() {
            "read" => {
                let path = args.get(1).ok_or_else(|| anyhow::anyhow!("read requires <path>"))?;
                let content = fs::read_to_string(path)?;
                Ok(content)
            }
            "write" => {
                let path = args.get(1).ok_or_else(|| anyhow::anyhow!("write requires <path>"))?;
                let data = args.get(2).ok_or_else(|| anyhow::anyhow!("write requires <data>"))?;
                fs::write(path, data)?;
                Ok(format!("Wrote {} bytes to {}", data.len(), path))
            }
            "create" => {
                let path = args.get(1).ok_or_else(|| anyhow::anyhow!("create requires <path>"))?;
                if Path::new(path).exists() {
                    return Err(anyhow::anyhow!("File already exists"));
                }
                fs::write(path, "")?;
                Ok(format!("Created empty file {}", path))
            }
            "delete" => {
                let path = args.get(1).ok_or_else(|| anyhow::anyhow!("delete requires <path>"))?;
                fs::remove_file(path)?;
                Ok(format!("Deleted {}", path))
            }
            "rename" => {
                let src = args.get(1).ok_or_else(|| anyhow::anyhow!("rename requires <src>"))?;
                let dst = args.get(2).ok_or_else(|| anyhow::anyhow!("rename requires <dst>"))?;
                fs::rename(src, dst)?;
                Ok(format!("Renamed {} to {}", src, dst))
            }
            "move" => {
                // alias for rename
                let src = args.get(1).ok_or_else(|| anyhow::anyhow!("move requires <src>"))?;
                let dst = args.get(2).ok_or_else(|| anyhow::anyhow!("move requires <dst>"))?;
                fs::rename(src, dst)?;
                Ok(format!("Moved {} to {}", src, dst))
            }
            "scan" => {
                let dir = args.get(1).ok_or_else(|| anyhow::anyhow!("scan requires <dir>"))?;
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
}

pub fn register_fs_tool(registry: &mut crate::tool::ToolRegistry) {
    registry.register(FsTool);
}
