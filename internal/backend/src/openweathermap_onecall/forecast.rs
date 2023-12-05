use local::location;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use location::Coordinates;
use shared_deps::bincode;
use std::collections::HashMap;

use crate::openweathermap_onecall::current::get_current;
use crate::openweathermap_onecall::future::get_future;
use crate::openweathermap_onecall::get_combined_data_formatted;
use crate::WeatherData;
use crate::WeatherForecast;

pub fn get_forecast(
    coordinates: &Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    let key = if settings.open_weather_map_one_call_key {settings.open_weather_map_api_key} else { String::from("439d4b804bc8187953eb36d2a8c26a02")};
    let data = get_combined_data_formatted(
        "https://openweathermap.org/data/2.5/",
        key,
        coordinates,
        settings.metric_default,
    )?;
    let mut forecast: Vec<WeatherData> = Vec::new();
    let weather_file = WeatherFile::weather_codes()?;
    let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
    forecast.push(get_current(
        &data.current,
        &data.daily[0],
        weather_codes.clone(),
    )?);
    for (count, item) in data.hourly.iter().enumerate() {
        forecast.push(get_future(
            item,
            &data.daily[count / 24],
            weather_codes.clone(),
        )?);
    }
    let loc = location::reverse_geocode(coordinates)?;
    Ok(WeatherForecast {
        datasource: String::from("Open Weather Map OneCall"),
        location: loc,
        forecast: forecast.clone(),
        raw_data: None,
    })
}
