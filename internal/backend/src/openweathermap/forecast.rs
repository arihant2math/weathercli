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

fn get_forecast_sentence(forecast: Vec<WeatherData>) -> String {
    let data = forecast;
    let mut rain: Vec<bool> = Vec::with_capacity(16);
    let mut snow: Vec<bool> = Vec::with_capacity(16);
    for period in &data {
        if period.conditions[0].condition_id / 100 == 5 {
            rain.push(true);
            snow.push(false);
        } else if period.conditions[0].condition_id / 100 == 6 {
            snow.push(true);
            rain.push(false);
        } else {
            rain.push(false);
            snow.push(false);
        }
    }
    if data[0].conditions[0].condition_id / 100 == 5 {
        let mut t = 0;
        for i in rain {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue raining for {} hours.", t * 3);
    } else if data[0].conditions[0].condition_id / 100 == 6 {
        let mut t = 0;
        for i in snow {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue snowing for {} hours.", t * 3);
    }
    let t = rain.iter().position(|&b| b);
    if let Some(h) = t {
        return format!("It will rain in {} hours", h * 3);
    }
    let t_s = snow.iter().position(|&b| b);
    if let Some(h_s) = t_s {
        return format!("It will snow in {} hours", h_s * 3);
    }
    "Conditions are predicted to be clear for the next 3 days.".to_string()
}

pub fn get_forecast(
    coordinates: &Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    if settings.open_weather_map_api_key.is_empty() {
        return Err(format!(
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
    let forecast_sentence = get_forecast_sentence(forecast.clone());
    let loc = location::reverse_geocode(coordinates)?;
    Ok(WeatherForecast {
        datasource: String::from("Open Weather Map"),
        location: loc,
        forecast: forecast.clone(),
        forecast_sentence,
        raw_data: None,
    })
}
