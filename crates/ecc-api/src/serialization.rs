use crate::contracts::JsonErrors;
use anyhow::{Result, bail};
use reqwest::Response;
use serde::de::DeserializeOwned;

pub async fn deser_response<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    if response.status().is_success() {
        Ok(response.json::<T>().await?)
    } else {
        let errors = response.json::<JsonErrors>().await?;
        bail!("{:#?}", errors)
    }
}

pub mod temperature_handler {
    use crate::helpers::{api_to_kelvin, kelvin_to_api};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(temp: &u16, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(kelvin_to_api(*temp))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(api_to_kelvin(u16::deserialize(deserializer)?))
    }
}

pub mod option_temperature_handler {
    use crate::helpers::{api_to_kelvin, kelvin_to_api};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(temp: &Option<u16>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match temp {
            Some(val) => {
                let api_val = kelvin_to_api(*val);
                serializer.serialize_some(&api_val)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt_api = Option::<u16>::deserialize(deserializer)?;
        Ok(opt_api.map(api_to_kelvin))
    }
}
