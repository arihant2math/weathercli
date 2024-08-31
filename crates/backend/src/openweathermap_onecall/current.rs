use std::collections::HashMap;

use chrono::DateTime;
use chrono::Duration;

use simd_json;

use crate::openweathermap_onecall::json::{CurrentJson, DailyJson};
use crate::WeatherCondition;
use crate::WindData;
use crate::{get_conditions_sentence, WeatherData};

pub fn get_current(
    data: &CurrentJson,
    daily: &DailyJson,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<WeatherData> {
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    for condition in data.weather.clone() {
        conditions.push(WeatherCondition::new(condition.id, &weather_codes)?);
    }
    Ok(WeatherData {
        time: DateTime::from_timestamp(data.dt, 0)
            .ok_or("Failed to parse timestamp for data".to_string())?,
        temperature: data.temp as f32,
        min_temp: daily.temp["min"] as f32,
        max_temp: daily.temp["max"] as f32,
        wind: WindData {
            speed: data.wind_speed,
            heading: data.wind_deg,
        },
        raw_data: simd_json::to_string_pretty(data).expect("dump to string failed"),
        dewpoint: data.humidity as f32,
        feels_like: data.feels_like as f32,
        aqi: 42, // TODO: Fix
        cloud_cover: data.clouds,
        condition_sentence: get_conditions_sentence(&conditions),
        conditions,
        rain_data: weather_structs::PrecipitationData {
            amount: data.rain.unwrap_or_default().one_hour,
            time: Duration::hours(1),
            probability: 100,
        },
        snow_data: weather_structs::PrecipitationData {
            amount: data.snow.unwrap_or_default().one_hour,
            time: Duration::hours(1),
            probability: 100,
        },
    })
}
