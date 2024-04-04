use crate::meteo::get_combined_data_formatted;
use crate::meteo::weather_data::get_weather_data;
use crate::WeatherData;
use crate::WeatherForecast;
use local::location;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use location::Coordinates;
use shared_deps::bincode;
use std::collections::HashMap;

pub fn get_forecast(
    coordinates: &Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    let data = get_combined_data_formatted(coordinates, settings.metric_default)?;
    let mut forecast: Vec<WeatherData> = Vec::new();
    let now = data
        .weather
        .hourly
        .time
        .iter()
        .position(|r| *r == data.weather.current_weather.time)
        .unwrap_or(0); // TODO: Log warning
    let weather_file = WeatherFile::weather_codes()?;
    let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
    let current = get_weather_data(
        data.weather.clone(),
        data.air_quality.clone(),
        now,
        settings.metric_default,
        weather_codes.clone(),
    )?;
    forecast.push(current);
    for i in now + 1..data.weather.hourly.time.len() - 1 {
        forecast.push(get_weather_data(
            data.weather.clone(),
            data.air_quality.clone(),
            i,
            settings.metric_default,
            weather_codes.clone(),
        )?);
    }
    let loc = location::reverse_geocode(coordinates)?;
    let f = WeatherForecast {
        datasource: String::from("meteo"),
        location: loc,
        forecast: forecast.clone(),
        raw_data: None,
    };
    Ok(f)
}
