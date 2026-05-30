use anyhow::{Context, Result, anyhow};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::{Args, Commands};

/// Persistent CLI configuration stored as TOML.
#[derive(Serialize, Deserialize, Default)]
pub(crate) struct AppConfig {
    /// List of device endpoints (IP addresses or hostnames) to target.
    /// `None` means no endpoints have been saved yet.
    pub endpoints: Option<Vec<String>>,
}

/// Resolve the path to the TOML configuration file.
///
/// Uses the OS-appropriate user configuration directory via the
/// [`directories`] crate (`com.mbwilding.ecc`). Falls back to
/// `config.toml` in the current working directory if no project
/// directory can be determined.
pub(crate) fn get_config_path() -> Result<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "mbwilding", "ecc") {
        let config_path = proj_dirs.config_dir().join("ecc.toml");
        Ok(config_path)
    } else {
        let fallback = PathBuf::from("config.toml");
        Ok(fallback)
    }
}

/// Load the [`AppConfig`] from `config_path`.
///
/// Returns a default (empty) config if the file does not yet exist.
/// Returns an error if the file exists but cannot be read or parsed.
pub(crate) fn load_config(config_path: &PathBuf) -> Result<AppConfig> {
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

/// Serialise `config` and write it to `config_path`.
///
/// Creates any missing parent directories. Returns an error if
/// serialisation or the file write fails.
pub(crate) fn save_config(config: &AppConfig, config_path: &PathBuf) -> Result<()> {
    let toml_str = toml::to_string(&config).context("Failed to serialise configuration")?;
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

/// Initialise the CLI session from `args`.
///
/// - If the command is [`Commands::Endpoints`], the provided endpoints are
///   saved to the configuration file and `Ok(None)` is returned, signalling
///   to the caller that no further action is needed.
/// - For all other commands, the saved endpoints are loaded from the
///   configuration file and returned as `Ok(Some(endpoints))`.
///
/// Returns an error if no endpoints have been configured yet or if the
/// configuration file cannot be read or written.
pub(crate) fn init(args: &Args) -> Result<Option<Vec<String>>> {
    if let Commands::Endpoints { endpoints } = &args.command {
        let config_path = get_config_path()?;
        let mut config = load_config(&config_path)?;
        config.endpoints = Some(endpoints.clone());
        save_config(&config, &config_path)?;
        return Ok(None);
    }
    let config_path = get_config_path()?;
    let config: AppConfig = load_config(&config_path)?;
    let endpoints = config.endpoints.ok_or_else(|| {
        anyhow!(
            "No endpoints defined in the configuration\n\
            Please set endpoints using command: ecc endpoints\n\
            For example: ecc endpoints 192.168.0.50 192.168.0.51"
        )
    })?;
    Ok(Some(endpoints))
}
