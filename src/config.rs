use anyhow::{Context, Result};
use dirs::config_dir;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub github_token: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self { github_token: None }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        if !config_path.exists() {
            return Ok(Config::default());
        }
        let config_str = fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read config file: {:?}", config_path))?;
        let config = serde_json::from_str(&config_str)
            .with_context(|| "Failed to parse config file as JSON")?;

        Ok(config)
    }
    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create a config directory: {:?}", parent))?;
        }
        let config_str = serde_json::to_string_pretty(self)
            .with_context(|| "Failed to serialize config to JSON")?;
        fs::write(&config_path, config_str)
            .with_context(|| format!("Failed to write config file: {:?}", config_path))?;

        Ok(())
    }
    pub fn set_github_token(&mut self, token: String) -> Result<()> {
        self.github_token = Some(token);
        self.save()
    }
}

fn get_config_path() -> Result<PathBuf> {
    let mut config_dir = config_dir().with_context(|| "Failed to determine config directory")?;

    config_dir.push("git-issue-flow");
    config_dir.push("config.json");

    Ok(config_dir)
}
