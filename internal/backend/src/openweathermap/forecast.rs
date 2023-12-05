use shared_deps::bincode;

use crate::openweathermap::current::get_current;
use crate::openweathermap::future::get_future;
use crate::WeatherData;
use crate::WeatherForecast;
use local::location;
use local::location::Coordinates;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use std::collections::HashMap;

pub fn get_forecast(
    coordinates: &Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    if settings.open_weather_map_api_key.is_empty() {
        Err(format!(
            "Improper openweathermap api key, {}",
            settings.open_weather_map_api_key
        ))?;
    }
    let data = crate::openweathermap::get_combined_data_formatted(
        "https://api.openweathermap.org/data/2.5/",
        settings.open_weather_map_api_key.clone(),
        coordinates,
        settings.metric_default,
    )?;
    let mut forecast: Vec<WeatherData> = Vec::new();
    let weather_file = WeatherFile::weather_codes()?;
    let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
    forecast.push(get_current(
        data.weather.clone(),
        data.air_quality.clone(),
        weather_codes.clone(),
    )?);
    let mut futures = data.forecast.list.iter().map(|item| get_future(item.clone(), weather_codes.clone()).unwrap()).collect();
    forecast.append(&mut futures);
    let loc = location::reverse_geocode(coordinates)?;
    Ok(WeatherForecast {
        datasource: String::from("Open Weather Map"),
        location: loc,
        forecast: forecast.clone(),
        raw_data: None,
    })
}
