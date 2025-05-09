use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
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
pub(crate) enum WifiSecurity {
    None,
    Wpa,
}
