use pyo3::prelude::*;
use std::collections::HashMap;
use windows::Devices::Geolocation::Geolocator;

#[cfg(target_os = "windows")]
fn get_location_windows() -> Vec<String> {
    let geolocator = Geolocator::new().expect("Geolocator not found");
    let geolocation = geolocator
        .GetGeopositionAsync()
        .expect("Location not found");
    let coordinates = geolocation
        .get()
        .expect("geolocation not found")
        .Coordinate()
        .expect("Coordinate not found")
        .Point()
        .expect("Point not found")
        .Position()
        .expect("Position not found");
    let latitude = coordinates.Latitude;
    let longitude = coordinates.Longitude;
    return vec![latitude.to_string(), longitude.to_string()];
}

fn get_location_web() -> Vec<String> {
    let resp = reqwest::blocking::get("https://ipinfo.io")
        .expect("")
        .json::<HashMap<String, String>>()
        .expect("");
    let location = resp.get("loc").expect("No loc section").split(",");
    let mut location_vec: Vec<String> = vec![];
    for s in location {
        location_vec.push(s.to_string());
    }
    return location_vec;
}

#[pyfunction]
pub fn get_location(no_sys_loc: bool) -> Vec<String> {
    // If no_sys_loc is true, the location will always be gotten from the web
    if (cfg!(windows)) && (!no_sys_loc) {
        return get_location_windows();
    }
    return get_location_web();
}
