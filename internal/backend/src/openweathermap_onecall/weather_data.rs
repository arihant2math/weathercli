use shared_deps::simd_json;

use crate::openweathermap_onecall::json::{DailyJson, MomentJson};
use crate::WeatherCondition;
use crate::WindData;
use crate::{get_conditions_sentence, WeatherData};
use local::now;
use std::collections::HashMap;

pub fn get_weather_data(
    data: &MomentJson,
    daily: &DailyJson,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<WeatherData> {
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    for condition in data.weather.clone() {
        conditions.push(WeatherCondition::new(condition.id, &weather_codes)?);
    }
    Ok(WeatherData {
        time: now() as i128,
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
    })
}
