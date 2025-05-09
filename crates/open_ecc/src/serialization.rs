use crate::contracts::JsonErrors;
use anyhow::{Result, bail};
use reqwest::Response;
use serde::de::DeserializeOwned;

pub(crate) async fn deser_response<T>(response: Response) -> Result<T>
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

pub(crate) mod u8_bool_handler {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(if *value { 1 } else { 0 })
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let num = u8::deserialize(deserializer)?;
        Ok(num != 0)
    }
}

pub(crate) mod u8_bool_option_handler {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<bool>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(x) => serializer.serialize_some(&if *x { 1u8 } else { 0u8 }),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<u8>::deserialize(deserializer)?.map(|num| num != 0))
    }
}

pub(crate) mod temperature_handler {
    use crate::helpers::{api_to_kelvin, kelvin_to_api};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &u16, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(kelvin_to_api(*value))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(api_to_kelvin(u16::deserialize(deserializer)?))
    }
}

pub(crate) mod temperature_option_handler {
    use crate::helpers::{api_to_kelvin, kelvin_to_api};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(value: &Option<u16>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(x) => serializer.serialize_some(&kelvin_to_api(*x)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u16>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Option::<u16>::deserialize(deserializer)?.map(api_to_kelvin))
    }
}
