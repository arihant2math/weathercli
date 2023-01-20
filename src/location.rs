use pyo3::prelude::*;
use std::collections::HashMap;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;

#[cfg(target_os = "windows")]
fn get_location_windows() -> Result<Vec<String>, windows::core::Error> {
    let geolocator = Geolocator::new()?;
    let geolocation = geolocator.GetGeopositionAsync()?;
    let coordinates = geolocation
        .get()?
        .Coordinate()?
        .Point()?
        .Position()?;
    let latitude = coordinates.Latitude;
    let longitude = coordinates.Longitude;
    Ok(vec![latitude.to_string(), longitude.to_string()])
}

fn get_location_web() -> Result<Vec<String>, reqwest::Error> {
    let resp = reqwest::blocking::get("https://ipinfo.io")?
        .json::<HashMap<String, String>>()?;
    let location = resp.get("loc").expect("No loc section").split(',');
    let mut location_vec: Vec<String> = vec![];
    for s in location {
        location_vec.push(s.to_string());
    }
    Ok(location_vec)
}

#[pyfunction]
#[cfg(target_os = "windows")]
pub fn get_location(no_sys_loc: bool) -> Vec<String> {
    // If no_sys_loc is true, the location will always be gotten from the web
    if !no_sys_loc {
        return get_location_windows().expect("windows location not found");
    }
    get_location_web().expect("web location not found")
}

#[pyfunction]
#[cfg(not(target_os = "windows"))]
pub fn get_location(no_sys_loc: bool) -> Vec<String> {
    get_location_web().expect("web location not found")
}
