use networking;
use shared_deps::serde_json::Value;
use shared_deps::simd_json;
use shared_deps::simd_json::prelude::ArrayTrait;
use std::collections::HashMap;
use std::thread;
pub use weather_structs::Coordinates;
pub use weather_structs::LocationData;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::PositionAccuracy;

use crate::cache;
use crate::json::bing::{BingJSON, ResourceSetsJSON};

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
    let mut resp = networking::get!("https://ipinfo.io")?.text;
    let json: HashMap<String, String> = unsafe { simd_json::from_str(&mut resp)? };
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

fn bing_maps_geocode(query: &str, bing_maps_api_key: &str) -> crate::Result<ResourceSetsJSON> {
    let mut r = networking::get!(
        format!(
            "https://dev.virtualearth.net/REST/v1/Locations?query=\"{query}\"&maxResults=5&key={bing_maps_api_key}"
        )
    )?;
    if r.status > 399 {
        Err("Bing maps geocoding failed")?;
    }
    let j: BingJSON = unsafe { simd_json::from_str(&mut r.text) }?;
    Ok(j.resource_sets[0].clone())
}

fn nominatim_geocode(query: &str) -> crate::Result<Vec<Coordinates>> { // TODO: return multiple results
    let mut r = networking::get!(format!(
        "https://nominatim.openstreetmap.org/search?q=\"{query}\"&format=jsonv2"
    ))?;
    let j: Value = unsafe { simd_json::from_str(&mut r.text) }?;
    let latitude = j[0]["lat"]
        .as_f64()
        .ok_or("latitude not found")?
        .to_string();
    let longitude = j[0]["lon"]
        .as_f64()
        .ok_or("longitude not found")?
        .to_string();
    Ok(vec![Coordinates {
        latitude: latitude.parse().unwrap(),
        longitude: longitude.parse().unwrap(),
    }])
}

fn nominatim_reverse_geocode(coordinates: &Coordinates) -> crate::Result<String> {
    let r = networking::get!(format!(
        "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=jsonv2",
        coordinates.latitude, coordinates.longitude
    ))?;
    Ok(r.text)
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
                    cache::update_hits("current_location").unwrap_or(());
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

pub fn geocode(query: String, bing_maps_api_key: &str) -> crate::Result<Coordinates> {
    let coordinates: crate::Result<Coordinates>;
    if !bing_maps_api_key.is_empty() {
        let coordinates_list = bing_maps_geocode(&query, bing_maps_api_key)?.resources;

        if coordinates_list.len() > 1 {
            println!("Multiple choices found, please choose one");
            let formatted_coordinates_list: Vec<String> = coordinates_list
                .iter()
                .map(|coordinate| {
                    format!("{} ({} Confidence)", coordinate.name, coordinate.confidence)
                })
                .collect();
            let index = terminal::prompt::radio(&formatted_coordinates_list, 0, None)?;
            coordinates = Ok(Coordinates {
                latitude: coordinates_list[index].point.coordinates[0],
                longitude: coordinates_list[index].point.coordinates[1],
            });
        } else {
            let index = 0;
            coordinates = Ok(Coordinates {
                latitude: coordinates_list[index].point.coordinates[0],
                longitude: coordinates_list[index].point.coordinates[1],
            });
        }
    } else {
        let coordinates_list = nominatim_geocode(&query)?;
        if coordinates_list.len() > 1 {
            println!("Multiple choices found, please choose one");
            let formatted_coordinates_list: Vec<String> = coordinates_list
                .iter()
                .map(|coordinate| {
                    format!("{},{}", coordinate.latitude, coordinate.longitude)
                })
                .collect();
            let index = terminal::prompt::radio(&formatted_coordinates_list, 0, None)?;
            coordinates = Ok(coordinates_list[index]);
        } else {
            coordinates = Ok(coordinates_list[0]);
        }
    }
    let real_coordinate = coordinates?;
    let v = format!("{},{}", real_coordinate.latitude, real_coordinate.longitude);
    thread::spawn(move || {
        cache::write(&("location".to_string() + &query.to_lowercase()), &v).unwrap();
    });
    Ok(real_coordinate)
}

fn option_to_string(option: Option<&str>) -> Option<String> {
    option.map(std::string::ToString::to_string)
}

pub fn reverse_geocode(coordinates: &Coordinates) -> crate::Result<LocationData> {
    fn convert_string(s: String) -> Option<String> {
        if s.is_empty() {
            None
        } else {
            Some(s)
        }
    }

    let k = "coordinates".to_string()
        + &coordinates.latitude.to_string()
        + ","
        + &coordinates.longitude.to_string();
    let attempt_cache = cache::read(&k);
    unsafe {
        match attempt_cache {
            Err(_e) => {
                let mut data = nominatim_reverse_geocode(coordinates)?;
                let place: Value = simd_json::from_str(&mut data)?;
                let country = place["address"]["country"]
                    .as_str()
                    .ok_or("country not found")?
                    .to_string();
                let village: Option<String> =
                    option_to_string(place["address"]["village"].as_str());
                let suburb: Option<String> = option_to_string(place["address"]["suburb"].as_str());
                let city: Option<String> = option_to_string(place["address"]["city"].as_str());
                #[allow(clippy::similar_names)]
                let county: Option<String> = option_to_string(place["address"]["county"].as_str());
                let state: Option<String> = option_to_string(place["address"]["state"].as_str());
                let v = format!(
                    "{}||{}||{}||{}||{}||{}",
                    village.clone().unwrap_or_default(),
                    suburb.clone().unwrap_or_default(),
                    city.clone().unwrap_or_default(),
                    county.clone().unwrap_or_default(),
                    state.clone().unwrap_or_default(),
                    country
                );
                thread::spawn(move || {
                    cache::write(&k, &v).unwrap_or_default();
                });
                Ok(LocationData {
                    village,
                    suburb,
                    city,
                    county,
                    state,
                    country,
                })
            }
            Ok(real_cache) => {
                let cache_string = "coordinates".to_string() + &k;
                thread::spawn(move || {
                    cache::update_hits(&cache_string).unwrap_or(());
                });
                let vec_collect: Vec<&str> = real_cache.split("||").collect();
                if vec_collect.len() != 6 {
                    cache::delete(&k)?; // Important it works or else it will be stuck in an infinite loop
                    return reverse_geocode(coordinates);
                }
                let village_string = vec_collect[0].to_string();
                let suburb_string = vec_collect[1].to_string();
                let city_string = vec_collect[2].to_string();
                let county_string = vec_collect[3].to_string();
                let state_string = vec_collect[4].to_string();
                let country = vec_collect[5].to_string();

                let village = convert_string(village_string);
                let suburb = convert_string(suburb_string);
                let city = convert_string(city_string);
                #[allow(clippy::similar_names)]
                let county = convert_string(county_string);
                let state = convert_string(state_string);
                Ok(LocationData {
                    village,
                    suburb,
                    city,
                    county,
                    state,
                    country,
                })
            }
        }
    }
}
