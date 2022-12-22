use pyo3::prelude::*;
use sha256::try_digest;
use std::path::Path;
use crate::openweathermap_json::{OpenWeatherMapAirQualityJson, OpenWeatherMapForecastJson, OpenWeatherMapJson};

mod color;
mod location;
mod networking;
mod updater;
mod weather_data;
mod wind_data;
mod openweathermap_json;

fn get_api_urls(url: String, api_key: String, location: Vec<String>, metric: bool) -> Vec<String> {
    // Gets the urls from the server
    let longitude = location.get(1).expect("");
    let latitude = location.get(0).expect("");
    let mut weather_string = String::from(format!(
        "{url}weather?lat={latitude}&lon={longitude}&appid={api_key}"
    ));
    let mut air_quality = String::from(format!(
        "{url}air_pollution?lat={latitude}&lon={longitude}&appid={api_key}"
    ));
    let mut forecast = String::from(format!(
        "{url}forecast?lat={latitude}&lon={longitude}&appid={api_key}"
    ));
    if metric {
        weather_string += "&units=metric";
        air_quality += "&units=metric";
        forecast += "&units=metric";
    } else {
        weather_string += "&units=imperial";
        air_quality += "&units=imperial";
        forecast += "&units=imperial";
    }
    return vec![weather_string, air_quality, forecast];
}

#[pyfunction]
fn get_combined_data_unformatted(
    open_weather_map_api_url: String,
    open_weather_map_api_key: String,
    coordinates: Vec<String>,
    metric: bool,
) -> Vec<String> {
    let urls = get_api_urls(
        open_weather_map_api_url,
        open_weather_map_api_key,
        coordinates,
        metric,
    );
    return networking::get_urls(urls);
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
    raw_data: Vec<String>
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
    return FormattedData {weather: r1, air_quality: r2, forecast: r3, raw_data: n};
}



#[pyfunction]
fn hash_file(filename: String) -> String {
    let input = Path::new(&filename);
    let val = try_digest(input).unwrap();
    return val;
}

#[pyfunction]
fn color_value(value: String, units: Option<String>, color: bool) -> String {
    let lightgreen_ex;
    let magenta;
    let lightblue_ex;
    if color {
        lightgreen_ex = color::code_to_chars(92);
        magenta = color::code_to_chars(35);
        lightblue_ex = color::code_to_chars(94);
    }
    else {
        lightgreen_ex = "".to_string();
        magenta = "".to_string();
        lightblue_ex = "".to_string();
    }
    let r;
    match units {
        Some(p) => r = lightgreen_ex + &value + &magenta + &p + &lightblue_ex,
        None => r = lightgreen_ex + &value + &lightblue_ex,
    }
    return r;
}

/// core module implemented in Rust.
#[pymodule]
fn core(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(location::get_location, m)?)?;
    m.add_function(wrap_pyfunction!(get_combined_data_unformatted, m)?)?;
    m.add_function(wrap_pyfunction!(get_combined_data_formatted, m)?)?;
    m.add_function(wrap_pyfunction!(hash_file, m)?)?;
    m.add_function(wrap_pyfunction!(color_value, m)?)?;
    m.add_class::<wind_data::WindData>()?;
    m.add_class::<weather_data::WeatherData>()?;
    networking::register_networking_module(py, m)?;
    updater::register_updater_module(py, m)?;
    Ok(())
}
