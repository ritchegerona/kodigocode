use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// High-level model family used for shared identity affordances across clients.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ModelFamily {
    DeepSeek,
    Anthropic,
    OpenAI,
    Google,
    Meta,
    Mistral,
    Qwen,
    Grok,
    Cohere,
    GptOss,
    Inferencer,
}

impl ModelFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeepSeek => "DeepSeek",
            Self::Anthropic => "Anthropic",
            Self::OpenAI => "OpenAI",
            Self::Google => "Google",
            Self::Meta => "Meta",
            Self::Mistral => "Mistral",
            Self::Qwen => "Qwen",
            Self::Grok => "Grok",
            Self::Cohere => "Cohere",
            Self::GptOss => "GPT-OSS",
            Self::Inferencer => "Inferencer",
        }
    }
}

impl std::fmt::Display for ModelFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Metadata for a single model entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub family: ModelFamily,
    pub aliases: Vec<String>,
    pub supports_tools: bool,
    pub supports_reasoning: bool,
}

/// The result of resolving a user-requested model name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelResolution {
    pub requested: Option<String>,
    pub resolved: ModelInfo,
    pub used_fallback: bool,
    pub fallback_chain: Vec<String>,
}

/// A registry of supported models and their aliases.
#[derive(Debug, Clone)]
pub struct ModelRegistry {
    models: Vec<ModelInfo>,
    alias_map: HashMap<String, usize>,
}

fn normalize(name: &str) -> String {
    name.to_lowercase()
}

