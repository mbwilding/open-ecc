use crate::args::{Args, Commands};
use anyhow::Result;
use args::WifiSecurity;
use clap::Parser;
use config::init;
use futures::future::join_all;
use open_ecc::{contracts::WifiConfig, ecc::Ecc, light::Light};

mod args;
mod config;

/// Entry point for the `ecc` binary.
///
/// Parses CLI arguments, resolves the configured device endpoints, then
/// dispatches the requested command. All per-device operations are run
/// concurrently using [`join_all`] so that commands complete in parallel
/// rather than sequentially when multiple endpoints are configured.
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let endpoints = init(&args)?;
    let endpoints = match endpoints {
        Some(e) => e,
        // The Endpoints subcommand saves config and exits; nothing more to do.
        None => return Ok(()),
    };

    let ecc = Ecc::default();
    let lights = endpoints.iter().map(|endpoint| Light::new(&ecc, endpoint));

    match args.command {
        Commands::Brightness { value } => {
            join_all(lights.map(|light| async move { light.brightness_set(value).await })).await;
        }
        Commands::Temperature { value } => {
            join_all(lights.map(|light| async move { light.temperature_set(value).await })).await;
        }
        Commands::Toggle => {
            join_all(lights.map(|light| async move { light.toggle().await })).await;
        }
        Commands::On => {
            join_all(lights.map(|light| async move { light.on().await })).await;
        }
        Commands::Off => {
            join_all(lights.map(|light| async move { light.off().await })).await;
        }
        Commands::Wifi {
            ssid,
            passphrase,
            security,
            channel,
        } => {
            let wifi_config = WifiConfig {
                ssid,
                passphrase,
                security_type: match security {
                    WifiSecurity::None => open_ecc::contracts::WifiSecurity::None,
                    WifiSecurity::Wpa => open_ecc::contracts::WifiSecurity::WpaOrWpa2Personal,
                },
                channel,
            };
            join_all(
                endpoints
                    .iter()
                    .map(|endpoint| async { ecc.wifi_config(endpoint, &wifi_config).await }),
            )
            .await;
        }
        _ => {}
    }

    Ok(())
}
