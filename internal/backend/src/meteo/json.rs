use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoHourlyJson {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f32>,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoDailyJson {
    pub time: Vec<String>,
    pub temperature_2m_max: Vec<f32>,
    pub temperature_2m_min: Vec<f32>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoCurrentWeatherJson {
    pub temperature: f32,
    pub windspeed: f64,
    pub winddirection: f64,
    pub weathercode: i64,
    pub time: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoForecastJson {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f32,
    pub current_weather: MeteoCurrentWeatherJson,
    pub hourly_units: HashMap<String, String>,
    pub hourly: MeteoHourlyJson,
    pub daily_units: HashMap<String, String>,
    pub daily: MeteoDailyJson,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoAQIHourlyJson {
    pub time: Vec<String>,
    pub european_aqi: Vec<Option<u8>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoAirQualityJson {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub hourly_units: HashMap<String, String>,
    pub hourly: MeteoAQIHourlyJson,
}
