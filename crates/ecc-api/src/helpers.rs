use crate::contracts::{AccessoryInfoGet, JsonErrors, Wifi};
use anyhow::{Result, bail};
use cipher::{BlockEncryptMut, KeyIvInit, block_padding::NoPadding};
use rand::Rng;
use reqwest::Response;
use serde::de::DeserializeOwned;

type Aes128Cbc = cbc::Encryptor<aes::Aes128>;

fn add_padding(bytes: &mut Vec<u8>) {
    while bytes.len() % 16 != 0 {
        bytes.push(0);
    }
}

fn random_prefix() -> Vec<u8> {
    let mut rng = rand::rng();
    (0..16).map(|_| rng.random_range(0..255)).collect()
}

fn get_encryption_key(data: &AccessoryInfoGet) -> String {
    format!(
        "4CB4{:04X}B0EADDEEEB2A038A31{:04X}56",
        data.hardware_board_type.swap_bytes(),
        data.firmware_build_number.swap_bytes()
    )
}

pub fn encrypt_wifi_payload(accessory_info: &AccessoryInfoGet, payload: &Wifi) -> Result<Vec<u8>> {
    let mut bytes_array = serde_json::to_vec_pretty(payload).expect("Failed to serialize JSON");
    add_padding(&mut bytes_array);
    let random_array = random_prefix();
    let mut data: Vec<u8> = [random_array, bytes_array].concat();

    let iv_bytes = hex::decode("049F6F1149C6F84B1B14913C71E9CDBE").expect("Invalid IV hex");

    let key = get_encryption_key(accessory_info);

    let key_bytes = hex::decode(key).expect("Invalid key hex string");

    let aes_cbc = Aes128Cbc::new_from_slices(&key_bytes, &iv_bytes).expect("Invalid key/iv length");

    let len = data.len();
    let encrypted_bytes = aes_cbc
        .encrypt_padded_mut::<NoPadding>(&mut data, len)
        .expect("Encryption failure");

    Ok(encrypted_bytes.to_vec())
}

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

pub fn api_to_kelvin(api: u16) -> u16 {
    const API_MIN: u16 = 143;
    const API_MAX: u16 = 344;
    const K_MIN: u16 = 2900;
    const K_MAX: u16 = 7000;

    let kelvin = ((api.saturating_sub(API_MIN)) as f64) * (K_MAX - K_MIN) as f64
        / ((API_MAX - API_MIN) as f64)
        + (K_MIN as f64);

    let stepped = (kelvin / 50.0).round() * 50.0;

    stepped.clamp(K_MIN as f64, K_MAX as f64) as u16
}

pub fn kelvin_to_api(kelvin: u16) -> u16 {
    const API_MIN: u16 = 143;
    const API_MAX: u16 = 344;
    const K_MIN: u16 = 2900;
    const K_MAX: u16 = 7000;

    let k = kelvin.clamp(K_MIN, K_MAX);

    let api = ((k - K_MIN) as f64) * (API_MAX - API_MIN) as f64 / ((K_MAX - K_MIN) as f64)
        + (API_MIN as f64);

    api.round() as u16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_encryption_key() {
        let accessory_info = AccessoryInfoGet {
            firmware_build_number: 198,
            hardware_board_type: 205,
            ..Default::default()
        };

        let expected = "4CB4CD00B0EADDEEEB2A038A31C60056".to_string();
        let result = get_encryption_key(&accessory_info);
        assert_eq!(
            result, expected,
            "Encryption key does not match expected value"
        );
    }

    #[test]
    fn test_api_to_kelvin() {
        assert_eq!(api_to_kelvin(143), 2900);
        assert_eq!(api_to_kelvin(344), 7000);
        let mid_kelvin = api_to_kelvin((143 + 344) / 2);
        assert!(mid_kelvin % 50 == 0);
    }

    #[test]
    fn test_kelvin_to_api() {
        assert_eq!(kelvin_to_api(2900), 143);
        assert_eq!(kelvin_to_api(7000), 344);
        for k in (2900..=7000).step_by(50) {
            let api = kelvin_to_api(k);
            let k2 = api_to_kelvin(api);
            assert_eq!(k, k2, "Failed at Kelvin {}", k);
        }
    }
}
