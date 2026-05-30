use crate::{
    contracts::{LightGet, LightPut, LightsGet, LightsPut},
    ecc::Ecc,
};
use anyhow::{Result, anyhow};

/// High-level wrapper for controlling a single Elgato Key Light.
///
/// `Light` borrows a shared [`Ecc`] client and binds it to one endpoint
/// address. All operations are async and correspond to HTTP requests
/// against the device.
///
/// For write operations that need the current state (e.g. [`toggle`]),
/// `Light` first fetches the current values with a GET and then applies
/// the transformation with a PUT. Operations that do not need the current
/// state (e.g. [`on`], [`brightness_set`]) skip the GET.
///
/// [`toggle`]: Light::toggle
/// [`on`]: Light::on
/// [`brightness_set`]: Light::brightness_set
pub struct Light<'a> {
    ecc: &'a Ecc,
    endpoint: &'a str,
}

impl<'a> Light<'a> {
    // ------------------------------------------------------------------ //
    // Constructors                                                         //
    // ------------------------------------------------------------------ //

    /// Create a `Light` bound to `endpoint` using the provided `ecc` client.
    ///
    /// `endpoint` should be an IP address or hostname reachable on the
    /// local network (e.g. `"192.168.0.50"`).
    pub fn new(ecc: &'a Ecc, endpoint: &'a str) -> Self {
        Self { ecc, endpoint }
    }

    // ------------------------------------------------------------------ //
    // Public                                                               //
    // ------------------------------------------------------------------ //

    /// Turn the light on.
    pub async fn on(&self) -> Result<()> {
        self.set_light(|_| LightPut { on: Some(true), ..Default::default() }).await?;
        Ok(())
    }

    /// Turn the light off.
    pub async fn off(&self) -> Result<()> {
        self.set_light(|_| LightPut { on: Some(false), ..Default::default() }).await?;
        Ok(())
    }

    /// Toggle the light between on and off.
    ///
    /// Fetches the current state first to determine the correct target state.
    pub async fn toggle(&self) -> Result<()> {
        self.set_light(|x| LightPut { on: Some(!x.on), ..Default::default() }).await?;
        Ok(())
    }

    /// Return `true` if the light is currently on.
    pub async fn state_get(&self) -> Result<bool> {
        self.field_get(|x| x.on).await
    }

    /// Set the light on/off state explicitly.
    pub async fn state_set(&self, state: bool) -> Result<()> {
        self.set_light(|_| LightPut { on: Some(state), ..Default::default() }).await?;
        Ok(())
    }

    /// Return the current colour temperature in Kelvin (range 2900-7000).
    pub async fn temperature_get(&self) -> Result<u16> {
        self.field_get(|x| x.temperature).await
    }

    /// Set the colour temperature in Kelvin (range 2900-7000).
    ///
    /// Returns the updated light state as confirmed by the device.
    pub async fn temperature_set(&self, value: u16) -> Result<LightsGet> {
        self.set_light(|_| LightPut { temperature: Some(value), ..Default::default() }).await
    }

    /// Return the current brightness level (range 0-100).
    pub async fn brightness_get(&self) -> Result<u8> {
        self.field_get(|x| x.brightness).await
    }

    /// Set the brightness level (range 0-100).
    ///
    /// Returns the updated light state as confirmed by the device.
    pub async fn brightness_set(&self, value: u8) -> Result<LightsGet> {
        self.set_light(|_| LightPut { brightness: Some(value), ..Default::default() }).await
    }

    // ------------------------------------------------------------------ //
    // Private                                                              //
    // ------------------------------------------------------------------ //

    /// Fetch the state of all lights on the device and extract a single
    /// field from the first light using the provided closure.
    ///
    /// Returns an error if the device reports no lights.
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

    /// Fetch the current state of all lights, apply `f` to each to produce
    /// a [`LightPut`], and send the updated state back to the device.
    ///
    /// This read-modify-write pattern ensures that only the intended fields
    /// are changed while all others retain their current values.
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
