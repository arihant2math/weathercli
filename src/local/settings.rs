use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::local::weather_file::WeatherFile;

#[pyclass]
#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SettingsJson {
    #[pyo3(get, set)]
    pub open_weather_map_api_key: Option<String>,
    #[pyo3(get, set)]
    pub bing_maps_api_key: Option<String>,
    #[pyo3(get, set)]
    pub ncdc_api_key: Option<String>,
    #[pyo3(get, set)]
    pub metric_default: Option<bool>,
    #[pyo3(get, set)]
    pub default_backend: Option<String>,
    #[pyo3(get, set)]
    pub constant_location: Option<bool>,
    #[pyo3(get, set)]
    pub default_layout: Option<String>,
    #[pyo3(get, set)]
    pub auto_update_internet_resources: Option<bool>,
    #[pyo3(get, set)]
    pub debug: Option<bool>,
    #[pyo3(get, set)]
    pub development: Option<bool>,
    #[pyo3(get, set)]
    pub show_alerts: Option<bool>,
    #[pyo3(get, set)]
    pub layout_file: Option<String>,
    #[pyo3(get, set)]
    pub enable_daemon: Option<bool>,
    #[pyo3(get, set)]
    pub daemon_update_interval: Option<i64>
}

#[pyclass]
pub struct Settings {
    #[pyo3(get)]
    pub internal: SettingsJson,
    value_base: Value,
    file: WeatherFile,
}

impl Default for Settings {
    fn default() -> Self {
        Self::new()
    }
}

#[pymethods]
impl Settings {
    #[new]
    pub fn new() -> Self {
        let file = WeatherFile::new("settings.json".to_string());
        let mut parsed: SettingsJson = serde_json::from_str(&file.data).expect("JSON read failed");
        let values: Value = serde_json::from_str(&file.data).expect("JSON read failed");
        if parsed.open_weather_map_api_key.is_none() {
            parsed.open_weather_map_api_key = Some("".to_string());
        }
        if parsed.bing_maps_api_key.is_none() {
            parsed.bing_maps_api_key = Some("".to_string());
        }
        if parsed.ncdc_api_key.is_none() {
            parsed.ncdc_api_key = Some("".to_string());
        }
        if parsed.metric_default.is_none() {
            parsed.metric_default = Some(false);
        }
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
        if parsed.constant_location.is_none() {
            parsed.constant_location = Some(false);
        }
        if parsed.auto_update_internet_resources.is_none() {
            parsed.auto_update_internet_resources = Some(true);
        }
        if parsed.debug.is_none() {
            parsed.debug = Some(true);
        }
        if parsed.development.is_none() {
            parsed.development = Some(true);
        }
        if parsed.show_alerts.is_none() {
            parsed.show_alerts = Some(true);
        }
        if parsed.show_alerts.is_none() {
            parsed.show_alerts = Some(true);
        }
        if parsed.layout_file.is_some()
            && parsed.layout_file.clone().unwrap().to_lowercase() == "none"
        {
            parsed.layout_file = None;
        }
        if parsed.enable_daemon.is_none() {
            parsed.enable_daemon = Some(true);
        }
        if parsed.daemon_update_interval.is_none() {
            parsed.daemon_update_interval = Some(300);
        }
        Settings { internal: parsed, value_base: values, file }
    }

    pub fn write(&mut self) {
        self.file.data = serde_json::to_string(&self.internal).unwrap();
        self.file.write();
    }

    pub fn set(&mut self, key: String, value: &str) {
        let j = json!(value); // TODO: TECHDEBT
        self.value_base[key] = j;
        self.write();
    }

    pub fn reload(&self) {

    }
}
