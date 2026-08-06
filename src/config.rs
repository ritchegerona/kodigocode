use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

/// Application configuration loaded from TOML.
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// Maximum number of tokens in the context window.
    #[serde(default = "default_max_context_tokens")]
    pub max_context_tokens: usize,
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_plugins_dir")]
    pub plugins_dir: PathBuf,
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_max_context_tokens() -> usize {
    // Default maximum tokens for context window; can be overridden via config.
    128_000
}


fn default_plugins_dir() -> PathBuf {
    // Default directory for plugins.

    // Default maximum tokens for the OpenClaude context window.
    // This can be overridden via configuration.

    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from(".") )
        .join("openclaude/plugins")
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_context_tokens: default_max_context_tokens(),
            log_level: default_log_level(),
            plugins_dir: default_plugins_dir(),
        }
    }
}

/// Load configuration from `$HOME/.config/openclaude/config.toml` or use defaults.
pub fn load() -> Result<Config> {
    let mut path = dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Cannot locate config dir"))?;
    path.push("openclaude");
    path.push("config.toml");
    if path.exists() {
        let content = fs::read_to_string(&path)?;
        let cfg: Config = toml::from_str(&content)?;
        Ok(cfg)
    } else {
        Ok(Config::default())
    }
}
