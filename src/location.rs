use std::collections::HashMap;
use std::thread;

use serde_json::Value;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::PositionAccuracy;

use crate::local::cache;
use crate::networking;

#[derive(Clone, Copy)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[cfg(target_os = "windows")]
fn get_windows() -> crate::Result<Coordinates> {
    let geolocator = Geolocator::new()?;
    geolocator.SetDesiredAccuracy(PositionAccuracy::High)?;
    let geolocation = geolocator.GetGeopositionAsync()?;
    let coordinates = geolocation.get()?.Coordinate()?.Point()?.Position()?;
    Ok(Coordinates {
        latitude: coordinates.Latitude,
        longitude: coordinates.Longitude,
    })
}

fn get_web() -> crate::Result<Coordinates> {
    let resp = networking::get_url("https://ipinfo.io", None, None, None)?.text;
    let json: HashMap<String, String> = serde_json::from_str(&resp)?;
    let location_vec: Vec<&str> = json
        .get("loc")
        .ok_or("No loc section")?
        .split(',')
        .collect();
    Ok(Coordinates {
        latitude: location_vec[0].parse().unwrap(),
        longitude: location_vec[1].parse().unwrap(),
    })
}

fn bing_maps_geocode(query: &str, bing_maps_api_key: String) -> crate::Result<Coordinates> {
    let r = networking::get_url(
        format!(
            "https://dev.virtualearth.net/REST/v1/Locations?query=\"{query}\"&maxResults=5&key={bing_maps_api_key}"
        ),
        None,
        None,
        None,
    )?;
    let j: Value = serde_json::from_str(&r.text)?;
    let j_data = &j["resourceSets"][0]["resources"][0]["point"]["coordinates"];
    Ok(Coordinates {
        latitude: j_data[0].as_f64().ok_or("latitude not found")?,
        longitude: j_data[1].as_f64().ok_or("longitude not found")?,
    })
}

fn nominatim_geocode(query: &str) -> crate::Result<Coordinates> {
    let r = networking::get_url(
        format!("https://nominatim.openstreetmap.org/search?q=\"{query}\"&format=jsonv2"),
        None,
        None,
        None,
    )?;
    let j: Value = serde_json::from_str(&r.text)?;
    let latitude = j[0]["lat"]
        .as_f64()
        .ok_or("latitude not found")?
        .to_string();
    let longitude = j[0]["lon"]
        .as_f64()
        .ok_or("longitude not found")?
        .to_string();
    Ok(Coordinates {
        latitude: latitude.parse().unwrap(),
        longitude: longitude.parse().unwrap(),
    })
}

fn nominatim_reverse_geocode(coordinates: Coordinates) -> crate::Result<String> {
    let r = networking::get_url(
        format!(
            "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=jsonv2",
            coordinates.latitude, coordinates.longitude
        ),
        None,
        None,
        None,
    );
    Ok(r?.text)
}

/// :param `no_sys_loc`: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(target_os = "windows")]
fn get_location_core(no_sys_loc: bool) -> crate::Result<Coordinates> {
    // If no_sys_loc is true, the location will always be gotten from the web
    if !no_sys_loc {
        return get_windows();
    }
    get_web()
}

/// :param no_sys_loc: if true the location will not be retrieved with the OS location api,
/// by default the location is retrieved with the OS api whenever possible
#[cfg(not(target_os = "windows"))]
fn get_location_core(_no_sys_loc: bool) -> crate::Result<Coordinates> {
    get_web()
}

pub fn get(no_sys_loc: bool, constant_location: bool) -> crate::Result<Coordinates> {
    if constant_location {
        let attempt_cache = cache::read("current_location");
        return Ok(match attempt_cache {
            Err(_e) => {
                let location = get_location_core(no_sys_loc)?;
                cache::write(
                    "current_location",
                    &format!("{},{}", location.latitude, location.longitude),
                )
                .unwrap();
                location
            }
            Ok(ca) => {
                thread::spawn(|| {
                    cache::update_hits("current_location".to_string()).unwrap_or(());
                });
                let splt = ca.split(',');
                let split_vec: Vec<&str> = splt.into_iter().collect();
                Coordinates {
                    latitude: split_vec[0].to_string().parse().unwrap(),
                    longitude: split_vec[1].to_string().parse().unwrap(),
                }
            }
        });
    }
    get_location_core(no_sys_loc)
}

pub fn geocode(query: String, bing_maps_api_key: String) -> crate::Result<Coordinates> {
    let attempt_cache = cache::read(&("location".to_string() + &query));

    match attempt_cache {
        Err(_e) => {
            let mut coordinates: crate::Result<Coordinates>;
            if bing_maps_api_key.is_empty() {
                coordinates = nominatim_geocode(&query);
            } else {
                coordinates = bing_maps_geocode(&query, bing_maps_api_key);
                if coordinates.is_err() {
                    println!("Bing maps geocoding failed");
                    coordinates = nominatim_geocode(&query);
                }
            }
            let real_coordinate = coordinates?;
            let v = format!("{},{}", real_coordinate.latitude, real_coordinate.longitude);
            thread::spawn(move || {
                cache::write(&("location".to_string() + &query.to_lowercase()), &v).unwrap();
            });
            Ok(real_coordinate)
        }
        Ok(real_cache) => {
            let cache_string = "location".to_string() + &query.to_lowercase();
            thread::spawn(move || {
                cache::update_hits(cache_string).unwrap_or(());
            });
            let vec_collect: Vec<&str> = real_cache.split(',').collect();
            Ok(Coordinates {
                latitude: vec_collect[0].to_string().parse().unwrap(),
                longitude: vec_collect[1].to_string().parse().unwrap(),
            })
        }
    }
}

pub fn reverse_geocode(coordinates: Coordinates) -> crate::Result<[String; 2]> {
    let k = "coordinates".to_string()
        + &coordinates.latitude.to_string()
        + ","
        + &coordinates.longitude.to_string();
    let attempt_cache = cache::read(&k);
    match attempt_cache {
        Err(_e) => {
            let data = nominatim_reverse_geocode(coordinates)?;
            let place: Value = serde_json::from_str(&data)?;
            let country = place["address"]["country"]
                .as_str()
                .ok_or("country not found")?
                .to_string();
            let mut region = "";
            if let Some(city) = place["address"]["city"].as_str() {
                region = city;
            } else if let Some(county) = place["address"]["county"].as_str() {
                region = county;
            }
            let v = region.to_string() + ",?`|" + &country;
            thread::spawn(move || {
                cache::write(&k, &v).unwrap_or_default();
            });
            Ok([region.to_string(), country])
        }
        Ok(real_cache) => {
            let cache_string = "coordinates".to_string() + &k;
            thread::spawn(move || {
                cache::update_hits(cache_string).unwrap_or(());
            });
            let vec_collect: Vec<&str> = real_cache.split(",?`|").collect();
            Ok([vec_collect[0].to_string(), vec_collect[1].to_string()])
        }
    }
}
