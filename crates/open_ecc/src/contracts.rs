use crate::serialization::{
    temperature_handler, temperature_option_handler, u8_bool_handler, u8_bool_option_handler,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsGet {
    pub number_of_lights: u8,
    pub lights: Vec<LightGet>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsPut {
    pub lights: Vec<LightPut>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LightGet {
    /// State
    #[serde(with = "u8_bool_handler")]
    pub on: bool,
    /// Brightness
    pub brightness: u8,
    #[serde(with = "temperature_handler")]
    /// Range: 2900 - 7000 Kelvin
    pub temperature: u16,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LightPut {
    /// State
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "u8_bool_option_handler"
    )]
    pub on: Option<bool>,
    /// Range: 0 - 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    /// Range: 2900 - 7000 Kelvin
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "temperature_option_handler"
    )]
    pub temperature: Option<u16>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsSettingsGet {
    /// State: 0 | 1
    pub power_on_behavior: u8,
    /// Range: 0 - 100
    pub power_on_brightness: u8,
    /// Range: 2900 - 7000 (increments of 50)
    #[serde(with = "temperature_handler")]
    pub power_on_temperature: u16,
    pub switch_on_duration_ms: u16,
    pub switch_off_duration_ms: u16,
    pub color_change_duration_ms: u16,
    pub remote_control: RemoteControl,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteControl {
    pub favourites: Vec<Favourite>,
    pub auto_mode: AutoMode,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Favourite {
    /// Range: 0 - 100
    pub brightness: u8,
    /// Range: 2900 - 7000 (increments of 50)
    #[serde(with = "temperature_handler")]
    pub temperature: u16,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AutoMode {
    pub target_lux_value: u16,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsSettingsPut {
    /// State: 0 | 1
    pub power_on_behavior: Option<u8>,
    /// Range: 0 - 100
    pub power_on_brightness: Option<u8>,
    /// Range: 2900 - 7000 (increments of 50)
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "temperature_option_handler"
    )]
    pub power_on_temperature: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_on_duration_ms: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_off_duration_ms: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_change_duration_ms: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_control: Option<RemoteControlPut>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteControlPut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites: Option<Vec<FavouritePut>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<AutoModePut>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FavouritePut {
    /// Range: 0 - 100
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    /// Range: 2900 - 7000 (increments of 50)
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "temperature_option_handler"
    )]
    pub temperature: Option<u16>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AutoModePut {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lux_value: Option<u16>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessoryInfoGet {
    pub product_name: String,
    pub hardware_board_type: u16,
    pub hardware_revision: String,
    pub mac_address: String,
    pub firmware_build_number: u16,
    pub firmware_version: String,
    pub serial_number: String,
    /// The user specified name of the device
    pub display_name: String,
    pub features: Vec<String>,
    #[serde(rename = "wifi-info")]
    pub wifi_info: WifiInfo,
    #[serde(rename = "bt-info")]
    pub bt_info: BtInfo,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessoryInfoPut {
    /// The user specified name of the device
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WifiInfo {
    pub ssid: String,
    #[serde(rename = "frequencyMHz")]
    pub frequency_mhz: u16,
    pub rssi: i8,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct BtInfo {
    pub broadcast_mode: u8,
    pub pairing: bool,
    pub paired: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WifiConfig {
    #[serde(rename = "SSID")]
    pub ssid: String,
    #[serde(rename = "Passphrase", skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
    #[serde(rename = "SecurityType")]
    pub security_type: WifiSecurity,
    /// Range: 1 - 14
    #[serde(rename = "Channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<u8>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum WifiSecurity {
    #[default]
    None = 0,
    WpaOrWpa2Personal = 2,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct JsonErrors {
    pub errors: Vec<JsonError>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct JsonError {
    pub message: String,
    pub code: i32,
}
