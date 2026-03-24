use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub show_debug_info: bool,
    pub dark_mode: bool,
    pub game_path: String,
    pub use_steam_launch: bool,
    pub launch_args: String,
}

pub const APP_NAME: &str = "dltweaks";

pub fn load_config() -> AppConfig {
    // Try to load config if it exists
    if let Some(path) = get_config_path() {
        if let Ok(content) = fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
    }

    // Otherwise, fallback to default.
    AppConfig::default()
}

pub fn save_config(config: &AppConfig) -> Result<()> {
    let path =
        get_config_path().ok_or_else(|| anyhow::anyhow!("Could not determine config path"))?;

    let parent = path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("No parent directory for config path"))?;

    fs::create_dir_all(parent).context("Failed to create config directory")?;

    let content = toml::to_string_pretty(config).context("Failed to serialize config to TOML")?;

    fs::write(&path, content)
        .with_context(|| format!("Failed to write config to {}", path.display()))?;

    Ok(())
}

fn get_config_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "", APP_NAME).map(|proj| proj.config_dir().join("config.toml"))
}
