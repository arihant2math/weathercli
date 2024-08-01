use std::collections::HashMap;
use std::io;

use log::{debug, error, trace};
use thiserror::Error;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::Geolocator;
#[cfg(target_os = "windows")]
use windows::Devices::Geolocation::PositionAccuracy;

use networking;
use shared_deps::serde_json::Value;
use shared_deps::simd_json;
use weather_dirs::cache_dir;
pub use weather_structs::Coordinates;
pub use weather_structs::LocationData;

use crate::cache_v2;
use crate::cache_v2::Cacheable;
use crate::json::bing::{BingJSON, ResourceSetsJSON};

#[derive(Debug, Error)]
pub enum CoordinateError {
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[cfg(target_os = "windows")]
    #[error("Win API error: {0}")]
    WinAPIError(#[from] windows::core::Error),
    #[error("JSON Error: {0}")]
    JSONError(#[from] shared_deps::simd_json::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),
    #[error("Weather Dirs Error: {0}")]
    WeatherDirsError(#[from] weather_dirs::Error),
    #[error("Cache Read Error: {0}")]
    CacheReadError(#[from] cache_v2::CacheReadError),
    #[error("Cache Item Read Error: {0}")]
    CacheItemReadError(#[from] cache_v2::CacheItemReadError),
    #[error("Cache Write Error: {0}")]
    CacheWriteError(#[from] cache_v2::CacheWriteError),
    #[error("Cache Item Write Error: {0}")]
    CacheItemWriteError(#[from] cache_v2::CacheItemWriteError),
    #[error("Parsing Error: {0}")]
    ParsingError(String),
    #[error("Server Error")]
    ServerError,
}

#[derive(Debug, Error)]
pub enum GeocodeError {
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[error("JSON Error: {0}")]
    JSONError(#[from] shared_deps::simd_json::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),
    #[error("Longitude not found")]
    Longitude,
    #[error("Latitude not found")]
    Latitude,
    #[error("Server Error")]
    BackendError,
}

#[derive(Debug, Error)]
pub enum ReverseGeocodeError {
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[error("JSON Error: {0}")]
    JSONError(#[from] shared_deps::simd_json::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] io::Error),
    #[error("Weather Dirs Error: {0}")]
    WeatherDirsError(#[from] weather_dirs::Error),
    #[error("Cache Read Error: {0}")]
    CacheReadError(#[from] cache_v2::CacheReadError),
    #[error("Cache Item Read Error: {0}")]
    CacheItemReadError(#[from] cache_v2::CacheItemReadError),
    #[error("Cache Write Error: {0}")]
    CacheWriteError(#[from] cache_v2::CacheWriteError),
    #[error("Cache Item Write Error: {0}")]
    CacheItemWriteError(#[from] cache_v2::CacheItemWriteError),
    #[error("Country not found")]
    CountryNotFound,
    #[error("Server Error")]
    BackendError,
}

#[cfg(target_os = "windows")]
fn get_windows() -> Result<Coordinates, CoordinateError> {
    let geolocator = Geolocator::new()?;
    geolocator.SetDesiredAccuracy(PositionAccuracy::High)?;
    let geolocation = geolocator.GetGeopositionAsync()?;
    let coordinates = geolocation.get()?.Coordinate()?.Point()?.Position()?;
    Ok(Coordinates {
        latitude: coordinates.Latitude,
        longitude: coordinates.Longitude,
    })
}

fn get_web() -> Result<Coordinates, CoordinateError> {
    let mut resp = networking::get!("https://ipinfo.io/json")?.text;
    let resp_dbg = resp.clone();
    trace!("{}", resp_dbg);
    let json: HashMap<String, String> = unsafe { simd_json::from_str(&mut resp)? };
    let location_vec: Vec<&str> = json
        .get("loc")
        .ok_or(CoordinateError::ServerError)?
        .split(',')
        .collect();
    Ok(Coordinates {
        latitude: location_vec[0].parse().unwrap(),
        longitude: location_vec[1].parse().unwrap(),
    })
}

fn bing_maps_geocode(
    query: &str,
    bing_maps_api_key: &str,
) -> Result<ResourceSetsJSON, GeocodeError> {
    let mut r = networking::get!(
        format!(
            "https://dev.virtualearth.net/REST/v1/Locations?query=\"{query}\"&maxResults=5&key={bing_maps_api_key}"
        )
    )?;
    if r.status > 399 {
        Err(GeocodeError::BackendError)?;
    }
    let j: BingJSON = unsafe { simd_json::from_str(&mut r.text) }?;
    Ok(j.resource_sets[0].clone())
}

fn nominatim_geocode(query: &str) -> Result<Vec<Coordinates>, GeocodeError> {
    // TODO: return multiple results
    let mut r = networking::get!(format!(
        "https://nominatim.openstreetmap.org/search?q=\"{query}\"&format=jsonv2"
    ))?;
    let j: Value = unsafe { simd_json::from_str(&mut r.text) }?;
    let latitude: f64 = j[0]["lat"]
        .as_str()
        .ok_or(GeocodeError::Latitude)?
        .parse()
        .unwrap();
    let longitude: f64 = j[0]["lon"]
        .as_str()
        .ok_or(GeocodeError::Longitude)?
        .parse()
        .unwrap();
    Ok(vec![Coordinates {
        latitude,
        longitude,
    }])
}

fn nominatim_reverse_geocode(coordinates: &Coordinates) -> Result<String, ReverseGeocodeError> {
    let r = networking::get!(format!(
        "https://nominatim.openstreetmap.org/reverse?lat={}&lon={}&format=jsonv2",
        coordinates.latitude, coordinates.longitude
    ))?;
    Ok(r.text)
}

#[cfg(target_os = "windows")]
fn get_location_core(no_sys_loc: bool) -> Result<Coordinates, CoordinateError> {
    // If no_sys_loc is true, the location will always be gotten from the web
    if !no_sys_loc {
        return get_windows();
    }
    get_web()
}

#[cfg(not(target_os = "windows"))]
fn get_location_core(_no_sys_loc: bool) -> Result<Coordinates, CoordinateError> {
    get_web()
}

/// Gets the curent location of the user.
/// # Safety
/// This method interacts directly with the operating system when on Windows.
pub fn get_location(
    no_sys_loc: bool,
    constant_location: bool,
) -> Result<Coordinates, CoordinateError> {
    if constant_location {
        let attempt_cache =
            Coordinates::from_file(&cache_dir()?.join("current_location.cache")).ok(); // TODO: handle error
        return Ok(match attempt_cache {
            Some(coordinates) => coordinates,
            None => {
                let location = get_location_core(no_sys_loc)?;
                location.to_file(&cache_dir()?.join("current_location.cache"))?;
                location
            }
        });
    }
    get_location_core(no_sys_loc)
}

#[deprecated]
pub fn get(no_sys_loc: bool, constant_location: bool) -> Result<Coordinates, CoordinateError> {
    get_location(no_sys_loc, constant_location)
}

pub fn geocode(query: String, bing_maps_api_key: &str) -> Result<Coordinates, GeocodeError> {
    let coordinates: Result<Coordinates, GeocodeError>;
    if !bing_maps_api_key.is_empty() {
        debug!("Using bing maps geocode");
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
            debug!("Using openweathermap geocode");
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
                .map(|coordinate| format!("{},{}", coordinate.latitude, coordinate.longitude))
                .collect();
            let index = terminal::prompt::radio(&formatted_coordinates_list, 0, None)?;
            coordinates = Ok(coordinates_list[index]);
        } else {
            coordinates = Ok(coordinates_list[0]);
        }
    }
    let real_coordinate = coordinates?;
    Ok(real_coordinate)
}

fn option_to_string(option: Option<&str>) -> Option<String> {
    option.map(std::string::ToString::to_string)
}

pub fn reverse_geocode(coordinates: &Coordinates) -> Result<LocationData, ReverseGeocodeError> {
    let reverse_geocode_cache_path = cache_dir()?.join("location.cache");

    let mut cache = HashMap::<Coordinates, LocationData>::from_file(&reverse_geocode_cache_path)
        .unwrap_or_default(); // TODO: handle error
    unsafe {
        match cache.get(&coordinates) {
            Some(cached) => Ok(cached.clone()),
            None => {
                let mut data = nominatim_reverse_geocode(coordinates)?;
                let place: Value = simd_json::from_str(&mut data)?;
                let country = place["address"]["country"]
                    .as_str()
                    .ok_or(ReverseGeocodeError::CountryNotFound)?
                    .to_string();
                let village: Option<String> =
                    option_to_string(place["address"]["village"].as_str());
                let suburb: Option<String> = option_to_string(place["address"]["suburb"].as_str());
                let city: Option<String> = option_to_string(place["address"]["city"].as_str());
                #[allow(clippy::similar_names)]
                let county: Option<String> = option_to_string(place["address"]["county"].as_str());
                let state: Option<String> = option_to_string(place["address"]["state"].as_str());
                let loc = LocationData {
                    country,
                    city,
                    state,
                    county,
                    village,
                    suburb,
                };
                cache.insert(*coordinates, loc.clone());
                cache
                    .to_file(&reverse_geocode_cache_path)
                    .unwrap_or_else(|e| {
                        error!("Failed to write cache: {}", e);
                    });
                Ok(loc)
            }
        }
    }
}
