use crate::backend::openweathermap_onecall::json::MainJson;

use crate::location::Coordinates;
use networking;

pub mod forecast;
pub mod json;
mod weather_data;

/// Gets the urls from the openweathermap api server
fn get_api_url(url: &str, api_key: String, location: Coordinates, metric: bool) -> String {
    let longitude = location.longitude;
    let latitude = location.latitude;
    let units = if metric { "metric" } else { "imperial" };
    format!("{url}onecall?lat={latitude}&lon={longitude}&units={units}&appid={api_key}")
}

/// Gets the urls from the openweathermap api server and returns a `FormattedData` struct with the data
pub fn get_combined_data_formatted(
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
    let mut n = networking::get_url(&url, None, None, None)?;
    let r: MainJson = unsafe { simd_json::from_str(&mut n.text) }?;
    Ok(r)
}
