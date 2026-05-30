# Open ECC

Cross-platform CLI and API for controlling Elgato Key Lights over their local HTTP API.

## Overview

Open ECC provides two crates:

- **`open_ecc`** - a Rust library for programmatic control of Elgato Key Lights
- **`open_ecc_cli`** - a CLI binary (`ecc`) built on top of the library

Devices are discovered by IP address or hostname and communicated with directly over HTTP on port 9123. No Elgato software or cloud connection is required.

## Features

- Cross-platform (Windows, macOS, Linux, FreeBSD)
- Control brightness, colour temperature, and on/off state
- Toggle lights and push Wi-Fi configuration
- Commands run in parallel across multiple devices
- Persistent endpoint configuration stored in the OS config directory
- Programmatic API for embedding light control in your own applications

## CLI

### Install

```
cargo install open_ecc_cli
```

### Configure endpoints

Before running any command, save the IP addresses or hostnames of your devices:

```
ecc endpoints 192.168.0.50 192.168.0.51
```

### Commands

| Command | Alias | Description |
|---|---|---|
| `brightness <0-100>` | `b` | Set brightness |
| `temperature <2900-7000>` | `k` | Set colour temperature in Kelvin |
| `toggle` | `t` | Toggle on/off state |
| `on` | `1` | Turn on |
| `off` | `0` | Turn off |
| `endpoints <ip...>` | `e` | Save device endpoints |
| `wifi` | `w` | Push Wi-Fi configuration |

Run `ecc --help` or `ecc <command> --help` for full usage details.

### Examples

```
ecc on
ecc brightness 75
ecc temperature 4500
ecc toggle
ecc wifi --ssid MyNetwork --security wpa --passphrase secret
```

## Library

Add `open_ecc` to your `Cargo.toml`:

```toml
open_ecc = "0.0.7"
```

```rust
use open_ecc::{ecc::Ecc, light::Light};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ecc = Ecc::default();
    let light = Light::new(&ecc, "192.168.0.50");
    light.on().await?;
    light.brightness_set(80).await?;
    light.temperature_set(5000).await?;
    Ok(())
}
```

See the [`open_ecc` crate README](crates/open_ecc/README.md) for more detail.
