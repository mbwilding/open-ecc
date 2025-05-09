use anyhow::{Ok, Result, anyhow};
use clap::{Parser, Subcommand, ValueEnum};
use config::{AppConfig, get_config_path, load_config, save_config};
use open_ecc::{
    contracts::{LightGet, LightPut, LightsPut, WifiConfig},
    ecc::Ecc,
};
use serde::{Deserialize, Serialize};

mod config;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Set the brightness level
    #[command(visible_alias = "b")]
    Brightness { value: u8 },
    /// Set the temperature
    #[command(visible_alias = "k")]
    Temperature { value: u16 },
    /// Toggle the current state of the light
    #[command(visible_alias = "t")]
    Toggle,
    /// Turn the light on
    #[command(visible_alias = "1")]
    On,
    /// Turn the light off
    #[command(visible_alias = "0")]
    Off,
    /// Set endpoints by providing space seperated IPs or host names
    #[command(visible_alias = "e")]
    Endpoints {
        /// Endpoints to save in config
        endpoints: Vec<String>,
    },
    /// Configure WiFi settings
    #[command(visible_alias = "w")]
    Wifi {
        /// WiFi SSID
        #[arg(long)]
        ssid: String,
        /// Passphrase, if any
        #[arg(long)]
        passphrase: Option<String>,
        /// Security type
        #[arg(long, value_enum)]
        security: WifiSecurity,
        /// Channel [range: 1-14]
        #[arg(long)]
        channel: Option<u8>,
    },
}

#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
enum WifiSecurity {
    None,
    Wpa,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if let Commands::Endpoints { endpoints } = args.command {
        let config_path = get_config_path()?;
        let mut config: AppConfig = load_config(&config_path)?;
        config.endpoints = Some(endpoints);
        save_config(&config, &config_path)?;
        return Ok(());
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

    let ecc = Ecc::default();

    match args.command {
        Commands::Brightness { value } => {
            process_lights(&ecc, &endpoints, |_| LightPut {
                brightness: Some(value),
                ..Default::default()
            })
            .await?;
        }
        Commands::Temperature { value } => {
            process_lights(&ecc, &endpoints, |_| LightPut {
                temperature: Some(value),
                ..Default::default()
            })
            .await?;
        }
        Commands::Toggle => {
            process_lights(&ecc, &endpoints, |light| LightPut {
                on: Some(!light.on),
                ..Default::default()
            })
            .await?;
        }
        Commands::On => {
            process_lights(&ecc, &endpoints, |_| LightPut {
                on: Some(true),
                ..Default::default()
            })
            .await?;
        }
        Commands::Off => {
            process_lights(&ecc, &endpoints, |_| LightPut {
                on: Some(false),
                ..Default::default()
            })
            .await?;
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
                // Discard prevents errors breaking the loop
                _ = ecc.wifi_config(&endpoint, &wifi_config).await;
            }
        }
        _ => {}
    }

    Ok(())
}

async fn process_lights<F>(ecc: &Ecc, endpoints: &[String], f: F) -> Result<()>
where
    F: Fn(LightGet) -> LightPut,
{
    for endpoint in endpoints {
        // This prevents errors breaking the loop
        let _ = async {
            let lights = ecc.lights_get(endpoint).await?;
            let lights_put = lights.lights.into_iter().map(&f).collect::<Vec<_>>();
            ecc.lights_put(endpoint, &LightsPut { lights: lights_put })
                .await?;
            Ok(())
        }
        .await;
    }
    Ok(())
}
