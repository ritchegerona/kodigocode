use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Application configuration loaded from TOML.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Config {
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
        }
    }
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
