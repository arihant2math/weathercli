use crate::backend::openweathermap_onecall::json::MainJson;

use crate::location::Coordinates;
use crate::networking;

pub mod forecast;
mod weather_data;
pub mod json;

/// Gets the urls from the openweathermap api server
pub fn get_api_url(url: &str, api_key: String, location: Coordinates, metric: bool) -> String {
    let longitude = location.longitude;
    let latitude = location.latitude;
    let mut weather_string = format!("{url}onecall?lat={latitude}&lon={longitude}");
    if metric {
        weather_string += "&units=metric";
    } else {
        weather_string += "&units=imperial";
    }
    weather_string += "&appid=";
    weather_string += &api_key;
    weather_string
}

/// Gets the urls from the openweathermap api server and returns a `FormattedData` struct with the data
pub fn open_weather_map_get_combined_data_formatted(
    open_weather_map_api_url: &str,
    open_weather_map_api_key: String,
    coordinates: Coordinates,
    metric: bool,
) -> crate::Result<MainJson> {
    let url = get_api_url(
        open_weather_map_api_url,
        open_weather_map_api_key,
        coordinates,
        metric,
    );
    let n = networking::get_url(&url, None, None, None)?;
    let r: MainJson = serde_json::from_str(&n.text)?;
    Ok(r)
}
