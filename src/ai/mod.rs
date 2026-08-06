use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDef {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone)]
pub enum StreamDelta {
    Text(String),
    Reasoning(String),
    // Tool call start includes optional arguments (as raw JSON)
    ToolUseStart { id: String, name: String, arguments: Option<serde_json::Value> },
    ToolUseEnd { id: String },
    Stop,
}

pub type StreamOutput = Pin<Box<dyn Stream<Item = Result<StreamDelta>> + Send>>;

#[async_trait]
pub trait AiProvider: Send + Sync {
    fn provider_name(&self) -> &'static str;

    async fn chat(
        &self,
        model: &str,
        messages: &[AiMessage],
        system: Option<&str>,
        tools: &[ToolDef],
    ) -> Result<String>;

    async fn stream_chat(
        &self,
        model: &str,
        messages: &[AiMessage],
        system: Option<&str>,
        tools: &[ToolDef],
    ) -> Result<StreamOutput>;
}

pub mod openai_compat;

pub fn make_provider(provider: &str, api_key: &str) -> Result<Box<dyn AiProvider>> {
    match provider.to_lowercase().as_str() {
        "openclaude" | "claude" | "anthropic" => {
            Ok(Box::new(openai_compat::CompatProvider::new(
                "openclaude".into(),
                api_key,
                "https://api.anthropic.com/v1".into(),
            )))
        }
        "openai" | "gpt" => Ok(Box::new(openai_compat::CompatProvider::new(
            "openai".into(),
            api_key,
            "https://api.openai.com/v1".into(),
        ))),
        "deepseek" => Ok(Box::new(openai_compat::CompatProvider::new(
            "deepseek".into(),
            api_key,
            "https://api.deepseek.com/v1".into(),
        ))),
"vertex" | "gemini" => Ok(Box::new(openai_compat::CompatProvider::new(
                "vertex".into(),
                api_key,
                "https://generativelanguage.googleapis.com/v1beta".into(),
            ))),
            "nvidia" | "nim" => Ok(Box::new(openai_compat::CompatProvider::new(
                "nvidia".into(),
                api_key,
                "https://integrate.api.nvidia.com/v1".into(),
            ))),
        _ => Err(anyhow::anyhow!("unsupported provider '{}'", provider)),
    }
}