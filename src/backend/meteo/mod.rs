use serde::{Deserialize, Serialize};

use crate::backend::meteo::json::{MeteoAirQualityJson, MeteoForecastJson};
use crate::location::Coordinates;
use crate::networking;
use crate::networking::Resp;

pub mod forecast;
mod json;
mod weather_data;

/// Gets the urls from the openweathermap api server
pub fn meteo_get_api_urls(location: Coordinates, metric: bool) -> [String; 2] {
    let latitude = location.latitude;
    let longitude = location.longitude;
    if metric {
        [format!("https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current_weather=true&hourly=temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,visibility,windspeed_10m,winddirection_10m&daily=temperature_2m_max,temperature_2m_min&timezone=auto"),
            format!("https://air-quality-api.open-meteo.com/v1/air-quality?latitude={latitude}&longitude={longitude}&hourly=european_aqi"),
        ]
    } else {
        [format!("https://api.open-meteo.com/v1/forecast?latitude={latitude}&longitude={longitude}&current_weather=true&hourly=temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,pressure_msl,visibility,windspeed_10m,winddirection_10m&daily=temperature_2m_max,temperature_2m_min&temperature_unit=fahrenheit&windspeed_unit=mph&precipitation_unit=inch&timezone=auto", ),
            format!("https://air-quality-api.open-meteo.com/v1/air-quality?latitude={latitude}&longitude={longitude}&hourly=european_aqi"),
        ]
    }
}

/// Gets the urls from the meteo api server and returns a `FormattedData` struct with the data
pub fn meteo_get_combined_data_formatted(
    coordinates: Coordinates,
    metric: bool,
) -> crate::Result<MeteoFormattedData> {
    let urls = meteo_get_api_urls(coordinates, metric);
    let mut n = networking::get_urls(&urls, None, None, None)?;
    unsafe {
        let r1: MeteoForecastJson = simd_json::serde::from_str(&mut n[0].text)?;
        let r2: MeteoAirQualityJson = simd_json::serde::from_str(&mut n[1].text)?;
        Ok(MeteoFormattedData {
            weather: r1,
            air_quality: r2,
            raw_data: n,
        })
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoFormattedData {
    pub weather: MeteoForecastJson,
    pub air_quality: MeteoAirQualityJson,
    pub raw_data: Vec<Resp>,
}
