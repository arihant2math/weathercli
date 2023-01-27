use crate::openweathermap_json::{OpenWeatherMapAirQualityJson, OpenWeatherMapForecastJson, OpenWeatherMapJson};
use pyo3::prelude::*;
use sha256::try_digest;
use std::path::Path;

mod location;
mod networking;
mod openweathermap_json;
mod updater;
mod weather_data;
mod wind_data;

fn get_api_urls(url: String, api_key: String, location: Vec<String>, metric: bool) -> Vec<String> {
    // Gets the urls from the server
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
    raw_data: Vec<String>,
}

#[pyfunction]
fn get_combined_data_formatted(
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
    let n = networking::get_urls(urls);
    let r1: OpenWeatherMapJson = serde_json::from_str(n.get(0).expect("")).expect("");
    let r2: OpenWeatherMapAirQualityJson = serde_json::from_str(n.get(1).expect("")).expect("");
    let r3: OpenWeatherMapForecastJson = serde_json::from_str(n.get(2).expect("")).expect("");
    FormattedData {
        weather: r1,
        air_quality: r2,
        forecast: r3,
        raw_data: n,
    }
}

#[pyfunction]
fn hash_file(filename: String) -> String {
    let input = Path::new(&filename);
    try_digest(input).unwrap()
}

/// core module implemented in Rust.
#[pymodule]
fn core(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(location::get_location, module)?)?;
    module.add_function(wrap_pyfunction!(get_combined_data_formatted, module)?)?;
    module.add_function(wrap_pyfunction!(hash_file, module)?)?;
    module.add_class::<wind_data::WindData>()?;
    module.add_class::<weather_data::WeatherData>()?;
    networking::register_networking_module(py, module)?;
    updater::register_updater_module(py, module)?;
    Ok(())
}
