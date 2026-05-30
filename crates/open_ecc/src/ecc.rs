use crate::{
    contracts::{
        AccessoryInfoGet, AccessoryInfoPut, LightsGet, LightsPut, LightsSettingsGet,
        LightsSettingsPut, WifiConfig,
    },
    helpers::encrypt_wifi_payload,
    serialization::deser_response,
};
use anyhow::Result;
use reqwest::{
    Client,
    header::{CONTENT_TYPE, HeaderValue},
};

/// Low-level HTTP client for the Elgato device API.
///
/// `Ecc` wraps a [`reqwest::Client`] and exposes one async method per
/// API endpoint. Each method accepts an `endpoint` string (IP address or
/// hostname) and builds the full URL internally.
///
/// All devices are assumed to be reachable over plain HTTP on port 9123
/// under the `/elgato` namespace. Use [`Ecc::default`] for the standard
/// configuration.
///
/// For a higher-level interface that groups operations on a single light,
/// see [`crate::light::Light`].
pub struct Ecc {
    client: Client,
    protocol: &'static str,
    port: u16,
    namespace: String,
}

impl Default for Ecc {
    /// Creates an `Ecc` client with the standard Elgato device settings:
    /// plain HTTP, port 9123, namespace `/elgato`.
    fn default() -> Self {
        Self {
            client: Client::new(),
            protocol: "http",
            port: 9123,
            namespace: "/elgato".to_string(),
        }
    }
}

impl Ecc {
    // ------------------------------------------------------------------ //
    // Public                                                               //
    // ------------------------------------------------------------------ //

    /// Push a new Wi-Fi configuration to a device.
    ///
    /// Fetches the device's accessory info to derive the AES-128-CBC
    /// encryption key, encrypts the payload, and sends it to the
    /// `/elgato/wifi-info` endpoint as `application/octet-stream`.
    pub async fn wifi_config(&self, endpoint: &str, payload: &WifiConfig) -> Result<()> {
        let accessory_info = self.accessory_info_get(endpoint).await?;
        let encrypted_bytes = encrypt_wifi_payload(&accessory_info, payload)?;
        let url = format!("{}/wifi-info", self.format_url(endpoint));
        self.client
            .put(&url)
            .header(CONTENT_TYPE, HeaderValue::from_static("application/octet-stream"))
            .body(encrypted_bytes)
            .send()
            .await?;
        Ok(())
    }

    /// Trigger the device's identify action (typically causes the light to
    /// flash so the physical unit can be located).
    pub async fn identify(&self, endpoint: &str) -> Result<()> {
        let url = format!("{}/identify", self.format_url(endpoint));
        self.client.post(&url).send().await?;
        Ok(())
    }

    /// Retrieve the current state of all lights on a device.
    pub async fn lights_get(&self, endpoint: &str) -> Result<LightsGet> {
        let url = format!("{}/lights", self.format_url(endpoint));
        let response = self.client.get(&url).send().await?;
        let result = deser_response::<LightsGet>(response).await?;
        Ok(result)
    }

    /// Update the state of all lights on a device and return the new state.
    pub async fn lights_put(&self, endpoint: &str, payload: &LightsPut) -> Result<LightsGet> {
        let url = format!("{}/lights", self.format_url(endpoint));
        let response = self.client.put(&url).json(payload).send().await?;
        let result = deser_response::<LightsGet>(response).await?;
        Ok(result)
    }

    /// Retrieve the persistent light settings for a device (power-on
    /// behaviour, transition durations, remote control favourites, etc.).
    pub async fn lights_settings_get(&self, endpoint: &str) -> Result<LightsSettingsGet> {
        let url = format!("{}/lights/settings", self.format_url(endpoint));
        let response = self.client.get(&url).send().await?;
        let result = deser_response::<LightsSettingsGet>(response).await?;
        Ok(result)
    }

    /// Update the persistent light settings for a device.
    pub async fn lights_settings_put(
        &self,
        endpoint: &str,
        payload: &LightsSettingsPut,
    ) -> Result<()> {
        let url = format!("{}/lights", self.format_url(endpoint));
        self.client.put(&url).json(payload).send().await?;
        Ok(())
    }

    /// Retrieve static accessory information from a device (product name,
    /// firmware version, MAC address, Wi-Fi info, etc.).
    pub async fn accessory_info_get(&self, endpoint: &str) -> Result<AccessoryInfoGet> {
        let url = format!("{}/accessory-info", self.format_url(endpoint));
        let response = self.client.get(&url).send().await?;
        let result = deser_response::<AccessoryInfoGet>(response).await?;
        Ok(result)
    }

    /// Update mutable accessory properties (currently only `display_name`).
    pub async fn accessory_info_put(
        &self,
        endpoint: &str,
        payload: &AccessoryInfoPut,
    ) -> Result<()> {
        let url = format!("{}/accessory-info", self.format_url(endpoint));
        self.client.put(&url).json(payload).send().await?;
        Ok(())
    }

    // ------------------------------------------------------------------ //
    // Private                                                              //
    // ------------------------------------------------------------------ //

    /// Build the base URL for a given endpoint, e.g.
    /// `http://192.168.0.50:9123/elgato`.
    fn format_url(&self, endpoint: &str) -> String {
        format!(
            "{}://{}:{}{}",
            self.protocol, endpoint, self.port, self.namespace
        )
    }
}
