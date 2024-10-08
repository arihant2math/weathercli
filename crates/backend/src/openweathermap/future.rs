use std::collections::HashMap;

use chrono::{DateTime, Duration};

use simd_json;
use weather_structs::PrecipitationData;
use weather_structs::WeatherCondition;
use weather_structs::WindData;
use weather_structs::{get_conditions_sentence, WeatherData};

use crate::openweathermap::json::OpenWeatherMapForecastItemJson;

pub fn get_future(
    data: OpenWeatherMapForecastItemJson,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<WeatherData> {
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    for condition in data.weather.clone() {
        conditions.push(WeatherCondition::new(condition.id, &weather_codes)?);
    }
    Ok(WeatherData {
        time: DateTime::from_timestamp(data.dt, 0)
            .ok_or("Failed to parse future timestamp for data".to_string())?,
        temperature: data.main.temp as f32,
        min_temp: data.main.temp_min as f32,
        max_temp: data.main.temp_max as f32,
        wind: WindData {
            speed: data.wind.speed,
            heading: data.wind.deg,
        },
        raw_data: simd_json::to_string_pretty(&data).expect("dump to string failed"),
        dewpoint: data.main.humidity as f32,
        feels_like: data.main.feels_like as f32,
        aqi: 0,
        cloud_cover: data.clouds.all,
        condition_sentence: get_conditions_sentence(&conditions),
        conditions: conditions,
        rain_data: PrecipitationData {
            amount: data.rain.unwrap_or_default().three_hour,
            time: Duration::hours(3),
            probability: (data.pop * 100.0) as u8,
        },
        snow_data: PrecipitationData {
            amount: data.snow.unwrap_or_default().three_hour,
            time: Duration::hours(3),
            probability: (data.pop * 100.0) as u8,
        },
    })
}
