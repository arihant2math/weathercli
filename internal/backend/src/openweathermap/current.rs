use chrono::{DateTime, Duration};
use shared_deps::simd_json;
use std::collections::HashMap;

use crate::{get_conditions_sentence, WeatherData};
use crate::openweathermap::json::{OpenWeatherMapAirQualityJson, OpenWeatherMapJson};
use crate::openweathermap_shared::json::PrecipitationJson;
use crate::weather_data::PrecipitationData;
use crate::WeatherCondition;
use crate::WindData;

pub fn get_current(
    data: OpenWeatherMapJson,
    aqi: OpenWeatherMapAirQualityJson,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<WeatherData> {
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    for condition in data.weather.clone() {
        conditions.push(WeatherCondition::new(condition.id, &weather_codes)?);
    }
    Ok(WeatherData {
        time: DateTime::from_timestamp(data.dt as i64, 0).ok_or("Failed to parse current timestamp for data".to_string())?.into(),
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
        aqi: aqi.list[0]
            .main
            .get("aqi")
            .expect("aqi not found")
            .abs_diff(0),
        cloud_cover: data.clouds.all,
        conditions: conditions.clone(),
        condition_sentence: get_conditions_sentence(conditions.clone()),
        rain_data: PrecipitationData {
            amount: data.rain.unwrap_or(PrecipitationJson::default()).one_hour as f32,
            time: Duration::hours(1),
            probability: 100
        },
        snow_data: PrecipitationData {
            amount: data.snow.unwrap_or(PrecipitationJson::default()).one_hour as f32,
            time: Duration::hours(1),
            probability: 100
        },
    })
}
