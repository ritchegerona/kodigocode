use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderDef {
    pub name: String,
    pub description: String,
    pub default_base_url: String,
    pub api_key_env: String,
    pub models: Vec<ModelDef>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDef {
    pub name: String,
    pub description: String,
}

pub fn all_providers() -> Vec<ProviderDef> {
    vec![
        ProviderDef {
            name: "openclaude".into(),
            description: "Anthropic Claude models (OpenAI-compatible API)".into(),
            default_base_url: "https://api.anthropic.com/v1".into(),
            api_key_env: "CLAUDE_API_KEY".into(),
            models: vec![
                ModelDef { name: "claude-sonnet-4-20250514".into(), description: "Claude Sonnet 4 — balanced performance".into() },
                ModelDef { name: "claude-opus-4-20250514".into(), description: "Claude Opus 4 — most powerful".into() },
                ModelDef { name: "claude-sonnet-3-7-20250219".into(), description: "Claude 3.7 Sonnet — extended thinking".into() },
                ModelDef { name: "claude-haiku-3-5-20241022".into(), description: "Claude 3.5 Haiku — fast & efficient".into() },
                ModelDef { name: "claude-opus-3-20240229".into(), description: "Claude 3 Opus".into() },
            ],
        },
        ProviderDef {
            name: "openai".into(),
            description: "OpenAI GPT models".into(),
            default_base_url: "https://api.openai.com/v1".into(),
            api_key_env: "OPENAI_API_KEY".into(),
            models: vec![
                ModelDef { name: "gpt-4o".into(), description: "GPT-4o — multimodal flagship".into() },
                ModelDef { name: "gpt-4o-mini".into(), description: "GPT-4o Mini — fast & affordable".into() },
                ModelDef { name: "gpt-4.1".into(), description: "GPT-4.1 — latest generation".into() },
                ModelDef { name: "gpt-4.1-mini".into(), description: "GPT-4.1 Mini — compact & fast".into() },
                ModelDef { name: "gpt-4-turbo".into(), description: "GPT-4 Turbo — 128K context".into() },
                ModelDef { name: "o3".into(), description: "o3 — reasoning model".into() },
                ModelDef { name: "o4-mini".into(), description: "o4-mini — fast reasoning".into() },
            ],
        },
        ProviderDef {
            name: "deepseek".into(),
            description: "DeepSeek AI models".into(),
            default_base_url: "https://api.deepseek.com/v1".into(),
            api_key_env: "DEEPSEEK_API_KEY".into(),
            models: vec![
                ModelDef { name: "deepseek-chat".into(), description: "DeepSeek-V3 — latest chat".into() },
                ModelDef { name: "deepseek-reasoner".into(), description: "DeepSeek-R1 — reasoning".into() },
            ],
        },
        ProviderDef {
            name: "vertex".into(),
            description: "Google Vertex AI / Gemini models".into(),
            default_base_url: "https://generativelanguage.googleapis.com/v1beta".into(),
            api_key_env: "GEMINI_API_KEY".into(),
            models: vec![
                ModelDef { name: "gemini-2.5-pro".into(), description: "Gemini 2.5 Pro — most capable".into() },
                ModelDef { name: "gemini-2.5-flash".into(), description: "Gemini 2.5 Flash — fast".into() },
                ModelDef { name: "gemini-2.0-flash".into(), description: "Gemini 2.0 Flash".into() },
                ModelDef { name: "gemini-1.5-pro".into(), description: "Gemini 1.5 Pro — 1M context".into() },
            ],
        },
        ProviderDef {
            name: "nvidia".into(),
            description: "NVIDIA NIM inference platform".into(),
            default_base_url: "https://integrate.api.nvidia.com/v1".into(),
            api_key_env: "NVIDIA_API_KEY".into(),
            models: vec![
                ModelDef { name: "meta/llama3-70b-instruct".into(), description: "Llama 3 70B Instruct".into() },
                ModelDef { name: "meta/llama3-8b-instruct".into(), description: "Llama 3 8B Instruct".into() },
                ModelDef { name: "meta/llama-3.1-405b-instruct".into(), description: "Llama 3.1 405B".into() },
                ModelDef { name: "meta/llama-3.1-70b-instruct".into(), description: "Llama 3.1 70B".into() },
                ModelDef { name: "meta/llama-3.1-8b-instruct".into(), description: "Llama 3.1 8B".into() },
                ModelDef { name: "meta/llama-3.2-90b-vision-instruct".into(), description: "Llama 3.2 90B Vision".into() },
                ModelDef { name: "microsoft/phi-4".into(), description: "Phi-4".into() },
                ModelDef { name: "mistralai/mixtral-8x22b-instruct-v0.1".into(), description: "Mixtral 8x22B".into() },
                ModelDef { name: "mistralai/mistral-large-2-instruct".into(), description: "Mistral Large 2".into() },
                ModelDef { name: "google/gemma-2-27b-it".into(), description: "Gemma 2 27B".into() },
                ModelDef { name: "deepseek-ai/deepseek-r1".into(), description: "DeepSeek-R1".into() },
                ModelDef { name: "deepseek-ai/deepseek-v3".into(), description: "DeepSeek-V3".into() },
            ],
        },
        ProviderDef {
            name: "groq".into(),
            description: "Groq — fast inference (LPU)".into(),
            default_base_url: "https://api.groq.com/openai/v1".into(),
            api_key_env: "GROQ_API_KEY".into(),
            models: vec![
                ModelDef { name: "llama-3.3-70b-versatile".into(), description: "Llama 3.3 70B".into() },
                ModelDef { name: "llama-3.1-8b-instant".into(), description: "Llama 3.1 8B Fast".into() },
                ModelDef { name: "mixtral-8x7b-32768".into(), description: "Mixtral 8x7B".into() },
                ModelDef { name: "gemma2-9b-it".into(), description: "Gemma 2 9B".into() },
                ModelDef { name: "deepseek-r1-distill-llama-70b".into(), description: "R1 Distill Llama 70B".into() },
            ],
        },
        ProviderDef {
            name: "mistral".into(),
            description: "Mistral AI models".into(),
            default_base_url: "https://api.mistral.ai/v1".into(),
            api_key_env: "MISTRAL_API_KEY".into(),
            models: vec![
                ModelDef { name: "mistral-large-latest".into(), description: "Mistral Large".into() },
                ModelDef { name: "mistral-medium-latest".into(), description: "Mistral Medium".into() },
                ModelDef { name: "mistral-small-latest".into(), description: "Mistral Small".into() },
                ModelDef { name: "pixtral-large-latest".into(), description: "Pixtral Large — vision".into() },
                ModelDef { name: "codestral-latest".into(), description: "Codestral — code generation".into() },
                ModelDef { name: "ministral-8b-latest".into(), description: "Ministral 8B".into() },
            ],
        },
        ProviderDef {
            name: "ollama".into(),
            description: "Ollama — local LLM server".into(),
            default_base_url: "http://localhost:11434/v1".into(),
            api_key_env: "OLLAMA_API_KEY".into(),
            models: vec![
                ModelDef { name: "llama3.2".into(), description: "Llama 3.2".into() },
                ModelDef { name: "llama3.1:70b".into(), description: "Llama 3.1 70B".into() },
                ModelDef { name: "codellama".into(), description: "Code Llama".into() },
                ModelDef { name: "mistral".into(), description: "Mistral".into() },
                ModelDef { name: "mixtral:8x7b".into(), description: "Mixtral 8x7B".into() },
                ModelDef { name: "gemma2:27b".into(), description: "Gemma 2 27B".into() },
                ModelDef { name: "phi4".into(), description: "Phi-4".into() },
                ModelDef { name: "qwen2.5-coder:32b".into(), description: "Qwen 2.5 Coder 32B".into() },
            ],
        },
        ProviderDef {
            name: "openrouter".into(),
            description: "OpenRouter — model aggregator".into(),
            default_base_url: "https://openrouter.ai/api/v1".into(),
            api_key_env: "OPENROUTER_API_KEY".into(),
            models: vec![
                ModelDef { name: "openai/gpt-4o".into(), description: "GPT-4o via OpenRouter".into() },
                ModelDef { name: "anthropic/claude-sonnet-4".into(), description: "Claude Sonnet 4".into() },
                ModelDef { name: "google/gemini-2.5-pro".into(), description: "Gemini 2.5 Pro".into() },
                ModelDef { name: "meta-llama/llama-3.3-70b-instruct".into(), description: "Llama 3.3 70B".into() },
                ModelDef { name: "deepseek/deepseek-r1".into(), description: "DeepSeek R1".into() },
            ],
        },
        ProviderDef {
            name: "lmstudio".into(),
            description: "LM Studio — local LLM server".into(),
            default_base_url: "http://localhost:1234/v1".into(),
            api_key_env: "LMSTUDIO_API_KEY".into(),
            models: vec![
                ModelDef { name: "local-model".into(), description: "Currently loaded model".into() },
            ],
        },
    ]
}

/// Find a provider by name (case-insensitive)
pub fn find_provider(name: &str) -> Option<ProviderDef> {
    let lower = name.to_lowercase();
    all_providers().into_iter().find(|p| p.name.to_lowercase() == lower)
}

/// Get the API key for a provider from environment
pub fn get_api_key(provider_name: &str) -> Option<String> {
    let provider = find_provider(provider_name)?;
    // Check provider-specific env var first
    std::env::var(&provider.api_key_env).ok()
        // Fall back to generic vars
        .or_else(|| std::env::var("API_KEY").ok())
        .or_else(|| std::env::var("OPENAI_API_KEY").ok())
}