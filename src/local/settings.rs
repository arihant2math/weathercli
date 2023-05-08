use serde::{Deserialize, Serialize};
use windows::Win32::System::Power::SYSTEM_POWER_STATUS;

use crate::local::weather_file::WeatherFile;

const fn _true() -> bool {
    true
}

fn _default_layout() -> String {
    String::from("default.res")
}

const fn _default_daemon_update_interval() -> i64 {
    600
}

fn _meteo() -> String {
    String::from("meteo")
}

fn _update_server() -> String {
    String::from("https://arihant2math.github.io/weathercli/")
}

fn _file() -> WeatherFile {
    WeatherFile::settings().unwrap()
}

#[cfg(not(windows))]
fn _constant_location() -> bool {
    false
}

#[cfg(windows)]
unsafe fn _constant_location_base() -> crate::Result<bool> {
    let mut power_status = SYSTEM_POWER_STATUS::default();
    let success = windows::Win32::System::Power::GetSystemPowerStatus(&mut power_status).0;
    match success {
        0 => {
            let error = windows::Win32::Foundation::GetLastError()
                .to_hresult()
                .message()
                .to_string();
            Err(error)?
        }
        _ => Ok(power_status.ACLineStatus == 255),
    }
}

#[cfg(windows)]
fn _constant_location() -> bool {
    unsafe { _constant_location_base().unwrap_or(false) }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::struct_excessive_bools)]
pub struct Settings {
    #[serde(default)]
    pub open_weather_map_api_key: String,
    #[serde(default)]
    pub bing_maps_api_key: String,
    #[serde(default)]
    pub ncdc_api_key: String,
    #[serde(default)]
    pub metric_default: bool,
    #[serde(default = "_meteo")]
    pub default_backend: String,
    #[serde(default = "_constant_location")]
    pub constant_location: bool,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub development: bool,
    #[serde(default = "_true")]
    pub show_alerts: bool,
    #[serde(default = "_default_layout")]
    pub layout_file: String,
    #[serde(default)]
    pub enable_daemon: bool,
    #[serde(default = "_default_daemon_update_interval")]
    pub daemon_update_interval: i64,
    #[serde(default = "_true")]
    pub auto_update_internet_resources: bool,
    pub installed_components: Option<Vec<String>>,
    #[serde(default = "_update_server")]
    pub update_server: String,
    #[serde(default)]
    pub enable_custom_backends: bool,
    #[serde(skip_serializing, skip_deserializing)]
    #[serde(default = "_file")]
    file: WeatherFile,
}

impl Settings {
    pub fn new() -> crate::Result<Self> {
        unsafe { Ok(simd_json::from_str(&mut _file().get_text()?)?) }
    }

    pub fn write(&mut self) -> crate::Result<()> {
        self.file.data = Vec::from(simd_json::to_string(&self)?);
        self.file.write()?;
        Ok(())
    }
}
