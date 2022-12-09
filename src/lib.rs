use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_urls(url: String, api_key: String, location: String, metric: bool) -> Vec<String> {
    // Gets the urls from the server
    let mut coordinates: Vec<&str> = location.split(",").collect();
    let longitude = coordinates.pop().expect("Need both coordinates").to_string();
    let latitude = coordinates.pop().expect("Need both coordinates").to_string();
    // let mut weather_string = url.borrow() + "weather?lat=" + &latitude + "&lon=" + &longitude + "&appid=" + &api_key;
    let mut weather_string = String::from(format!("{url}weather?lat={latitude}&lon={longitude}&appid={api_key}"));
    // let mut air_quality = url.borrow() + "air_pollution?lat=" + &latitude + "&lon=" + &longitude + "&appid=" + &api_key;
    let mut air_quality = String::from(format!("{url}air_pollution?lat={latitude}&lon={longitude}&appid={api_key}"));
    // let mut forecast = url.borrow() + "forecast?lat=" + &latitude + "&lon=" + &longitude + "&appid=" + &api_key;
    let mut forecast = String::from(format!("{url}forecast?lat={latitude}&lon={longitude}&appid={api_key}"));
    if metric {
        weather_string += "&units=metric";
        air_quality += "&units=metric";
        forecast += "&units=metric";
    }
    else {
        weather_string += "&units=imperial";
        air_quality += "&units=imperial";
        forecast += "&units=imperial";
    }
    return vec![weather_string, air_quality, forecast];
}

/// A Python module implemented in Rust.
#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_urls, m)?)?;
    Ok(())
}