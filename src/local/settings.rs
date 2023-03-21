use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::local::weather_file::WeatherFile;

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct SettingsJson {
    #[pyo3(get, set)]
    OPEN_WEATHER_MAP_API_KEY: Option<String>,
    #[pyo3(get, set)]
    BING_MAPS_API_KEY: Option<String>,
    #[pyo3(get, set)]
    NCDC_API_KEY: Option<String>,
    #[pyo3(get, set)]
    METRIC_DEFAULT: Option<bool>,
    #[pyo3(get, set)]
    DEFAULT_BACKEND: Option<String>,
    #[pyo3(get, set)]
    CONSTANT_LOCATION: Option<bool>,
    #[pyo3(get, set)]
    DEFAULT_LAYOUT: Option<String>,
    #[pyo3(get, set)]
    AUTO_UPDATE_INTERNET_RESOURCES: Option<bool>,
    #[pyo3(get, set)]
    DEBUG: Option<bool>,
    #[pyo3(get, set)]
    DEVELOPMENT: Option<bool>,
    #[pyo3(get, set)]
    SHOW_ALERTS: Option<bool>,
    #[pyo3(get, set)]
    LAYOUT_FILE: Option<String>,
    #[pyo3(get, set)]
    ENABLE_DAEMON: Option<bool>
}

#[pyclass]
pub struct Settings {
    #[pyo3(get)]
    internal: SettingsJson,
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
        if parsed.OPEN_WEATHER_MAP_API_KEY.is_none() {
            parsed.OPEN_WEATHER_MAP_API_KEY = Some("".to_string());
        }
        if parsed.BING_MAPS_API_KEY.is_none() {
            parsed.BING_MAPS_API_KEY = Some("".to_string());
        }
        if parsed.NCDC_API_KEY.is_none() {
            parsed.NCDC_API_KEY = Some("".to_string());
        }
        if parsed.METRIC_DEFAULT.is_none() {
            parsed.METRIC_DEFAULT = Some(false);
        }
        let valid_backends = vec![
            "OPENWEATHERMAP".to_string(),
            "METEO".to_string(),
            "NWS".to_string(),
            "THEWEATHERCHANNEL".to_string(),
        ];
        if parsed.DEFAULT_BACKEND.is_none()
            || !valid_backends.contains(&parsed.DEFAULT_BACKEND.clone().unwrap())
        {
            parsed.DEFAULT_BACKEND = Some("METEO".to_string());
        }
        if parsed.CONSTANT_LOCATION.is_none() {
            parsed.CONSTANT_LOCATION = Some(false);
        }
        if parsed.AUTO_UPDATE_INTERNET_RESOURCES.is_none() {
            parsed.AUTO_UPDATE_INTERNET_RESOURCES = Some(true);
        }
        if parsed.DEBUG.is_none() {
            parsed.DEBUG = Some(true);
        }
        if parsed.DEVELOPMENT.is_none() {
            parsed.DEVELOPMENT = Some(true);
        }
        if parsed.SHOW_ALERTS.is_none() {
            parsed.SHOW_ALERTS = Some(true);
        }
        if parsed.SHOW_ALERTS.is_none() {
            parsed.SHOW_ALERTS = Some(true);
        }
        if parsed.LAYOUT_FILE.is_some()
            && parsed.LAYOUT_FILE.clone().unwrap().to_lowercase() == "none"
        {
            parsed.LAYOUT_FILE = None;
        }
        if parsed.ENABLE_DAEMON.is_none() {
            parsed.ENABLE_DAEMON = Some(true);
        }
        let internal = parsed;
        Settings { internal, file }
    }

    fn write(&mut self) {
        self.file.data = serde_json::to_string(&self.internal).unwrap();
        self.file.write();
    }
}
