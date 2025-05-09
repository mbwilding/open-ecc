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

pub struct Ecc {
    client: Client,
    protocol: &'static str,
    port: u16,
    namespace: String,
}

impl Default for Ecc {
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
    fn format_url(&self, endpoint: &str) -> String {
        format!(
            "{}://{}:{}{}",
            self.protocol, endpoint, self.port, self.namespace
        )
    }

    pub async fn wifi_config(&self, endpoint: &str, payload: &WifiConfig) -> Result<()> {
        let accessory_info = self.accessory_info_get(endpoint).await?;
        let encrypted_bytes = encrypt_wifi_payload(&accessory_info, payload)?;
        let url = format!("{}/wifi-info", self.format_url(endpoint));
        self.client
            .put(&url)
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .body(encrypted_bytes)
            .send()
            .await?;
        Ok(())
    }

    pub async fn identify(&self, endpoint: &str) -> Result<()> {
        let url = format!("{}/identify", self.format_url(endpoint));
        self.client.post(&url).send().await?;
        Ok(())
    }

    pub async fn lights_get(&self, endpoint: &str) -> Result<LightsGet> {
        let url = format!("{}/lights", self.format_url(endpoint));
        let response = self.client.get(&url).send().await?;
        let result = deser_response::<LightsGet>(response).await?;
        Ok(result)
    }

    pub async fn lights_put(&self, endpoint: &str, payload: &LightsPut) -> Result<LightsGet> {
        let url = format!("{}/lights", self.format_url(endpoint));
        let response = self.client.put(&url).json(payload).send().await?;
        let result = deser_response::<LightsGet>(response).await?;
        Ok(result)
    }

    pub async fn lights_settings_get(&self, endpoint: &str) -> Result<LightsSettingsGet> {
        let url = format!("{}/lights/settings", self.format_url(endpoint));
        let response = self.client.get(&url).send().await?;
        let result = deser_response::<LightsSettingsGet>(response).await?;
        Ok(result)
    }

    pub async fn lights_settings_put(
        &self,
        endpoint: &str,
        payload: &LightsSettingsPut,
    ) -> Result<()> {
        let url = format!("{}/lights", self.format_url(endpoint));
        self.client.put(&url).json(payload).send().await?;
        Ok(())
    }

    pub async fn accessory_info_get(&self, endpoint: &str) -> Result<AccessoryInfoGet> {
        let url = format!("{}/accessory-info", self.format_url(endpoint));
        let response = self.client.get(&url).send().await?;
        let result = deser_response::<AccessoryInfoGet>(response).await?;
        Ok(result)
    }

    pub async fn accessory_info_put(
        &self,
        endpoint: &str,
        payload: &AccessoryInfoPut,
    ) -> Result<()> {
        let url = format!("{}/accessory-info", self.format_url(endpoint));
        self.client.put(&url).json(payload).send().await?;
        Ok(())
    }
}
