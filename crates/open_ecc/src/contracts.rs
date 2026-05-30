use crate::serialization::{
    temperature_handler, temperature_option_handler, u8_bool_handler, u8_bool_option_handler,
};
use serde::{Deserialize, Serialize};

/// Response body returned by `GET /elgato/lights`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsGet {
    /// Number of light elements reported by the device.
    pub number_of_lights: u8,
    /// Current state of each light.
    pub lights: Vec<LightGet>,
}

/// Request body for `PUT /elgato/lights`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsPut {
    /// Desired state for each light. Must contain the same number of
    /// elements as the device reports in [`LightsGet::number_of_lights`].
    pub lights: Vec<LightPut>,
}

/// State of a single light as returned by the device.
///
/// The `on` field is serialised as `u8` (0/1) on the wire; this crate
/// converts it to `bool` automatically. The `temperature` field is stored
/// in Kelvin here but converted to/from the device's internal unit
/// (range 143-344) by the [`temperature_handler`] serde module.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LightGet {
    /// Whether the light is on.
    #[serde(with = "u8_bool_handler")]
    pub on: bool,
    /// Brightness level in the range 0-100.
    pub brightness: u8,
    /// Colour temperature in Kelvin, range 2900-7000.
    #[serde(with = "temperature_handler")]
    pub temperature: u16,
}

/// Desired state for a single light sent in a PUT request.
///
/// All fields are optional; only supplied fields are serialised and sent,
/// so partial updates are supported. The same wire-format conversions as
/// [`LightGet`] apply.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct LightPut {
    /// Turn the light on (`true`) or off (`false`).
    #[serde(skip_serializing_if = "Option::is_none", with = "u8_bool_option_handler")]
    pub on: Option<bool>,
    /// Brightness level in the range 0-100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    /// Colour temperature in Kelvin, range 2900-7000.
    #[serde(skip_serializing_if = "Option::is_none", with = "temperature_option_handler")]
    pub temperature: Option<u16>,
}

/// Response body returned by `GET /elgato/lights/settings`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsSettingsGet {
    /// Power-on behaviour: `0` = last state, `1` = always on.
    pub power_on_behavior: u8,
    /// Brightness applied when the light powers on, range 0-100.
    pub power_on_brightness: u8,
    /// Colour temperature applied when the light powers on, in Kelvin
    /// (range 2900-7000, increments of 50).
    #[serde(with = "temperature_handler")]
    pub power_on_temperature: u16,
    /// Time in milliseconds for the light to reach full brightness when switching on.
    pub switch_on_duration_ms: u16,
    /// Time in milliseconds for the light to dim to off when switching off.
    pub switch_off_duration_ms: u16,
    /// Time in milliseconds for a colour temperature transition.
    pub color_change_duration_ms: u16,
    /// Remote control configuration.
    pub remote_control: RemoteControl,
}

/// Remote control configuration returned by the device.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteControl {
    /// Saved favourite brightness/temperature presets.
    pub favourites: Vec<Favourite>,
    /// Auto-mode (lux-based) settings.
    pub auto_mode: AutoMode,
}

/// A single saved favourite preset as returned by the device.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct Favourite {
    /// Brightness level, range 0-100.
    pub brightness: u8,
    /// Colour temperature in Kelvin, range 2900-7000 (increments of 50).
    #[serde(with = "temperature_handler")]
    pub temperature: u16,
}

/// Auto-mode (ambient lux) settings as returned by the device.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AutoMode {
    /// Target illuminance in lux for auto-mode.
    pub target_lux_value: u16,
}

/// Request body for `PUT /elgato/lights/settings`.
///
/// All fields are optional; omitted fields are not serialised and
/// therefore left unchanged on the device.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LightsSettingsPut {
    /// Power-on behaviour: `0` = last state, `1` = always on.
    pub power_on_behavior: Option<u8>,
    /// Brightness applied when the light powers on, range 0-100.
    pub power_on_brightness: Option<u8>,
    /// Colour temperature applied when the light powers on, in Kelvin
    /// (range 2900-7000, increments of 50).
    #[serde(skip_serializing_if = "Option::is_none", with = "temperature_option_handler")]
    pub power_on_temperature: Option<u16>,
    /// Time in milliseconds for the light to reach full brightness when switching on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_on_duration_ms: Option<u16>,
    /// Time in milliseconds for the light to dim to off when switching off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch_off_duration_ms: Option<u16>,
    /// Time in milliseconds for a colour temperature transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_change_duration_ms: Option<u16>,
    /// Remote control configuration to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_control: Option<RemoteControlPut>,
}

