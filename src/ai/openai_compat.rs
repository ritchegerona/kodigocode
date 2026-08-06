use anyhow::Result;
use async_trait::async_trait;
use reqwest::Client;
use serde_json::json;
use tokio_stream::StreamExt;

use super::{AiMessage, AiProvider, StreamDelta, StreamOutput, ToolDef};

pub struct CompatProvider {
    name: String,
    client: Client,
    api_key: String,
    base_url: String,
}

impl CompatProvider {
    pub fn new(name: String, api_key: &str, base_url: String) -> Self {
        Self {
            name,
            client: Client::new(),
            api_key: api_key.to_string(),
            base_url,
        }
    }

    fn build_body(&self, model: &str, messages: &[AiMessage], tools: &[ToolDef]) -> serde_json::Value {
        let mut msgs: Vec<serde_json::Value> = vec![];

        for m in messages {
            msgs.push(json!({
                "role": m.role,
                "content": m.content,
            }));
        }

        let mut body = json!({
            "model": model,
            "messages": msgs,
        });

        if !tools.is_empty() {
            let tl: Vec<serde_json::Value> = tools
                .iter()
                .map(|t| {
                    json!({
                        "type": "function",
                        "function": {
                            "name": t.name,
                            "description": t.description,
                            "parameters": t.parameters,
                        }
                    })
                })
                .collect();
            body["tools"] = json!(tl);
        }

        body
    }

    fn auth_header(&self) -> (&str, String) {
        match self.name.as_str() {
            "openclaude" => ("x-api-key", self.api_key.clone()),
            _ => ("Authorization", format!("Bearer {}", self.api_key)),
        }
    }
}

#[async_trait]
impl AiProvider for CompatProvider {
    fn provider_name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }

    async fn chat(
        &self,
        model: &str,
        messages: &[AiMessage],
        system: Option<&str>,
        tools: &[ToolDef],
    ) -> Result<String> {
        let body = self.build_body(model, messages, tools);
        let url = format!("{}/chat/completions", self.base_url);
        let (auth_name, auth_value) = self.auth_header();
        let resp = self
            .client
            .post(&url)
            .header(auth_name, auth_value)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let data: serde_json::Value = resp.json().await?;
        Ok(data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .into())
    }

    async fn stream_chat(
        &self,
        model: &str,
        messages: &[AiMessage],
        system: Option<&str>,
        tools: &[ToolDef],
    ) -> Result<StreamOutput> {
        let mut body = self.build_body(model, messages, tools);
        body["stream"] = json!(true);

        let url = format!("{}/chat/completions", self.base_url);
        let (auth_name, auth_value) = self.auth_header();
        let resp = self
            .client
            .post(&url)
            .header(auth_name, auth_value)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        let mut byte_stream = resp.bytes_stream();
        let mut buf = String::new();

        let stream = async_stream::stream! {
            loop {
                match byte_stream.next().await {
                    Some(Ok(bytes)) => {
                        let text = String::from_utf8_lossy(&bytes);
                        buf.push_str(&text);
                        while let Some(pos) = buf.find('\n') {
                            let line = buf[..pos].trim().to_string();
                            buf = buf[pos+1..].to_string();
                            if line.is_empty() || !line.starts_with("data: ") {
                                continue;
                            }
                            let data = &line[6..];
                            if data == "[DONE]" {
                                yield Ok(StreamDelta::Stop);
                                return;
                            }
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(data) {
                            // Finish reason
                            if parsed["choices"][0]["finish_reason"].as_str() == Some("stop") {
                                yield Ok(StreamDelta::Stop);
                                continue;
                            }
                            // Tool calls detection
                            if let Some(tool_calls) = parsed["choices"][0]["delta"]["tool_calls"].as_array() {
                                for tc in tool_calls {
                                    let id = tc["id"].as_str().unwrap_or_default().to_string();
                                    let name = tc["function"]["name"].as_str().unwrap_or_default().to_string();
                                    let args = tc["function"]["arguments"].clone();
                                    let args_opt = if args.is_null() { None } else { Some(args) };
                                    yield Ok(StreamDelta::ToolUseStart { id, name, arguments: args_opt });
                                }
                                continue;
                            }
                            // Normal content
                            if let Some(c) = parsed["choices"][0]["delta"]["content"].as_str() {
                                yield Ok(StreamDelta::Text(c.to_string()));
                            }
                            if let Some(c) = parsed["choices"][0]["delta"]["reasoning_content"].as_str() {
                                yield Ok(StreamDelta::Reasoning(c.to_string()));
                            }
                        }
                        }
                    }
                    Some(Err(e)) => {
                        yield Err(anyhow::anyhow!("stream read error: {}", e));
                        return;
                    }
                    None => break,
                }
            }
        };

        Ok(Box::pin(stream))
    }
}