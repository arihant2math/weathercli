use shared_deps::simd_json;
use chrono::DateTime;
use chrono::Duration;

use crate::openweathermap_onecall::json::{DailyJson, MomentJson};
use crate::WeatherCondition;
use crate::WindData;
use crate::{get_conditions_sentence, WeatherData};
use std::collections::HashMap;
use crate::openweathermap_shared::json::PrecipitationJson;

pub fn get_future(
    data: &MomentJson,
    daily: &DailyJson,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<WeatherData> {
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    for condition in data.weather.clone() {
        conditions.push(WeatherCondition::new(condition.id, &weather_codes)?);
    }
    Ok(WeatherData {
        time: DateTime::from_timestamp(data.dt as i64, 0).ok_or("Failed to parse timestamp for data".to_string())?.into(),
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
        conditions: conditions.clone(),
        condition_sentence: get_conditions_sentence(conditions.clone()),
        rain_data: crate::weather_data::PrecipitationData {
            amount: data.rain.unwrap_or(PrecipitationJson::default()).one_hour,
            time: Duration::hours(1),
            probability: (data.pop * 100.0) as u8,
        },
        snow_data: crate::weather_data::PrecipitationData {
            amount: data.snow.unwrap_or(PrecipitationJson::default()).one_hour,
            time: Duration::hours(1),
            probability: (data.pop * 100.0) as u8,
        },
    })
}