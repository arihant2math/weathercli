use serde::{Deserialize, Serialize};

use crate::backend::meteo::meteo_json::{MeteoAirQualityJson, MeteoForecastJson};
use crate::networking;
use crate::networking::Resp;

mod meteo_current;
pub mod meteo_forecast;
mod meteo_json;

/// Gets the urls from the openweathermap api server
pub fn meteo_get_api_urls(location: Vec<String>, metric: bool) -> Vec<String> {
    let longitude = location.get(0).expect("");
    let latitude = location.get(1).expect("");
    return if !metric {
        vec![format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,pressure_msl,visibility,windspeed_10m,winddirection_10m&daily=temperature_2m_max,temperature_2m_min&temperature_unit=fahrenheit&windspeed_unit=mph&precipitation_unit=inch&timezone=auto",
                     longitude, latitude),
             format!("https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi", longitude, latitude),
        ]
    } else {
        vec![format!("https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current_weather=true&hourly=temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,visibility,windspeed_10m,winddirection_10m&daily=temperature_2m_max,temperature_2m_min&timezone=auto", longitude, latitude),
             format!("https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&hourly=european_aqi", longitude, latitude),
        ]
    };
}

/// Gets the urls from the meteo api server and returns a FormattedData struct with the data
pub fn meteo_get_combined_data_formatted(
    coordinates: Vec<String>,
    metric: bool,
) -> MeteoFormattedData {
    let urls = meteo_get_api_urls(coordinates, metric);
    let n = networking::get_urls(urls, None, None, None);
    let r1: MeteoForecastJson = serde_json::from_str(&n[0].text).expect("");
    let r2: MeteoAirQualityJson = serde_json::from_str(&n[1].text).expect("");
    MeteoFormattedData {
        weather: r1,
        air_quality: r2,
        raw_data: n,
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoFormattedData {
    pub weather: MeteoForecastJson,
    pub air_quality: MeteoAirQualityJson,
    pub raw_data: Vec<Resp>,
}
