use crate::backend::weather_condition::WeatherCondition;
use crate::backend::weather_data::{get_conditions_sentence, WeatherData};
use crate::backend::WindData;
use crate::now;
use std::collections::HashMap;
use crate::backend::openweathermap_onecall::json::MomentJson;

pub fn get_weather_data(
    data: MomentJson,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<WeatherData> {
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    for condition in data.weather.clone() {
        conditions.push(WeatherCondition::new(condition.id as u16, &weather_codes)?);
    }
    Ok(WeatherData {
        time: now() as i128,
        temperature: data.temp as f32,
        min_temp: 0.0,
        max_temp: 0.0,
        wind: WindData {
            speed: data.wind_speed,
            heading: data.wind_deg,
        },
        raw_data: serde_json::to_string_pretty(&data).expect("dump to string failed"),
        dewpoint: data.humidity as f32,
        feels_like: data.feels_like as f32,
        aqi: 0,
        cloud_cover: data.clouds,
        conditions: conditions.clone(),
        condition_sentence: get_conditions_sentence(conditions.clone()),
    })
}
