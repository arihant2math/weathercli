use crate::openweathermap_onecall::json::MainJson;

use shared_deps::simd_json;

use local::location::Coordinates;
use networking;

pub mod forecast;
pub mod json;
mod future;
mod current;

/// Gets the urls from the openweathermap api server
fn get_api_url(url: &str, api_key: &str, location: &Coordinates, metric: bool) -> String {
    let longitude = location.longitude;
    let latitude = location.latitude;
    let units = if metric { "metric" } else { "imperial" };
    format!("{url}onecall?lat={latitude}&lon={longitude}&units={units}&appid={api_key}")
}

/// Gets the urls from the openweathermap api server and returns a `FormattedData` struct with the data
pub fn get_combined_data_formatted(
    open_weather_map_api_url: &str,
    open_weather_map_api_key: String,
    coordinates: &Coordinates,
    metric: bool,
) -> crate::Result<MainJson> {
    let url = get_api_url(
        open_weather_map_api_url,
        &open_weather_map_api_key,
        coordinates,
        metric,
    );
    let mut n = networking::get!(&*url, Some(networking::SNEAK_USER_AGENT))?;
    let r: MainJson = unsafe { simd_json::from_str(&mut n.text) }?;
    Ok(r)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_get_forecast() {
        let location = local::location::Coordinates {
            latitude: 37.354,
            longitude: -121.955,
        };
        let data = crate::openweathermap_onecall::forecast::get_forecast(&location, local::settings::Settings::new().unwrap()).unwrap();
    }
}
