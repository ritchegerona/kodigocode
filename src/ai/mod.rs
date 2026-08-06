use anyhow::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use tokio_stream::Stream;
#[path = "../providers.rs"]
mod providers;

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
    make_provider_with_url(provider, api_key, None)
}

pub fn make_provider_with_url(provider_name: &str, api_key: &str, base_url: Option<&str>) -> Result<Box<dyn AiProvider>> {
    let def = providers::find_provider(provider_name)
        .unwrap_or_else(|| {
            // Fallback: build a generic entry for unknown providers
            providers::ProviderDef {
                name: provider_name.to_string(),
                description: String::new(),
                default_base_url: "https://api.openai.com/v1".into(),
                api_key_env: "API_KEY".into(),
                models: vec![],
            }
        });

    let url = base_url.unwrap_or(&def.default_base_url);
    Ok(Box::new(openai_compat::CompatProvider::new(
        def.name.clone(),
        api_key,
        url.to_string(),
    )))
}