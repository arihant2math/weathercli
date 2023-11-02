use crate::openweathermap_onecall::get_combined_data_formatted;
use crate::openweathermap_onecall::weather_data::get_weather_data;
use crate::WeatherData;
use crate::WeatherForecast;
use local::location;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use location::Coordinates;
use std::collections::HashMap;

// TODO: add minute precision
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
        return format!("It will continue raining for {t} hours.");
    } else if data[0].conditions[0].condition_id / 100 == 6 {
        let mut t = 0;
        for i in snow {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue snowing for {t} hours.");
    }
    let t = rain.iter().position(|&b| b);
    if let Some(h) = t {
        return format!("It will rain in {h} hours");
    }
    let t_s = snow.iter().position(|&b| b);
    if let Some(h_s) = t_s {
        return format!("It will snow in {h_s} hours");
    }
    "Conditions are predicted to be clear for the next 3 days.".to_string()
}

pub fn get_forecast(
    coordinates: Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    let data = get_combined_data_formatted(
        "https://openweathermap.org/data/2.5/",
        "439d4b804bc8187953eb36d2a8c26a02".to_string(),
        coordinates,
        settings.metric_default,
    )?;
    let mut forecast: Vec<WeatherData> = Vec::new();
    let weather_file = WeatherFile::weather_codes()?;
    let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
    forecast.push(get_weather_data(
        &data.current,
        &data.daily[0],
        weather_codes.clone(),
    )?);
    for (count, item) in data.hourly.iter().enumerate() {
        forecast.push(get_weather_data(
            item,
            &data.daily[count / 24],
            weather_codes.clone(),
        )?); //TODO: Fix
    }
    let region_country = location::reverse_geocode(coordinates)?;
    let forecast_sentence = get_forecast_sentence(forecast.clone());
    Ok(WeatherForecast {
                datasource: String::from("Open Weather Map OneCall"),
        region: region_country[0].clone(),
        country: region_country[1].clone(),
        forecast: forecast.clone(),
        current_weather: forecast.into_iter().next().unwrap(),
        forecast_sentence,
        raw_data: None,
    })
}