impl Default for ModelRegistry {
    fn default() -> Self {
        let models = vec![
            // DeepSeek models
            ModelInfo {
                id: "deepseek-chat".to_string(),
                family: ModelFamily::DeepSeek,
                aliases: vec!["deepseek-v3".to_string(), "deepseek-v3.2".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            ModelInfo {
                id: "deepseek-reasoner".to_string(),
                family: ModelFamily::DeepSeek,
                aliases: vec!["deepseek-r1".to_string()],
                supports_tools: true,
                supports_reasoning: true,
            },
            // Anthropic models
            ModelInfo {
                id: "claude-sonnet-4-20250514".to_string(),
                family: ModelFamily::Anthropic,
                aliases: vec!["claude-sonnet".to_string(), "sonnet".to_string()],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "claude-opus-4-20250514".to_string(),
                family: ModelFamily::Anthropic,
                aliases: vec!["claude-opus".to_string(), "opus".to_string()],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "claude-haiku-3-5-20241022".to_string(),
                family: ModelFamily::Anthropic,
                aliases: vec!["claude-haiku".to_string(), "haiku".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            // OpenAI models
            ModelInfo {
                id: "gpt-4o".to_string(),
                family: ModelFamily::OpenAI,
                aliases: vec!["gpt4o".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            ModelInfo {
                id: "gpt-4o-mini".to_string(),
                family: ModelFamily::OpenAI,
                aliases: vec!["gpt4o-mini".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            ModelInfo {
                id: "gpt-4.1".to_string(),
                family: ModelFamily::OpenAI,
                aliases: vec!["gpt41".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            ModelInfo {
                id: "o3".to_string(),
                family: ModelFamily::OpenAI,
                aliases: vec![],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "o4-mini".to_string(),
                family: ModelFamily::OpenAI,
                aliases: vec!["o4".to_string()],
                supports_tools: true,
                supports_reasoning: true,
            },
            // Google models
            ModelInfo {
                id: "gemini-2.5-pro".to_string(),
                family: ModelFamily::Google,
                aliases: vec!["gemini-pro".to_string()],
                supports_tools: true,
                supports_reasoning: true,
            },
            ModelInfo {
                id: "gemini-2.5-flash".to_string(),
                family: ModelFamily::Google,
                aliases: vec!["gemini-flash".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            // Mistral models
            ModelInfo {
                id: "mistral-large-latest".to_string(),
                family: ModelFamily::Mistral,
                aliases: vec!["mistral-large".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            ModelInfo {
                id: "codestral-latest".to_string(),
                family: ModelFamily::Mistral,
                aliases: vec!["codestral".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            // Meta models
            ModelInfo {
                id: "llama-3.3-70b-versatile".to_string(),
                family: ModelFamily::Meta,
                aliases: vec!["llama3.3".to_string(), "llama-3.3".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            ModelInfo {
                id: "meta/llama3-70b-instruct".to_string(),
                family: ModelFamily::Meta,
                aliases: vec!["llama3-70b".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
            // Qwen models
            ModelInfo {
                id: "qwen2.5-coder:32b".to_string(),
                family: ModelFamily::Qwen,
                aliases: vec!["qwen-coder".to_string(), "qwen2.5-coder".to_string()],
                supports_tools: true,
                supports_reasoning: false,
            },
        ];
        Self::new(models)
    }
}

impl ModelRegistry {
    pub fn new(models: Vec<ModelInfo>) -> Self {
        let mut alias_map = HashMap::new();
        for (idx, model) in models.iter().enumerate() {
            alias_map.entry(normalize(&model.id)).or_insert(idx);
            for alias in &model.aliases {
                alias_map.entry(normalize(alias)).or_insert(idx);
            }
        }
        Self { models, alias_map }
    }

    /// Returns all models in the registry.
    pub fn list(&self) -> Vec<ModelInfo> {
        self.models.clone()
    }

    /// Returns models grouped by family.
    pub fn list_by_family(&self) -> HashMap<ModelFamily, Vec<ModelInfo>> {
        let mut grouped: HashMap<ModelFamily, Vec<ModelInfo>> = HashMap::new();
        for model in &self.models {
            grouped.entry(model.family).or_default().push(model.clone());
        }
        grouped
    }

    /// Resolves a user-requested model name to a ModelInfo.
    pub fn resolve(
        &self,
        requested: Option<&str>,
        family_hint: Option<ModelFamily>,
    ) -> ModelResolution {
        let mut fallback_chain = Vec::new();

        if let Some(name) = requested {
            fallback_chain.push(format!("requested:{}", name));

            // Try exact family + name match first
            if let Some(family) = family_hint {
                if let Some(model) = self.models.iter().find(|m| {
                    m.family == family
                        && (normalize(&m.id) == normalize(name)
                            || m.aliases
                                .iter()
                                .any(|a| normalize(a) == normalize(name)))
                }) {
                    return ModelResolution {
                        requested: Some(name.to_string()),
                        resolved: model.clone(),
                        used_fallback: false,
                        fallback_chain,
                    };
                }
            }

            // Try alias map
            let canon = normalize(name);
            if let Some(&idx) = self.alias_map.get(&canon) {
                return ModelResolution {
                    requested: Some(name.to_string()),
                    resolved: self.models[idx].clone(),
                    used_fallback: false,
                    fallback_chain,
                };
            }

            fallback_chain.push("alias_not_found".to_string());
        }

        // Default to the first model in the hinted family
        if let Some(family) = family_hint {
            if let Some(model) = self.models.iter().find(|m| m.family == family) {
                fallback_chain.push(format!("family_default:{}", family));
                return ModelResolution {
                    requested: requested.map(String::from),
                    resolved: model.clone(),
                    used_fallback: true,
                    fallback_chain,
                };
            }
        }

        // Last resort: first model
        if let Some(first) = self.models.first() {
            fallback_chain.push("global_default".to_string());
            return ModelResolution {
                requested: requested.map(String::from),
                resolved: first.clone(),
                used_fallback: true,
                fallback_chain,
            };
        }

        // Return a sensible default even when empty
        ModelResolution {
            requested: requested.map(String::from),
            resolved: ModelInfo {
                id: "gpt-4o".to_string(),
                family: ModelFamily::OpenAI,
                aliases: vec![],
                supports_tools: true,
                supports_reasoning: false,
            },
            used_fallback: true,
            fallback_chain,
        }
    }
}