use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::struct_excessive_bools)]
pub struct SettingsJson {
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
    #[serde(default)]
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
}

#[derive(Clone)]
pub struct Settings {
    pub internal: SettingsJson,
    file: WeatherFile,
}

impl Settings {
    pub fn new() -> crate::Result<Self> {
        let file = WeatherFile::settings()?;
        unsafe {
            let parsed: SettingsJson = simd_json::serde::from_str(&mut file.get_text()?)?;
            Ok(Self {
                internal: parsed,
                file,
            })
        }
    }

    pub fn write(&mut self) -> crate::Result<()> {
        self.file.data = Vec::from(simd_json::serde::to_string(&self.internal)?);
        self.file.write()?;
        Ok(())
    }

    pub fn reload(&mut self) -> crate::Result<()> {
        self.file = WeatherFile::settings()?;
        unsafe {
            let parsed: SettingsJson = simd_json::serde::from_str(&mut self.file.get_text()?)?;
            self.internal = parsed;
        }
        Ok(())
    }
}
