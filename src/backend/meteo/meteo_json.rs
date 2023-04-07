use std::collections::HashMap;

use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoHourlyJson {
    pub time: Vec<String>,
    pub temperature_2m: Vec<i32>,
    pub rain: Vec<f32>,
    pub showers: Vec<f32>,
    pub snowfall: Vec<f32>,
    pub cloudcover: Vec<u8>,
    pub dewpoint_2m: Vec<f32>,
    pub apparent_temperature: Vec<f32>,
    pub pressure_msl: Vec<f64>,
    pub visibility: Vec<f64>,
    pub windspeed_10m: Vec<f32>,
    pub winddirection_10m: Vec<f32>,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoDailyJson {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoCurrentWeatherJson {
    pub temperature: f32,
    pub windspeed: u64,
    pub winddirection: u16,
    pub weathercode: i64,
    pub time: String,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoForecastJson {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: i32,
    pub current_weather: MeteoCurrentWeatherJson,
    pub hourly_units: HashMap<String, String>,
    pub hourly: MeteoHourlyJson,
    pub daily_units: HashMap<String, String>,
    pub daily: MeteoDailyJson,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoAQIHourlyJson {
    pub time: Vec<String>,
    pub european_aqi: Vec<u8>,
}

#[pyclass]
#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoAirQualityJson {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub hourly_units: HashMap<String, String>,
    pub hourly: MeteoAQIHourlyJson
}
