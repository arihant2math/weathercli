use std::collections::HashMap;

use crate::openweathermap_shared::json::OpenWeatherMapConditionJson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapCoordinatesJson {
    pub lon: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapMainJson {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub humidity: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapWindJson {
    pub speed: f64,
    pub deg: u16,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapCloudsJson {
    pub all: u8,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapSysJson {
    #[serde(rename = "type")]
    pub type_name: i64,
    pub id: i64,
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapJson {
    pub coord: OpenWeatherMapCoordinatesJson,
    pub weather: Vec<OpenWeatherMapConditionJson>,
    pub base: String,
    pub main: OpenWeatherMapMainJson,
    pub visibility: i32,
    pub wind: OpenWeatherMapWindJson,
    pub clouds: OpenWeatherMapCloudsJson,
    pub sys: OpenWeatherMapSysJson,
    pub timezone: i64,
    pub id: i64,
    pub name: String,
    pub cod: i32,
    pub dt: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapAirQualityItemJson {
    pub main: HashMap<String, i8>,
    pub components: HashMap<String, f64>,
    pub dt: i64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapAirQualityJson {
    pub coord: OpenWeatherMapCoordinatesJson,
    pub list: Vec<OpenWeatherMapAirQualityItemJson>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastMainJson {
    pub temp: f64,
    pub feels_like: f64,
    pub temp_min: f64,
    pub temp_max: f64,
    pub pressure: i32,
    pub sea_level: i32,
    pub grnd_level: i32,
    pub humidity: i32,
    pub temp_kf: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastWindJson {
    pub speed: f64,
    pub deg: u16,
    pub gust: f64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastSysJson {
    pod: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastItemJson {
    pub dt: i64,
    pub main: OpenWeatherMapForecastMainJson,
    pub weather: Vec<OpenWeatherMapConditionJson>,
    pub clouds: OpenWeatherMapCloudsJson,
    pub wind: OpenWeatherMapForecastWindJson,
    pub visibility: i32,
    pub pop: f64,
    pub sys: OpenWeatherMapForecastSysJson,
    pub dt_txt: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastJson {
    cod: String,
    message: i64,
    cnt: i64,
    pub list: Vec<OpenWeatherMapForecastItemJson>,
}
