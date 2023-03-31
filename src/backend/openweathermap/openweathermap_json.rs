use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapCoordinatesJson {
    #[pyo3(get)]
    pub lon: f64,
    #[pyo3(get)]
    pub lat: f64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapConditionJson {
    #[pyo3(get)]
    pub id: i16,
    #[pyo3(get)]
    pub main: String,
    #[pyo3(get)]
    pub description: String,
    #[pyo3(get)]
    pub icon: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapMainJson {
    #[pyo3(get)]
    pub temp: f64,
    #[pyo3(get)]
    pub feels_like: f64,
    #[pyo3(get)]
    pub temp_min: f64,
    #[pyo3(get)]
    pub temp_max: f64,
    #[pyo3(get)]
    pub pressure: i32,
    #[pyo3(get)]
    pub humidity: i32,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapWindJson {
    #[pyo3(get)]
    pub speed: f64,
    #[pyo3(get)]
    pub deg: i16,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapCloudsJson {
    #[pyo3(get)]
    pub all: u8,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapSysJson {
    #[serde(rename = "type")]
    #[pyo3(get)]
    pub type_name: i64,
    #[pyo3(get)]
    pub id: i64,
    #[pyo3(get)]
    pub country: String,
    #[pyo3(get)]
    pub sunrise: i64,
    #[pyo3(get)]
    pub sunset: i64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapJson {
    #[pyo3(get)]
    pub coord: OpenWeatherMapCoordinatesJson,
    #[pyo3(get)]
    pub weather: Vec<OpenWeatherMapConditionJson>,
    #[pyo3(get)]
    pub base: String,
    #[pyo3(get)]
    pub main: OpenWeatherMapMainJson,
    #[pyo3(get)]
    pub visibility: i32,
    #[pyo3(get)]
    pub wind: OpenWeatherMapWindJson,
    #[pyo3(get)]
    pub clouds: OpenWeatherMapCloudsJson,
    #[pyo3(get)]
    pub sys: OpenWeatherMapSysJson,
    #[pyo3(get)]
    pub timezone: i64,
    #[pyo3(get)]
    pub id: i64,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub cod: i32,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapAirQualityItemJson {
    #[pyo3(get)]
    pub main: HashMap<String, i8>,
    #[pyo3(get)]
    pub components: HashMap<String, f64>,
    #[pyo3(get)]
    pub dt: i64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapAirQualityJson {
    #[pyo3(get)]
    pub coord: OpenWeatherMapCoordinatesJson,
    #[pyo3(get)]
    pub list: Vec<OpenWeatherMapAirQualityItemJson>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastMainJson {
    #[pyo3(get)]
    pub temp: f64,
    #[pyo3(get)]
    pub feels_like: f64,
    #[pyo3(get)]
    pub temp_min: f64,
    #[pyo3(get)]
    pub temp_max: f64,
    #[pyo3(get)]
    pub pressure: i32,
    #[pyo3(get)]
    pub sea_level: i32,
    #[pyo3(get)]
    pub grnd_level: i32,
    #[pyo3(get)]
    pub humidity: i64,
    #[pyo3(get)]
    pub temp_kf: f64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastWindJson {
    #[pyo3(get)]
    pub speed: f64,
    #[pyo3(get)]
    pub deg: i16,
    #[pyo3(get)]
    pub gust: f64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastSysJson {
    #[pyo3(get)]
    pod: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastItemJson {
    #[pyo3(get)]
    pub dt: i64,
    #[pyo3(get)]
    pub main: OpenWeatherMapForecastMainJson,
    #[pyo3(get)]
    pub weather: Vec<OpenWeatherMapConditionJson>,
    #[pyo3(get)]
    pub clouds: OpenWeatherMapCloudsJson,
    #[pyo3(get)]
    pub wind: OpenWeatherMapForecastWindJson,
    #[pyo3(get)]
    pub visibility: i32,
    #[pyo3(get)]
    pub pop: f64,
    #[pyo3(get)]
    pub sys: OpenWeatherMapForecastSysJson,
    #[pyo3(get)]
    pub dt_txt: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastJson {
    #[pyo3(get)]
    cod: String,
    #[pyo3(get)]
    message: i64,
    #[pyo3(get)]
    cnt: i64,
    #[pyo3(get)]
    pub list: Vec<OpenWeatherMapForecastItemJson>,
}
