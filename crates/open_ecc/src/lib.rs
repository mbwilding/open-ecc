//! # open_ecc
//!
//! Unofficial Rust API for Elgato Key Lights.
//!
//! Communicates with devices over their local HTTP API on port 9123.
//! All temperature values exposed through this crate are in Kelvin;
//! conversion to and from the device's internal unit is handled
//! transparently by the serialisation layer.
//!
//! ## Quick start
//!
//! ```no_run
//! use open_ecc::{ecc::Ecc, light::Light};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let ecc = Ecc::default();
//!     let light = Light::new(&ecc, "192.168.0.50");
//!     light.on().await?;
//!     light.brightness_set(80).await?;
//!     light.temperature_set(5000).await?;
//!     Ok(())
//! }
//! ```

/// Serialisable types that mirror the Elgato HTTP API JSON schema.
pub mod contracts;

/// Low-level HTTP client for the Elgato device API.
pub mod ecc;

/// Internal encryption and unit-conversion helpers.
pub(crate) mod helpers;

/// High-level, ergonomic wrapper around a single light endpoint.
pub mod light;

/// Custom serde handlers for the wire format used by the device API.
pub(crate) mod serialization;
