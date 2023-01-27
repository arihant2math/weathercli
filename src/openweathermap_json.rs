use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapCoordinatesJson {
    #[pyo3(get)]
    lon: f64,
    #[pyo3(get)]
    lat: f64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapConditionJson {
    #[pyo3(get)]
    id: i16,
    #[pyo3(get)]
    main: String,
    #[pyo3(get)]
    description: String,
    #[pyo3(get)]
    icon: String,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapMainJson {
    #[pyo3(get)]
    temp: f64,
    #[pyo3(get)]
    feels_like: f64,
    #[pyo3(get)]
    temp_min: f64,
    #[pyo3(get)]
    temp_max: f64,
    #[pyo3(get)]
    pressure: i32,
    #[pyo3(get)]
    humidity: i32,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapWindJson {
    #[pyo3(get)]
    speed: f64,
    #[pyo3(get)]
    deg: i16,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapCloudsJson {
    #[pyo3(get)]
    all: i8,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapSysJson {
    #[serde(rename = "type")]
    type_name: i64,
    #[pyo3(get)]
    id: i64,
    #[pyo3(get)]
    country: String,
    #[pyo3(get)]
    sunrise: i64,
    #[pyo3(get)]
    sunset: i64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapJson {
    #[pyo3(get)]
    coord: OpenWeatherMapCoordinatesJson,
    #[pyo3(get)]
    weather: Vec<OpenWeatherMapConditionJson>,
    #[pyo3(get)]
    base: String,
    #[pyo3(get)]
    main: OpenWeatherMapMainJson,
    #[pyo3(get)]
    visibility: i32,
    #[pyo3(get)]
    wind: OpenWeatherMapWindJson,
    #[pyo3(get)]
    clouds: OpenWeatherMapCloudsJson,
    #[pyo3(get)]
    sys: OpenWeatherMapSysJson,
    #[pyo3(get)]
    timezone: i64,
    #[pyo3(get)]
    id: i64,
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    cod: i32,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapAirQualityItemJson {
    #[pyo3(get)]
    main: HashMap<String, i8>,
    #[pyo3(get)]
    components: HashMap<String, f64>,
    #[pyo3(get)]
    dt: i64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapAirQualityJson {
    #[pyo3(get)]
    coord: OpenWeatherMapCoordinatesJson,
    #[pyo3(get)]
    list: Vec<OpenWeatherMapAirQualityItemJson>,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastMainJson {
    #[pyo3(get)]
    temp: f64,
    #[pyo3(get)]
    feels_like: f64,
    #[pyo3(get)]
    temp_min: f64,
    #[pyo3(get)]
    temp_max: f64,
    #[pyo3(get)]
    pressure: i32,
    #[pyo3(get)]
    sea_level: i32,
    #[pyo3(get)]
    grnd_level: i32,
    #[pyo3(get)]
    humidity: i64,
    #[pyo3(get)]
    temp_kf: f64,
}

#[pyclass]
#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapForecastWindJson {
    #[pyo3(get)]
    speed: f64,
    #[pyo3(get)]
    deg: i16,
    #[pyo3(get)]
    gust: f64,
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
    dt: i64,
    #[pyo3(get)]
    main: OpenWeatherMapForecastMainJson,
    #[pyo3(get)]
    weather: Vec<OpenWeatherMapConditionJson>,
    #[pyo3(get)]
    clouds: OpenWeatherMapCloudsJson,
    #[pyo3(get)]
    wind: OpenWeatherMapForecastWindJson,
    #[pyo3(get)]
    visibility: i32,
    #[pyo3(get)]
    pop: f64,
    #[pyo3(get)]
    sys: OpenWeatherMapForecastSysJson,
    #[pyo3(get)]
    dt_txt: String,
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
    list: Vec<OpenWeatherMapForecastItemJson>,
}
