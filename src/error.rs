use thiserror::Error;

/// Central application error type.
#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Tool execution error: {0}")]
    Tool(#[from] anyhow::Error),

    #[error("Unknown tool: {0}")]
    UnknownTool(String),

    #[error("Unexpected error: {0}")]
    Other(String),
}
