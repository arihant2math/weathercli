use pyo3::prelude::*;
use windows::Devices::Geolocation::Geolocator;
// use futures::executor;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn get_urls(url: String, api_key: String, location: String, metric: bool) -> Vec<String> {
    // Gets the urls from the server
    let mut coordinates: Vec<&str> = location.split(",").collect();
    let longitude = coordinates.pop().expect("Need both coordinates").to_string();
    let latitude = coordinates.pop().expect("Need both coordinates").to_string();
    let mut weather_string = String::from(format!("{url}weather?lat={latitude}&lon={longitude}&appid={api_key}"));
    let mut air_quality = String::from(format!("{url}air_pollution?lat={latitude}&lon={longitude}&appid={api_key}"));
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

#[pyfunction]
fn get_location_windows() -> Vec<String> {
    let geolocator = Geolocator::new().expect("Geolocator not found");
    let geolocation = geolocator.GetGeopositionAsync().expect("Location not found");
    let coordinates = geolocation.get().expect("geolocation not found").Coordinate().expect("Coordinate not found").Point().expect("Point not found").Position().expect("Position not found");
    let latitude = coordinates.Latitude;
    let longitude = coordinates.Longitude;
    return vec![latitude.to_string(), longitude.to_string()];
}

/// core module implemented in Rust.
#[pymodule]
fn core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_urls, m)?)?;
    m.add_function(wrap_pyfunction!(get_location_windows, m)?)?;
    Ok(())
}
