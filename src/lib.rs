use pyo3::prelude::*;
use rayon::prelude::*;
use reqwest;
use sha256::try_digest;
use std::path::Path;

mod location;
mod wind_data;
mod weather_data;
mod update;
mod networking;

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
    metric: bool
) -> Vec<String> {
    let urls = get_api_urls(open_weather_map_api_url, open_weather_map_api_key,
                            coordinates, metric);
    return networking::get_urls(urls);
}

#[pyfunction]
fn hash_file(filename: String) -> String {
    let input = Path::new(&filename);
    let val = try_digest(input).unwrap();
    return val;
}

/// core module implemented in Rust.
#[pymodule]
fn core(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(location::get_location, m)?)?;
    m.add_function(wrap_pyfunction!(get_combined_data_unformatted, m)?)?;
    m.add_function(wrap_pyfunction!(hash_file, m)?)?;
    m.add_class::<wind_data::WindData>()?;
    m.add_class::<weather_data::WeatherData>()?;
    networking::register_networking_module(py, m)?;
    update::register_update_module(py, m)?;
    Ok(())
}
