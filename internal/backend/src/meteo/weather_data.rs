use shared_deps::simd_json;
use crate::meteo::json::{MeteoAirQualityJson, MeteoForecastJson};
use crate::WeatherCondition;
use crate::WindData;
use crate::{get_conditions_sentence, WeatherData};
use std::collections::HashMap;
use crate::weather_condition::get_clouds_condition;
use chrono::{Utc, Duration, NaiveDateTime};

pub fn get_weather_data(
    data: MeteoForecastJson,
    aqi: MeteoAirQualityJson,
    index: usize,
    metric: bool,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<WeatherData> {
    let cloud_cover = data.hourly.cloudcover[index];
    let conditions = get_conditions(data.clone(), metric, index, cloud_cover, weather_codes)?;
    let native_time = NaiveDateTime::parse_from_str(&data.hourly.time[index], "%Y-%m-%dT%H:%M")?;
    let time = native_time.and_local_timezone(Utc).single().ok_or("timezone offset failure")?;
    let d = WeatherData {
        time,
        temperature: data.current_weather.temperature,
        min_temp: data.daily.temperature_2m_min[index / 24],
        max_temp: data.daily.temperature_2m_max[index / 24],
        wind: WindData {
            speed: data.current_weather.windspeed,
            heading: data.current_weather.winddirection as u16,
        },
        raw_data: simd_json::to_string_pretty(&data)?,
        dewpoint: data.hourly.dewpoint_2m[index],
        feels_like: data.hourly.apparent_temperature[index],
        aqi: aqi
            .hourly
            .european_aqi
            .get(index)
            .unwrap_or(&Some(0))
            .unwrap_or(0_u8),
        cloud_cover,
        conditions: conditions.clone(),
        condition_sentence: get_conditions_sentence(conditions),
        rain_data: crate::weather_data::PrecipitationData {
            amount: data.hourly.rain[index],
            time: Duration::hours(1),
            probability: data.hourly.precipitation_probability[index],
        },
        snow_data: crate::weather_data::PrecipitationData {
            amount: data.hourly.snowfall[index],
            time: Duration::hours(1),
            probability: data.hourly.precipitation_probability[index],
        },
    };
    Ok(d)
}

fn get_conditions(
    data: MeteoForecastJson,
    metric: bool,
    index: usize,
    cloud_cover: u8,
    weather_codes: HashMap<String, Vec<String>>,
) -> crate::Result<Vec<WeatherCondition>> {
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    conditions.push(get_clouds_condition(cloud_cover, &weather_codes)?);
    if data.hourly.rain[index] != 0.0 {
        let rain = data.hourly.rain[index];
        let metric = metric;
        if (0.0 < rain && rain < 0.098 && !metric) || (0.0 < rain && rain < 2.5 && metric) {
            conditions.push(WeatherCondition::new(500, &weather_codes)?);
        } else if (rain < 0.39 && !metric) || (rain < 10.0 && metric) {
            conditions.push(WeatherCondition::new(501, &weather_codes)?);
        } else if (rain < 2.0 && !metric) || (rain < 50.0 && metric) {
            conditions.push(WeatherCondition::new(502, &weather_codes)?);
        } else if rain != 0.0 {
            conditions.push(WeatherCondition::new(503, &weather_codes)?);
        }
    }
    if data.hourly.snowfall[index] != 0.0 {
        conditions.push(WeatherCondition::new(601, &weather_codes)?);
    }
    Ok(conditions)
}
