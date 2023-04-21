use std::collections::HashMap;
use std::thread;

use serde_json::Value;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;

use crate::local::cache;
use crate::networking;

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
    let r = networking::get_url(format!(
            "https://dev.virtualearth.net/REST/v1/Locations?query=\"{}\"&maxResults=5&key={}",
            query, bing_maps_api_key
        ), None, None, None);
    let j: Value = serde_json::from_str(&r.text).expect("json parsing failed");
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
    let r = networking::get_url(format!("https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=jsonv2", lat, lon), None, None, None);
    r.text
}

/// :param no_sys_loc: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(target_os = "windows")]
fn get_location_core(no_sys_loc: bool) -> [String; 2] {
    // If no_sys_loc is true, the location will always be gotten from the web
    if !no_sys_loc {
        return get_location_windows().unwrap_or(
            get_location_web().expect("Windows location and web location fallback failed"),
        );
    }
    get_location_web().expect("web location not found")
}

/// :param no_sys_loc: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(not(target_os = "windows"))]
fn get_location_core(_no_sys_loc: bool) -> [String; 2] {
    get_location_web().expect("web location not found")
}

pub fn get_location(no_sys_loc: bool, constant_location: bool) -> [String; 2] {
    if constant_location {
        let attempt_cache = cache::read("current_location");
        return match attempt_cache {
            None => {
                let location = get_location_core(no_sys_loc);
                cache::write(
                    "current_location",
                    location.join(",").as_str(),
                );
                location
            }
            Some(ca) => {
                let handle = thread::spawn(|| {
                    cache::update_hits("current_location".to_string());
                });
                let splt = ca.split(',');
                let split_vec: Vec<&str> = splt.into_iter().collect();
                handle.join().expect("Update Hits Thread Failed");
                [split_vec[0].to_string(), split_vec[1].to_string()]
            }
        };
    }
    get_location_core(no_sys_loc)
}

pub fn get_coordinates(location_string: String, bing_maps_api_key: String) -> Option<[String; 2]> {
    let attempt_cache = cache::read(&("location".to_string() + &location_string));

    match attempt_cache {
        None => {
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
            let v = real_coordinate.join(",");
            thread::spawn(move || {
                cache::write(
                &("location".to_string() + &location_string.to_lowercase()),
                &v,
                );
            });
            Some([
                real_coordinate[0].to_string(),
                real_coordinate[1].to_string(),
            ])
        }
        Some(real_cache) => {
            let cache_string = "location".to_string() + &location_string.to_lowercase();
            thread::spawn(move || {
                cache::update_hits(cache_string);
            });
            let vec_collect: Vec<&str> = real_cache.split(',').collect();
            Some([vec_collect[0].to_string(), vec_collect[1].to_string()])
        }
    }
}

pub fn reverse_location(latitude: &str, longitude: &str) -> [String; 2] {
    let k = "coordinates".to_string() + latitude + "," + longitude;
    let attempt_cache = cache::read(&k);
    match attempt_cache {
        None => {
            let data = nominatim_reverse_geocode(latitude, longitude);
            let place: Value = serde_json::from_str(&data).unwrap();
            let country = place["address"]["country"].as_str().unwrap().to_string();
            let mut region = "";
            if place["address"]["city"].as_str().is_some() {
                region = place["address"]["city"].as_str().unwrap();
            } else if place["address"]["county"].as_str().is_some() {
                region = place["address"]["county"].as_str().unwrap();
            }
            let v = region.to_string() + ",?`|" + &country;
            thread::spawn(move || {
                cache::write(
                    &k,
                    &v,
                );
            });
            [region.to_string(), country]
        }
        Some(real_cache) => {
            let cache_string = "coordinates".to_string() + &k;
            thread::spawn(move || {
                cache::update_hits(cache_string);
            });
            let vec_collect: Vec<&str> = real_cache.split(",?`|").collect();
            [vec_collect[0].to_string(), vec_collect[1].to_string()]
        }
    }
}
