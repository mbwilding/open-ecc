use anyhow::{Result, anyhow};
use clap::{Parser, Subcommand};
use config::{AppConfig, get_config_path, load_config, save_config};
use open_ecc::{
    api::Ecc,
    contracts::{LightGet, LightPut, LightsPut},
};

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
    Brightness {
        #[arg(short, long)]
        value: u8,
    },
    /// Set the temperature
    Temperature {
        #[arg(short, long)]
        value: u16,
    },
    /// Toggle the current state of the light
    Toggle,
    /// Turn the light on
    On,
    /// Turn the light off
    Off,
    /// Set endpoints by providing space seperated IPs or host names
    Endpoints {
        /// Endpoints to save in config
        endpoints: Vec<String>,
    },
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
                on: Some(light.on ^ 1),
                ..Default::default()
            })
            .await?;
        }
        Commands::On => {
            process_lights(&ecc, &endpoints, |_| LightPut {
                on: Some(1),
                ..Default::default()
            })
            .await?;
        }
        Commands::Off => {
            process_lights(&ecc, &endpoints, |_| LightPut {
                on: Some(0),
                ..Default::default()
            })
            .await?;
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
        let lights = ecc.lights_get(endpoint).await?;
        let lights_put = lights.lights.into_iter().map(&f).collect::<Vec<_>>();
        ecc.lights_put(endpoint, &LightsPut { lights: lights_put })
            .await?;
    }
    Ok(())
}
