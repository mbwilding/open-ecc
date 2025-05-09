use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct AppConfig {
    pub endpoints: Option<Vec<String>>,
}

pub fn get_config_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "mbwilding", "ecc") {
        let config_path = proj_dirs.config_dir().join("ecc.toml");
        Ok(config_path)
    } else {
        let fallback = PathBuf::from("config.toml");
        Ok(fallback)
    }
}

pub fn load_config(config_path: &PathBuf) -> Result<AppConfig> {
    if config_path.exists() {
        let config_str = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;
        let config: AppConfig =
            toml::from_str(&config_str).with_context(|| "Configuration file is malformed")?;
        Ok(config)
    } else {
        Ok(AppConfig::default())
    }
}

pub fn save_config(config: &AppConfig, config_path: &PathBuf) -> Result<()> {
    let toml_str = toml::to_string(&config).context("Failed to serialize configuration")?;
    fs::create_dir_all(
        config_path
            .parent()
            .ok_or_else(|| anyhow!("Invalid config path"))?,
    )?;
    fs::write(config_path, toml_str).with_context(|| {
        format!(
            "Failed to write configuration file at {}",
            config_path.display()
        )
    })?;
    Ok(())
}
