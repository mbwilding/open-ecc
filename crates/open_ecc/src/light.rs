use crate::{
    contracts::{LightGet, LightPut, LightsGet, LightsPut},
    ecc::Ecc,
};
use anyhow::{Result, anyhow};

pub struct Light<'a> {
    ecc: &'a Ecc,
    endpoint: &'a str,
}

impl<'a> Light<'a> {
    // Construtors

    pub fn new(ecc: &'a Ecc, endpoint: &'a str) -> Self {
        Self { ecc, endpoint }
    }

    // Public

    /// Turn on the light
    pub async fn on(&self) -> Result<()> {
        self.set_light(|_| LightPut {
            on: Some(true),
            ..Default::default()
        })
        .await?;
        Ok(())
    }

    /// Turn off the light
    pub async fn off(&self) -> Result<()> {
        self.set_light(|_| LightPut {
            on: Some(false),
            ..Default::default()
        })
        .await?;
        Ok(())
    }

    /// Toggle the light state
    pub async fn toggle(&self) -> Result<()> {
        self.set_light(|x| LightPut {
            on: Some(!x.on),
            ..Default::default()
        })
        .await?;
        Ok(())
    }

    /// Get light state
    pub async fn state_get(&self) -> Result<bool> {
        self.field_get(|x| x.on).await
    }

    /// Set light state
    pub async fn state_set(&self, state: bool) -> Result<()> {
        self.set_light(|_| LightPut {
            on: Some(state),
            ..Default::default()
        })
        .await?;
        Ok(())
    }

    /// Get temperature in Kelvin [2900..=7000]
    pub async fn temperature_get(&self) -> Result<u16> {
        self.field_get(|x| x.temperature).await
    }

    /// Set temperature in Kelvin [2900..=7000]
    pub async fn temperature_set(&self, value: u16) -> Result<LightsGet> {
        self.set_light(|_| LightPut {
            temperature: Some(value),
            ..Default::default()
        })
        .await
    }

    /// Get brightness [0..=100]
    pub async fn brightness_get(&self) -> Result<u8> {
        self.field_get(|x| x.brightness).await
    }

    /// Set brightness [0..=100]
    pub async fn brightness_set(&self, value: u8) -> Result<LightsGet> {
        self.set_light(|_| LightPut {
            brightness: Some(value),
            ..Default::default()
        })
        .await
    }

    // Private

    async fn field_get<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(LightGet) -> T,
    {
        let lights = self.ecc.lights_get(self.endpoint).await?;
        let value = lights
            .lights
            .into_iter()
            .next()
            .map(f)
            .ok_or(anyhow!("No lights found"))?;
        Ok(value)
    }

    async fn set_light<F>(&self, f: F) -> Result<LightsGet>
    where
        F: Fn(LightGet) -> LightPut,
    {
        let lights = self.ecc.lights_get(self.endpoint).await?;
        let lights_put = lights.lights.into_iter().map(&f).collect::<Vec<_>>();
        self.ecc
            .lights_put(self.endpoint, &LightsPut { lights: lights_put })
            .await
    }
}
