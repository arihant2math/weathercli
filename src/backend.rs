use pyo3::prelude::*;

use crate::networking;
use crate::networking::Resp;
use crate::openweathermap_json::{
    OpenWeatherMapAirQualityJson, OpenWeatherMapForecastJson, OpenWeatherMapJson,
};

pub mod weather_condition;
pub mod weather_data;
pub mod weather_forecast;
pub mod wind_data;

/// Gets the urls from the openweathermap api server
fn get_api_urls(url: String, api_key: String, location: Vec<String>, metric: bool) -> Vec<String> {
    let longitude = location.get(1).expect("");
    let latitude = location.get(0).expect("");
    let mut weather_string = format!("{url}weather?lat={latitude}&lon={longitude}&appid={api_key}");
    let mut air_quality =
        format!("{url}air_pollution?lat={latitude}&lon={longitude}&appid={api_key}");
    let mut forecast = format!("{url}forecast?lat={latitude}&lon={longitude}&appid={api_key}");
    if metric {
        weather_string += "&units=metric";
        air_quality += "&units=metric";
        forecast += "&units=metric";
    } else {
        weather_string += "&units=imperial";
        air_quality += "&units=imperial";
        forecast += "&units=imperial";
    }
    vec![weather_string, air_quality, forecast]
}

#[pyclass]
#[derive(Clone)]
struct FormattedData {
    #[pyo3(get)]
    weather: OpenWeatherMapJson,
    #[pyo3(get)]
    air_quality: OpenWeatherMapAirQualityJson,
    #[pyo3(get)]
    forecast: OpenWeatherMapForecastJson,
    #[pyo3(get)]
    raw_data: Vec<Resp>,
}

/// Gets the urls from the openweathermap api server and returns a FormattedData struct with the data
#[pyfunction]
fn open_weather_map_get_combined_data_formatted(
    open_weather_map_api_url: String,
    open_weather_map_api_key: String,
    coordinates: Vec<String>,
    metric: bool,
) -> FormattedData {
    let urls = get_api_urls(
        open_weather_map_api_url,
        open_weather_map_api_key,
        coordinates,
        metric,
    );
    let n = networking::get_urls(urls, None, None, None);
    let r1: OpenWeatherMapJson = serde_json::from_str(&n[0].text).expect("");
    let r2: OpenWeatherMapAirQualityJson = serde_json::from_str(&n[1].text).expect("");
    let r3: OpenWeatherMapForecastJson = serde_json::from_str(&n[2].text).expect("");
    FormattedData {
        weather: r1,
        air_quality: r2,
        forecast: r3,
        raw_data: n,
    }
}

pub fn register_backend_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "backend")?;
    child_module.add_function(wrap_pyfunction!(
        open_weather_map_get_combined_data_formatted,
        child_module
    )?)?;
    child_module.add_class::<FormattedData>()?;
    child_module.add_class::<wind_data::WindData>()?;
    child_module.add_class::<weather_data::WeatherData>()?;
    child_module.add_class::<weather_condition::WeatherCondition>()?;
    child_module.add_class::<weather_forecast::WeatherForecast>()?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
