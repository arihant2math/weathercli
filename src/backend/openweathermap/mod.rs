use serde::{Deserialize, Serialize};

use crate::backend::openweathermap::openweathermap_json::{
    OpenWeatherMapAirQualityJson, OpenWeatherMapForecastJson, OpenWeatherMapJson,
};
use crate::networking;
use crate::networking::Resp;

mod openweathermap_current;
pub mod openweathermap_forecast;
mod openweathermap_future;
pub mod openweathermap_json;

/// Gets the urls from the openweathermap api server
pub fn open_weather_map_get_api_urls(
    url: &str,
    api_key: String,
    location: Vec<String>,
    metric: bool,
) -> Vec<String> {
    let longitude = location.get(1).expect("");
    let latitude = location.get(0).expect("");
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
    vec![weather_string, air_quality, forecast]
}

/// Gets the urls from the openweathermap api server and returns a FormattedData struct with the data
pub fn open_weather_map_get_combined_data_formatted(
    open_weather_map_api_url: &str,
    open_weather_map_api_key: String,
    coordinates: Vec<String>,
    metric: bool,
) -> crate::Result<OpenWeatherMapFormattedData> {
    let urls = open_weather_map_get_api_urls(
        open_weather_map_api_url,
        open_weather_map_api_key,
        coordinates,
        metric,
    );
    let n = networking::get_urls(urls, None, None, None)?;
    let r1: OpenWeatherMapJson = serde_json::from_str(&n[0].text)?;
    let r2: OpenWeatherMapAirQualityJson = serde_json::from_str(&n[1].text)?;
    let r3: OpenWeatherMapForecastJson = serde_json::from_str(&n[2].text)?;
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