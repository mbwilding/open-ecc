# open_ecc

Unofficial Rust API for controlling Elgato Key Lights over their local HTTP API.

## Overview

`open_ecc` communicates with Elgato Key Lights directly on your local network over HTTP (port 9123). No Elgato software or cloud connection is required.

The crate exposes two levels of API:

- **`Light`** - high-level wrapper for common operations on a single device
- **`Ecc`** - low-level HTTP client giving direct access to every device endpoint

All temperature values are in Kelvin throughout the public API. Conversion to and from the device's internal unit is handled transparently.

## Usage

Add the crate to your `Cargo.toml`:

```toml
open_ecc = "0.0.7"
```

### High-level API

```rust
use open_ecc::{ecc::Ecc, light::Light};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ecc = Ecc::default();
    let light = Light::new(&ecc, "192.168.0.50");

    light.on().await?;
    light.brightness_set(80).await?;
    light.temperature_set(5000).await?;

    println!("Brightness: {}", light.brightness_get().await?);
    Ok(())
}
```

### Controlling multiple lights in parallel

```rust
use futures::future::join_all;
use open_ecc::{ecc::Ecc, light::Light};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ecc = Ecc::default();
    let endpoints = vec!["192.168.0.50", "192.168.0.51"];
    let lights: Vec<_> = endpoints.iter().map(|ep| Light::new(&ecc, ep)).collect();

    join_all(lights.iter().map(|l| async { l.on().await })).await;
    Ok(())
}
```

### Low-level API

```rust
use open_ecc::{ecc::Ecc, contracts::AccessoryInfoPut};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ecc = Ecc::default();
    let info = ecc.accessory_info_get("192.168.0.50").await?;
    println!("{} - firmware {}", info.product_name, info.firmware_version);

    ecc.accessory_info_put("192.168.0.50", &AccessoryInfoPut {
        display_name: Some("Key Light Left".to_string()),
    }).await?;
    Ok(())
}
```

## Supported operations

| Method | Description |
|---|---|
| `Light::on` / `off` / `toggle` | Control on/off state |
| `Light::brightness_get` / `brightness_set` | Brightness 0-100 |
| `Light::temperature_get` / `temperature_set` | Colour temperature 2900-7000 K |
| `Ecc::lights_get` / `lights_put` | Raw light state GET/PUT |
| `Ecc::lights_settings_get` / `lights_settings_put` | Persistent device settings |
| `Ecc::accessory_info_get` / `accessory_info_put` | Device info and display name |
| `Ecc::identify` | Flash the light to locate it |
| `Ecc::wifi_config` | Push encrypted Wi-Fi credentials |
