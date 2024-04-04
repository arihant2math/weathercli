use std::collections::HashMap;

use chrono::{DateTime, Duration};

use local::weather_file::WeatherFile;
use shared_deps::bincode;
use weather_structs::get_clouds_condition;
use weather_structs::weather_data::PrecipitationData;

use crate::nws::json::NWSJSON;
use crate::WeatherCondition;
use crate::WindData;
use crate::{get_conditions_sentence, WeatherData};

fn convert_temp(value: f64, metric: bool) -> f64 {
    if metric {
        value
    } else {
        value * 9.0 / 5.0 + 32.0
    }
}

fn convert_speed(value: f64, metric: bool) -> f64 {
    if metric {
        value
    } else {
        value * 0.62
    }
}

fn get_conditions(
    data: NWSJSON,
    metric: bool,
    index: usize,
    cloud_cover: u8,
) -> crate::Result<Vec<WeatherCondition>> {
    let weather_file = WeatherFile::weather_codes()?;
    let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    conditions.push(get_clouds_condition(cloud_cover, &weather_codes)?);
    if data.properties.quantitative_precipitation.values[index]
        .value
        .unwrap_or(0.0)
        != 0.0
    {
        let rain = data.properties.quantitative_precipitation.values[index]
            .value
            .unwrap_or(0.0);
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
    if data.properties.snowfall_amount.values[index]
        .value
        .unwrap_or(0.0)
        != 0.0
    {
        conditions.push(WeatherCondition::new(601, &weather_codes)?);
    }
    Ok(conditions)
}

pub fn get_current(data: NWSJSON, metric: bool) -> crate::Result<WeatherData> {
    let cloud_cover = data.properties.sky_cover.values[0].value.unwrap_or(-1) as u8;
    let conditions = get_conditions(data.clone(), metric, 0, cloud_cover)?;
    let d = WeatherData {
        time: DateTime::parse_from_rfc3339(&data.properties.temperature.values[0].valid_time)?
            .into(),
        temperature: convert_temp(
            data.properties.temperature.values[0]
                .value
                .unwrap_or(-2048.0),
            metric,
        ) as f32,
        min_temp: convert_temp(
            data.properties.min_temperature.values[0]
                .value
                .unwrap_or(-2048.0),
            metric,
        ) as f32,
        max_temp: convert_temp(
            data.properties.max_temperature.values[0]
                .value
                .unwrap_or(-2048.0),
            metric,
        ) as f32,
        wind: WindData {
            speed: convert_speed(
                data.properties.wind_speed.values[0].value.unwrap_or(-1.0),
                metric,
            ),
            heading: data.properties.wind_direction.values[0]
                .value
                .unwrap_or(-2048) as u16,
        },
        raw_data: String::new(),
        dewpoint: convert_temp(
            data.properties.dewpoint.values[0].value.unwrap_or(-2048.0),
            metric,
        ) as f32,
        feels_like: convert_temp(
            data.properties.apparent_temperature.values[0]
                .value
                .unwrap_or(-2048.0),
            metric,
        ) as f32,
        aqi: 0,
        cloud_cover,
        condition_sentence: get_conditions_sentence(&conditions),
        conditions: conditions,
        rain_data: PrecipitationData {
            amount: data.properties.quantitative_precipitation.values[0]
                .value
                .unwrap_or(-1.0) as f32,
            probability: (data.properties.probability_of_precipitation.values[0]
                .value
                .unwrap_or(-1.0)
                * 100.0) as u8,
            time: Duration::hours(1),
        },
        snow_data: PrecipitationData {
            amount: data.properties.snowfall_amount.values[0]
                .value
                .unwrap_or(-1.0) as f32,
            probability: (data.properties.probability_of_precipitation.values[0]
                .value
                .unwrap_or(-1.0)
                * 100.0) as u8,
            time: Duration::hours(1),
        },
    };
    Ok(d)
}
