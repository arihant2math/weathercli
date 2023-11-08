use std::collections::HashMap;

use crate::openweathermap_shared::json::OpenWeatherMapConditionJson;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MomentJson {
    pub dt: i64,
    pub sunrise: Option<i64>,
    pub sunset: Option<i64>,
    pub temp: f64,
    pub feels_like: f64,
    pub pressure: i64,
    pub humidity: i64,
    pub dew_point: f64,
    pub uvi: f64,
    pub clouds: u8,
    pub visibility: u64,
    pub wind_speed: f64,
    pub wind_deg: u16,
    pub weather: Vec<OpenWeatherMapConditionJson>,
    pub rain: Option<HashMap<String, f64>>,
    pub snow: Option<HashMap<String, f64>>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MinutelyJson {
    pub dt: i64,
    pub precipitation: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DailyJson {
    pub dt: i64,
    pub sunrise: i64,
    pub sunset: i64,
    pub moonrise: i64,
    pub moonset: i64,
    pub moon_phase: f64,
    pub temp: HashMap<String, f64>, // TODO: Optimize this into actual structs
    pub feels_like: HashMap<String, f64>,
    pub pressure: i64,
    pub humidity: i64,
    pub dew_point: f64,
    pub wind_speed: f64,
    pub wind_deg: i64,
    pub weather: Vec<OpenWeatherMapConditionJson>,
    pub clouds: u8,
    pub pop: f64,
    pub rain: Option<f64>,
    pub snow: Option<f64>,
    pub uvi: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AlertsJson {
    pub sender_name: String,
    pub event: String,
    pub start: i64,
    pub end: i64,
    pub description: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MainJson {
    pub lat: f64,
    pub lon: f64,
    pub timezone: String,
    pub timezone_offset: i64,
    pub current: MomentJson,
    pub minutely: Option<Vec<MinutelyJson>>,
    pub hourly: Vec<MomentJson>,
    pub daily: Vec<DailyJson>,
    pub alerts: Option<Vec<AlertsJson>>,
}
