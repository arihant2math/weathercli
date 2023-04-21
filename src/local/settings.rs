use serde::{Deserialize, Serialize};

use crate::local::weather_file::WeatherFile;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SettingsJson {
    pub open_weather_map_api_key: Option<String>,
    pub bing_maps_api_key: Option<String>,
    pub ncdc_api_key: Option<String>,
    pub metric_default: Option<bool>,
    pub default_backend: Option<String>,
    pub constant_location: Option<bool>,
    pub default_layout: Option<String>,
    pub auto_update_internet_resources: Option<bool>,
    pub debug: Option<bool>,
    pub development: Option<bool>,
    pub show_alerts: Option<bool>,
    pub layout_file: Option<String>,
    pub enable_daemon: Option<bool>,
    pub daemon_update_interval: Option<i64>,
    pub installed_components: Option<Vec<String>>,
}

#[derive(Clone)]
pub struct Settings {
    pub internal: SettingsJson,
    file: WeatherFile,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

impl Settings {
    pub fn new() -> Self {
        let file = WeatherFile::settings();
        let mut parsed: SettingsJson = serde_json::from_str(&file.data).expect("JSON read failed");
        parsed.open_weather_map_api_key =
            Some(parsed.open_weather_map_api_key.unwrap_or("".to_string()));
        parsed.bing_maps_api_key = Some(parsed.bing_maps_api_key.unwrap_or("".to_string()));
        parsed.ncdc_api_key = Some(parsed.ncdc_api_key.unwrap_or("".to_string()));
        parsed.metric_default = Some(parsed.metric_default.unwrap_or(false));
        let valid_backends = vec![
            "OPENWEATHERMAP".to_string(),
            "METEO".to_string(),
            "NWS".to_string(),
            "THEWEATHERCHANNEL".to_string(),
        ];
        if parsed.default_backend.is_none()
            || !valid_backends.contains(&parsed.default_backend.clone().unwrap())
        {
            parsed.default_backend = Some("METEO".to_string());
        }
        parsed.constant_location = Some(parsed.constant_location.unwrap_or(false));
        parsed.auto_update_internet_resources =
            Some(parsed.auto_update_internet_resources.unwrap_or(true));
        parsed.debug = Some(parsed.debug.unwrap_or(false));
        parsed.development = Some(parsed.development.unwrap_or(false));
        parsed.show_alerts = Some(parsed.show_alerts.unwrap_or(true));
        parsed.enable_daemon = Some(parsed.enable_daemon.unwrap_or(false));
        parsed.daemon_update_interval = Some(parsed.daemon_update_interval.unwrap_or(300));
        Settings {
            internal: parsed,
            file,
        }
    }

    pub fn write(&mut self) {
        self.file.data = serde_json::to_string(&self.internal).unwrap();
        self.file.write();
    }

    pub fn reload(&self) {}
}
