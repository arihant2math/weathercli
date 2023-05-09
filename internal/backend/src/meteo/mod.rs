use serde::{Deserialize, Serialize};

use crate::meteo::json::{MeteoAirQualityJson, MeteoForecastJson};
use location::Coordinates;
use networking;
use networking::Resp;

pub mod forecast;
mod json;
mod weather_data;

/// Formats the urls
fn get_api_urls(location: Coordinates, metric: bool) -> [String; 2] {
    let latitude = location.latitude;
    let longitude = location.longitude;
    let base_forecast_url = "https://api.open-meteo.com/v1/forecast";
    let base_air_quaility_url = "https://air-quality-api.open-meteo.com/v1/air-quality";
    let hourly = "temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,pressure_msl,visibility,windspeed_10m,winddirection_10m";
    let daily = "temperature_2m_max,temperature_2m_min";
    let units = if metric {
        "" // TODO: Strong units here
    } else {
        "&temperature_unit=fahrenheit&windspeed_unit=mph&precipitation_unit=inch"
    };
    [format!("{base_forecast_url}?latitude={latitude}&longitude={longitude}&current_weather=true&hourly={hourly}&daily={daily}&timezone=auto{units}"),
        format!("{base_air_quaility_url}?latitude={latitude}&longitude={longitude}&hourly=european_aqi")]
}

/// Gets the urls from the meteo api server and returns a `FormattedData` struct with the data
pub fn get_combined_data_formatted(
    coordinates: Coordinates,
    metric: bool,
) -> crate::Result<MeteoFormattedData> {
    let urls = get_api_urls(coordinates, metric);
    let mut n = networking::get_urls(&urls, None, None, None)?;
    unsafe {
        let r1: MeteoForecastJson = simd_json::from_str(&mut n[0].text)?;
        let r2: MeteoAirQualityJson = simd_json::from_str(&mut n[1].text)?;
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
