use serde::{Deserialize, Serialize};

use crate::openweathermap::json::{
    OpenWeatherMapAirQualityJson, OpenWeatherMapForecastJson, OpenWeatherMapJson,
};
use local::location::Coordinates;
use networking;
use networking::Resp;

mod current;
pub mod forecast;
mod future;
pub mod json;

/// Gets the urls from the openweathermap api server
fn get_api_urls(url: &str, api_key: &str, location: &Coordinates, metric: bool) -> [String; 3] {
    let longitude = location.longitude;
    let latitude = location.latitude;
    let mut weather_string = format!("{url}weather?lat={latitude}&lon={longitude}&appid={api_key}");
    let mut air_quality =
        format!("{url}air_pollution?lat={latitude}&lon={longitude}&appid={api_key}");
    let mut forecast = format!("{url}forecast?lat={latitude}&lon={longitude}&appid={api_key}");
    if metric {
        weather_string += "&units=metric";
        air_quality += "&units=metric";
        forecast += "&units=metric";
    } else {
        weather_string += "&units=imperial";
        air_quality += "&units=imperial";
        forecast += "&units=imperial";
    }
    [weather_string, air_quality, forecast]
}

/// Gets the urls from the openweathermap api server and returns a `FormattedData` struct with the data
pub fn get_combined_data_formatted(
    open_weather_map_api_url: &str,
    open_weather_map_api_key: String,
    coordinates: &Coordinates,
    metric: bool,
) -> crate::Result<OpenWeatherMapFormattedData> {
    let urls = get_api_urls(
        open_weather_map_api_url,
        &open_weather_map_api_key,
        coordinates,
        metric,
    );
    let mut n = networking::get_urls(&urls, None, None, None)?;
    let r1: OpenWeatherMapJson = unsafe { simd_json::from_str(&mut n[0].text) }?;
    let r2: OpenWeatherMapAirQualityJson = unsafe { simd_json::from_str(&mut n[1].text) }?;
    let r3: OpenWeatherMapForecastJson = unsafe { simd_json::from_str(&mut n[2].text) }?;
    Ok(OpenWeatherMapFormattedData {
        weather: r1,
        air_quality: r2,
        forecast: r3,
        raw_data: n,
    })
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OpenWeatherMapFormattedData {
    pub weather: OpenWeatherMapJson,
    pub air_quality: OpenWeatherMapAirQualityJson,
    pub forecast: OpenWeatherMapForecastJson,
    pub raw_data: Vec<Resp>,
}
