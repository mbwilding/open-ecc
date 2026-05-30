use clap::{Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};

/// Top-level CLI arguments parsed by clap.
#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Args {
    /// The subcommand to execute.
    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI subcommands.
#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Set the brightness level (range 0-100).
    #[command(visible_alias = "b")]
    Brightness { value: u8 },

    /// Set the colour temperature in Kelvin (range 2900-7000).
    #[command(visible_alias = "k")]
    Temperature { value: u16 },

    /// Toggle the current on/off state of all configured lights.
    #[command(visible_alias = "t")]
    Toggle,

    /// Turn all configured lights on.
    #[command(visible_alias = "1")]
    On,

    /// Turn all configured lights off.
    #[command(visible_alias = "0")]
    Off,

    /// Save a list of device endpoints (IP addresses or hostnames) to the
    /// configuration file.
    ///
    /// These endpoints are used by all other subcommands. Existing endpoints
    /// are replaced entirely.
    ///
    /// Example: `ecc endpoints 192.168.0.50 192.168.0.51`
    #[command(visible_alias = "e")]
    Endpoints {
        /// Space-separated IP addresses or hostnames to save.
        endpoints: Vec<String>,
    },

    /// Push a new Wi-Fi configuration to all configured devices.
    #[command(visible_alias = "w")]
    Wifi {
        /// Wi-Fi network SSID.
        #[arg(long)]
        ssid: String,
        /// Network passphrase. Required for WPA/WPA2 networks.
        #[arg(long)]
        passphrase: Option<String>,
        /// Security type.
        #[arg(long, value_enum)]
        security: WifiSecurity,
        /// Wi-Fi channel number (range 1-14).
        #[arg(long)]
        channel: Option<u8>,
    },
}

/// Wi-Fi security types available at the CLI.
///
/// Maps to [`open_ecc::contracts::WifiSecurity`] in the library.
#[derive(ValueEnum, Debug, Clone, Serialize, Deserialize)]
pub(crate) enum WifiSecurity {
    /// Open network, no authentication.
    None,
    /// WPA or WPA2 Personal (PSK).
    Wpa,
}