/// Remote control configuration for a PUT request.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RemoteControlPut {
    /// Favourite presets to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favourites: Option<Vec<FavouritePut>>,
    /// Auto-mode settings to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_mode: Option<AutoModePut>,
}

/// A favourite preset for a PUT request.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct FavouritePut {
    /// Brightness level, range 0-100.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<u8>,
    /// Colour temperature in Kelvin, range 2900-7000 (increments of 50).
    #[serde(skip_serializing_if = "Option::is_none", with = "temperature_option_handler")]
    pub temperature: Option<u16>,
}

/// Auto-mode settings for a PUT request.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct AutoModePut {
    /// Target illuminance in lux for auto-mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_lux_value: Option<u16>,
}

/// Static device information returned by `GET /elgato/accessory-info`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessoryInfoGet {
    /// Elgato product name (e.g. `"Elgato Key Light"`).
    pub product_name: String,
    /// Hardware board type identifier; used in Wi-Fi payload encryption.
    pub hardware_board_type: u16,
    /// Hardware revision string.
    pub hardware_revision: String,
    /// Device MAC address.
    pub mac_address: String,
    /// Firmware build number; used in Wi-Fi payload encryption.
    pub firmware_build_number: u16,
    /// Human-readable firmware version string.
    pub firmware_version: String,
    /// Device serial number.
    pub serial_number: String,
    /// User-assigned display name for the device.
    pub display_name: String,
    /// List of feature strings advertised by the device.
    pub features: Vec<String>,
    /// Current Wi-Fi connection details.
    #[serde(rename = "wifi-info")]
    pub wifi_info: WifiInfo,
    /// Bluetooth status information.
    #[serde(rename = "bt-info")]
    pub bt_info: BtInfo,
}

/// Mutable device properties for `PUT /elgato/accessory-info`.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessoryInfoPut {
    /// User-assigned display name to set on the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

/// Wi-Fi connection details as reported by the device.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WifiInfo {
    /// SSID of the connected network.
    pub ssid: String,
    /// Operating frequency in MHz.
    #[serde(rename = "frequencyMHz")]
    pub frequency_mhz: u16,
    /// Received signal strength in dBm (negative value).
    pub rssi: i8,
}

/// Bluetooth status as reported by the device.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub struct BtInfo {
    /// Bluetooth broadcast mode identifier.
    pub broadcast_mode: u8,
    /// Whether the device is in pairing mode.
    pub pairing: bool,
    /// Whether the device is currently paired.
    pub paired: bool,
}

/// Wi-Fi provisioning payload sent to the device via
/// [`crate::ecc::Ecc::wifi_config`].
///
/// This struct is serialised to pretty-printed JSON, padded to a
/// 16-byte boundary, prefixed with a random 16-byte nonce, and then
/// encrypted with AES-128-CBC before transmission.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct WifiConfig {
    /// Network SSID.
    #[serde(rename = "SSID")]
    pub ssid: String,
    /// Network passphrase. Omitted from the payload when `None`.
    #[serde(rename = "Passphrase", skip_serializing_if = "Option::is_none")]
    pub passphrase: Option<String>,
    /// Wi-Fi security type.
    #[serde(rename = "SecurityType")]
    pub security_type: WifiSecurity,
    /// Wi-Fi channel number, range 1-14. Omitted from the payload when `None`.
    #[serde(rename = "Channel", skip_serializing_if = "Option::is_none")]
    pub channel: Option<u8>,
}

/// Wi-Fi security types supported by the device.
#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
pub enum WifiSecurity {
    /// Open network, no authentication.
    #[default]
    None = 0,
    /// WPA or WPA2 Personal (PSK).
    WpaOrWpa2Personal = 2,
}

/// Error response body returned by the device API on non-2xx status codes.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct JsonErrors {
    pub errors: Vec<JsonError>,
}

/// A single error entry within a [`JsonErrors`] response.
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct JsonError {
    /// Human-readable error description.
    pub message: String,
    /// Device-defined error code.
    pub code: i32,
}
