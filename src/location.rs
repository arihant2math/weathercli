use std::collections::HashMap;

use pyo3::prelude::*;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;

#[cfg(target_os = "windows")]
fn get_location_windows() -> Result<[String; 2], windows::core::Error> {
    let geolocator = Geolocator::new()?;
    let geolocation = geolocator.GetGeopositionAsync()?;
    let coordinates = geolocation.get()?.Coordinate()?.Point()?.Position()?;
    Ok([
        coordinates.Latitude.to_string(),
        coordinates.Longitude.to_string(),
    ])
}

fn get_location_web() -> Result<[String; 2], reqwest::Error> {
    let resp = reqwest::blocking::get("https://ipinfo.io")?.json::<HashMap<String, String>>()?;
    let location = resp.get("loc").expect("No loc section").split(',');
    let mut location_vec: Vec<String> = vec![];
    for s in location {
        location_vec.push(s.to_string());
    }
    let mut location_list = ["".to_string(), "".to_string()];
    location_list[0] = location_vec.get(0).expect("").to_string();
    location_list[1] = location_vec.get(1).expect("").to_string();
    Ok(location_list)
}

#[pyfunction]
#[cfg(target_os = "windows")]
pub fn get_location(no_sys_loc: bool) -> [String; 2] {
    // If no_sys_loc is true, the location will always be gotten from the web
    if !no_sys_loc {
        return get_location_windows().expect("windows location not found");
    }
    get_location_web().expect("web location not found")
}

#[pyfunction]
#[cfg(not(target_os = "windows"))]
pub fn get_location(no_sys_loc: bool) -> [String; 2] {
    get_location_web().expect("web location not found")
}
