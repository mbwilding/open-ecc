use crate::args::{Args, Commands};
use anyhow::Result;
use args::WifiSecurity;
use clap::Parser;
use config::init;
use open_ecc::{contracts::WifiConfig, ecc::Ecc, light::Light};

mod args;
mod config;

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let endpoints = init(&args)?;
    let endpoints = match endpoints {
        Some(e) => e,
        None => return Ok(()),
    };

    let ecc = Ecc::default();
    let lights = endpoints.iter().map(|endpoint| Light::new(&ecc, endpoint));

    match args.command {
        Commands::Brightness { value } => {
            for light in lights {
                _ = light.brightness_set(value).await;
            }
        }
        Commands::Temperature { value } => {
            for light in lights {
                _ = light.temperature_set(value).await;
            }
        }
        Commands::Toggle => {
            for light in lights {
                _ = light.toggle().await;
            }
        }
        Commands::On => {
            for light in lights {
                _ = light.on().await;
            }
        }
        Commands::Off => {
            for light in lights {
                _ = light.off().await;
            }
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
            for endpoint in endpoints {
                _ = ecc.wifi_config(&endpoint, &wifi_config).await;
            }
        }
        _ => {}
    }

    Ok(())
}
