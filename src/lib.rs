use pyo3::prelude::*;
use rayon::prelude::*;
use reqwest;

mod location;
mod wind_data;
mod weather_data;

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

fn get_urls(urls: Vec<String>) -> Vec<String> {
    let data : Vec<_>= urls
        .par_iter()
        .map(|url| reqwest::blocking::get(url).expect("Url Get failed").text().expect("text expected"))
        .collect();

    return data;
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
    return get_urls(urls);
}

/// core module implemented in Rust.
#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(location::get_location, m)?)?;
    m.add_function(wrap_pyfunction!(get_combined_data_unformatted, m)?)?;
    m.add_class::<wind_data::WindData>()?;
    m.add_class::<weather_data::WeatherData>()?;
    Ok(())
}
