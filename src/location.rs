use pyo3::prelude::*;
use serde_json::Value;
use std::collections::HashMap;
use std::thread;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;

use crate::local::cache;

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

fn bing_maps_location_query(query: &str, bing_maps_api_key: String) -> Option<Vec<String>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("weathercli/1")
        .build()
        .unwrap();
    let r = client
        .get(format!(
            "https://dev.virtualearth.net/REST/v1/Locations?query=\"{}\"&maxResults=5&key={}",
            query, bing_maps_api_key
        ))
        .send()
        .unwrap();
    let j: Value = r.json::<Value>().expect("json parsing failed");
    let j_data = &j["resourceSets"][0]["resources"][0]["point"]["coordinates"];
    Some(vec![
        j_data[0].as_f64()?.to_string(),
        j_data[1].as_f64()?.to_string(),
    ])
}

fn nominatim_geocode(query: &str) -> Option<Vec<String>> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("weathercli/1")
        .build()
        .unwrap();
    let r = client
        .get(format!(
            "https://nominatim.openstreetmap.org/search?q=\"{}\"&format=jsonv2",
            query
        ))
        .send()
        .unwrap();
    let j: Value = r.json::<Value>().expect("json parsing failed");
    let lat = j[0]["lat"].as_f64()?.to_string();
    let lon = j[0]["lon"].as_f64()?.to_string();
    Some(vec![lat, lon])
}

fn nominatim_reverse_geocode(lat: &str, lon: &str) -> String {
    let client = reqwest::blocking::Client::builder()
        .user_agent("weathercli/1")
        .build()
        .unwrap();
    let r = client
        .get(format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=jsonv2",
            lat, lon
        ))
        .send()
        .unwrap();
    r.text().unwrap()
}

/// :param no_sys_loc: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(target_os = "windows")]
fn get_location_core(no_sys_loc: bool) -> [String; 2] {
    // If no_sys_loc is true, the location will always be gotten from the web
    if !no_sys_loc {
        return get_location_windows().expect("windows location not found");
    }
    get_location_web().expect("web location not found")
}

/// :param no_sys_loc: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(not(target_os = "windows"))]
fn get_location_core(_no_sys_loc: bool) -> [String; 2] {
    get_location_web().expect("web location not found")
}

// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
//
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//
//     handle.join().unwrap();
// }

#[pyfunction]
pub fn get_location(no_sys_loc: bool, constant_location: bool) -> [String; 2] {
    if constant_location {
        let attempt_cache = cache::read("current_location".to_string());
        return if let Some(..) = attempt_cache {
            let location = get_location_core(no_sys_loc);
            cache::write(
                "current_location".to_string(),
                location.join(",").as_str().to_string(),
            );
            location
        } else {
            let handle = thread::spawn(|| {
                cache::update_hits("current_location".to_string());
            });
            let ca = attempt_cache.unwrap();
            let splt = ca.split(',');
            let split_vec: Vec<&str> = splt.into_iter().collect();
            handle.join().expect("Update Hits Thread Failed");
            [split_vec[0].to_string(), split_vec[1].to_string()]
        };
    }
    get_location_core(no_sys_loc)
}

#[pyfunction]
fn get_coordinates(location_string: String, bing_maps_api_key: String) -> Option<[String; 2]> {
    let attempt_cache = cache::read("location".to_string() + &location_string);
    return if let Some(..) = attempt_cache {
        let mut coordinates: Option<Vec<String>>;
        if bing_maps_api_key != *"" {
            coordinates = bing_maps_location_query(&location_string, bing_maps_api_key);
            if coordinates.is_none() {
                println!("Bing maps geocoding failed");
                coordinates = nominatim_geocode(&location_string);
            }
        } else {
            coordinates = nominatim_geocode(&location_string);
        }
        coordinates.as_ref()?;
        let real_coordinate = coordinates.unwrap();
        cache::write(
            "location".to_string() + &location_string.to_lowercase(),
            real_coordinate.join(",").as_str().to_string(),
        );
        Some([
            real_coordinate[0].to_string(),
            real_coordinate[1].to_string(),
        ])
    } else {
        let cache_string = "location".to_string() + &location_string.to_lowercase();
        let handle = thread::spawn(|| {
            cache::update_hits(cache_string);
        });
        let real_cache = attempt_cache.unwrap();
        let vec_collect: Vec<&str> = real_cache.split(',').collect();
        handle.join().expect("Update Hits Thread Failed");
        Some([vec_collect[0].to_string(), vec_collect[1].to_string()])
    };
}

#[pyfunction]
fn reverse_location(latitude: f64, longitude: f64) -> [String; 2] {
    let k = latitude.to_string() + "," + &longitude.to_string();
    let attempt_cache = cache::read("coordinates".to_string() + &k);
    if attempt_cache.is_none() {
        let data = nominatim_reverse_geocode(&latitude.to_string(), &longitude.to_string());
        let place: Value = serde_json::from_str(&data).unwrap();
        let country = place["address"]["country"].as_str().unwrap().to_string();
        let mut region = "";
        if place["address"]["city"].as_str().is_some() {
            region = place["address"]["city"].as_str().unwrap();
        } else if place["address"]["county"].as_str().is_some() {
            region = place["address"]["county"].as_str().unwrap();
        }
        cache::write(
            "coordinate".to_string() + &k,
            region.to_string() + ",?`|" + &country,
        );
        [region.to_string(), country]
    } else {
        let cache_string = "coordinates".to_string() + &k;
        let handle = thread::spawn(|| {
            cache::update_hits(cache_string);
        });
        let real_cache = attempt_cache.unwrap();
        let vec_collect: Vec<&str> = real_cache.split(",?`|").collect();
        handle.join().expect("Update Hits Thread Failed");
        [vec_collect[0].to_string(), vec_collect[1].to_string()]
    }
}

pub fn register_location_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "location")?;
    child_module.add_function(wrap_pyfunction!(get_location, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(get_coordinates, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(reverse_location, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
