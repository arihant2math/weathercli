use std::collections::HashMap;
use std::thread;

use serde_json::Value;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;
use windows::Devices::Geolocation::PositionAccuracy;

use crate::local::cache;
use crate::networking;

#[cfg(target_os = "windows")]
fn get_location_windows() -> crate::Result<[String; 2]> {
    let geolocator = Geolocator::new()?;
    geolocator.SetDesiredAccuracy(PositionAccuracy::High)?;
    let geolocation = geolocator.GetGeopositionAsync()?;
    let coordinates = geolocation.get()?.Coordinate()?.Point()?.Position()?;
    Ok([
        coordinates.Latitude.to_string(),
        coordinates.Longitude.to_string(),
    ])
}

fn get_location_web() -> crate::Result<[String; 2]> {
    let resp = networking::get_url("https://ipinfo.io", None, None, None)?.text;
    let json: HashMap<String, String> = serde_json::from_str(&resp)?;
    let location_vec: Vec<&str> = json.get("loc").ok_or_else(|| "No loc section".to_string())?.split(',').collect();
    let mut location_list = ["".to_string(), "".to_string()];
    location_list[0] = location_vec[0].to_string();
    location_list[1] = location_vec[1].to_string();
    Ok(location_list)
}

fn bing_maps_location_query(query: &str, bing_maps_api_key: String) -> crate::Result<Vec<String>> {
    let r = networking::get_url(
        format!(
            "https://dev.virtualearth.net/REST/v1/Locations?query=\"{}\"&maxResults=5&key={}",
            query, bing_maps_api_key
        ),
        None,
        None,
        None,
    )?;
    let j: Value = serde_json::from_str(&r.text)?;
    let j_data = &j["resourceSets"][0]["resources"][0]["point"]["coordinates"];
    Ok(vec![
        j_data[0].as_f64().ok_or_else(|| "latitude not found")?.to_string(),
        j_data[1].as_f64().ok_or_else(|| "longitude not found")?.to_string(),
    ])
}

fn nominatim_geocode(query: &str) -> crate::Result<Vec<String>> {
    let r = networking::get_url(format!(
            "https://nominatim.openstreetmap.org/search?q=\"{}\"&format=jsonv2",
            query
        ), None, None, None)?;
    let j: Value = serde_json::from_str(&r.text)?;
    let lat = j[0]["lat"].as_f64().ok_or_else(|| "latitude not found")?.to_string();
    let lon = j[0]["lon"].as_f64().ok_or_else(|| "longitude not found")?.to_string();
    Ok(vec![lat, lon])
}

fn nominatim_reverse_geocode(lat: &str, lon: &str) -> crate::Result<String> {
    let r = networking::get_url(
        format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=jsonv2",
            lat, lon
        ),
        None,
        None,
        None,
    );
    Ok(r?.text)
}

/// :param no_sys_loc: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(target_os = "windows")]
fn get_location_core(no_sys_loc: bool) -> crate::Result<[String; 2]> {
    // If no_sys_loc is true, the location will always be gotten from the web
    if !no_sys_loc {
        return get_location_windows();
    }
    get_location_web()
}

/// :param no_sys_loc: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(not(target_os = "windows"))]
fn get_location_core(_no_sys_loc: bool) -> crate::Result<[String; 2]> {
    get_location_web()
}

pub fn get_location(no_sys_loc: bool, constant_location: bool) -> crate::Result<[String; 2]> {
    if constant_location {
        let attempt_cache = cache::read("current_location");
        return Ok(match attempt_cache {
            None => {
                let location = get_location_core(no_sys_loc)?;
                cache::write("current_location", location.join(",").as_str());
                location
            }
            Some(ca) => {
                thread::spawn(|| {
                    cache::update_hits("current_location".to_string());
                });
                let splt = ca.split(',');
                let split_vec: Vec<&str> = splt.into_iter().collect();
                [split_vec[0].to_string(), split_vec[1].to_string()]
            }
        });
    }
    get_location_core(no_sys_loc)
}

pub fn get_coordinates(location_string: String, bing_maps_api_key: String) -> crate::Result<[String; 2]> {
    let attempt_cache = cache::read(&("location".to_string() + &location_string));

    match attempt_cache {
        None => {
            let mut coordinates: crate::Result<Vec<String>>;
            if bing_maps_api_key != *"" {
                coordinates = bing_maps_location_query(&location_string, bing_maps_api_key);
                if coordinates.is_err() {
                    println!("Bing maps geocoding failed");
                    coordinates = nominatim_geocode(&location_string);
                }
            } else {
                coordinates = nominatim_geocode(&location_string);
            }
            let real_coordinate = coordinates?;
            let v = real_coordinate.join(",");
            thread::spawn(move || {
                cache::write(
                    &("location".to_string() + &location_string.to_lowercase()),
                    &v,
                );
            });
            Ok([
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
            Ok([vec_collect[0].to_string(), vec_collect[1].to_string()])
        }
    }
}

pub fn reverse_location(latitude: &str, longitude: &str) -> crate::Result<[String; 2]> {
    let k = "coordinates".to_string() + latitude + "," + longitude;
    let attempt_cache = cache::read(&k);
    match attempt_cache {
        None => {
            let data = nominatim_reverse_geocode(latitude, longitude)?;
            let place: Value = serde_json::from_str(&data)?;
            let country = place["address"]["country"].as_str().ok_or_else(|| "country not found")?.to_string();
            let mut region = "";
            if let Some(city) = place["address"]["city"].as_str() {
                region = city;
            } else if let Some(county) = place["address"]["county"].as_str() {
                region = county;
            }
            let v = region.to_string() + ",?`|" + &country;
            thread::spawn(move || {
                cache::write(&k, &v);
            });
            Ok([region.to_string(), country])
        }
        Some(real_cache) => {
            let cache_string = "coordinates".to_string() + &k;
            thread::spawn(move || {
                cache::update_hits(cache_string);
            });
            let vec_collect: Vec<&str> = real_cache.split(",?`|").collect();
            Ok([vec_collect[0].to_string(), vec_collect[1].to_string()])
        }
    }
}
