use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProviderConfig {
    #[serde(default = "default_provider")]
    pub provider: String,
    #[serde(default = "default_model")]
    pub model: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Secrets {
    #[serde(default)]
    pub api_keys: HashMap<String, String>,
    #[serde(default)]
    pub base_urls: HashMap<String, String>,
}

fn default_provider() -> String {
    "openclaude".to_string()
}

fn default_model() -> String {
    "claude-sonnet-4-20250514".to_string()
}

impl Default for ProviderConfig {
    fn default() -> Self {
        Self {
            provider: default_provider(),
            model: default_model(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
    #[serde(default = "default_max_context_tokens")]
    pub max_context_tokens: usize,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_plugins_dir")]
    pub plugins_dir: PathBuf,
    #[serde(default)]
    pub provider: ProviderConfig,
    #[serde(default)]
    pub base_urls: HashMap<String, String>,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_max_context_tokens() -> usize {
    128_000
}

fn default_plugins_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("openclaude")
        .join("plugins")
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_context_tokens: default_max_context_tokens(),
            log_level: default_log_level(),
            plugins_dir: default_plugins_dir(),
            provider: ProviderConfig::default(),
            base_urls: HashMap::new(),
        }
    }
}

/// Returns the path to `secrets.toml`.
pub fn secrets_path() -> Result<PathBuf> {
    Ok(config_base_dir()?.join("secrets.toml"))
}

/// Load API keys and base URLs from secrets.toml.
pub fn load_secrets() -> Result<Secrets> {
    let path = secrets_path()?;
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let secrets: Secrets = toml::from_str(&content)?;
        Ok(secrets)
    } else {
        Ok(Secrets::default())
    }
}

/// Persist secrets to disk.
pub fn save_secrets(secrets: &Secrets) -> Result<()> {
    let path = secrets_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(secrets)?;
    fs::write(&path, content)?;
    Ok(())
}

/// Set an API key for a provider, persisting to disk.
pub fn set_api_key(provider: &str, key: &str) -> Result<()> {
    let mut secrets = load_secrets()?;
    secrets.api_keys.insert(provider.to_lowercase(), key.to_string());
    save_secrets(&secrets)?;
    std::env::set_var(
        format!("{}_API_KEY", provider.to_lowercase()),
        key,
    );
    Ok(())
}

/// Look up a stored API key for a provider.
pub fn get_stored_api_key(provider: &str) -> Option<String> {
    load_secrets()
        .ok()
        .and_then(|s| s.api_keys.get(&provider.to_lowercase()).cloned())
}

/// Set a base URL override for a provider, persisting to config.
pub fn set_base_url(cfg: &mut Config, provider: &str, url: &str) -> Result<()> {
    cfg.base_urls.insert(provider.to_lowercase(), url.to_string());
    save(cfg)
}

/// Get stored base URL for a provider.
pub fn get_stored_base_url(provider: &str) -> Option<String> {
    load_secrets()
        .ok()
        .and_then(|s| s.base_urls.get(&provider.to_lowercase()).cloned())
}

/// Returns the configuration directory path (ensuring it exists).
fn config_base_dir() -> Result<PathBuf> {
    let base = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Cannot locate config directory"))?
        .join("openclaude");
    fs::create_dir_all(&base)?;
    Ok(base)
}

/// Returns the path to `config.toml`.
pub fn config_path() -> Result<PathBuf> {
    Ok(config_base_dir()?.join("config.toml"))
}

/// Load configuration from `$HOME/.config/openclaude/config.toml` or use defaults.
/// If the file nor the directory exist, they are created automatically with defaults.
pub fn load() -> Result<Config> {
    let path = config_path()?;
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let cfg: Config = toml::from_str(&content)?;
        Ok(cfg)
    } else {
        let cfg = Config::default();
        save(&cfg)?;
        Ok(cfg)
    }
}

/// Write the current configuration back to disk.
pub fn save(cfg: &Config) -> Result<()> {
    let path = config_path()?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let content = toml::to_string_pretty(cfg)?;
    fs::write(&path, content)?;
    Ok(())
}

/// Ensure the plugins directory exists, creating it if necessary.
pub fn ensure_plugins_dir(cfg: &Config) -> Result<()> {
    fs::create_dir_all(&cfg.plugins_dir)?;
    Ok(())
}
