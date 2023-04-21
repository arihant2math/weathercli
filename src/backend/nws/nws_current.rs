use crate::backend::nws::nws_json::NWSJSON;
use crate::backend::weather_condition::WeatherCondition;
use crate::backend::weather_data::{get_conditions_sentence, WeatherDataRS};
use crate::backend::wind_data::WindData;
use crate::local::weather_file::WeatherFile;
use crate::now;

fn convert_temp(value: f64, metric: bool) -> f64 {
    return if metric {
        value
    } else {
        value * 9.0 / 5.0 + 32.0
    };
}

fn convert_speed(value: f64, metric: bool) -> f64 {
    return if metric { value } else { value * 0.62 };
}

fn get_conditions(
    data: NWSJSON,
    metric: bool,
    index: usize,
    cloud_cover: u8,
) -> Vec<WeatherCondition> {
    let weather_file = WeatherFile::weather_codes();
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    if cloud_cover == 0 {
        conditions.push(WeatherCondition::new(800, &weather_file.data));
    } else if cloud_cover < 25 {
        conditions.push(WeatherCondition::new(801, &weather_file.data));
    } else if cloud_cover < 50 {
        conditions.push(WeatherCondition::new(802, &weather_file.data));
    } else if cloud_cover < 85 {
        conditions.push(WeatherCondition::new(803, &weather_file.data));
    } else {
        conditions.push(WeatherCondition::new(804, &weather_file.data));
    }
    if data.properties.quantitative_precipitation.values[index].value != 0.0 {
        let rain = data.properties.quantitative_precipitation.values[index].value;
        let metric = metric;
        if (0.0 < rain && rain < 0.098 && !metric) || (0.0 < rain && rain < 2.5 && metric) {
            conditions.push(WeatherCondition::new(500, &weather_file.data));
        } else if (rain < 0.39 && !metric) || (rain < 10.0 && metric) {
            conditions.push(WeatherCondition::new(501, &weather_file.data));
        } else if (rain < 2.0 && !metric) || (rain < 50.0 && metric) {
            conditions.push(WeatherCondition::new(502, &weather_file.data));
        } else if rain != 0.0 {
            conditions.push(WeatherCondition::new(503, &weather_file.data));
        }
    }
    if data.properties.snowfall_amount.values[index].value != 0.0 {
        conditions.push(WeatherCondition::new(601, &weather_file.data));
    }
    return conditions;
}

pub fn get_nws_current(data: NWSJSON, metric: bool) -> WeatherDataRS {
    let cloud_cover = data.properties.sky_cover.values[0].value as u8;
    let conditions = get_conditions(data.clone(), metric, 0, cloud_cover);
    WeatherDataRS {
        time: now() as i128,
        temperature: convert_temp(data.properties.temperature.values[0].value.clone(), metric)
            as f32,
        min_temp: convert_temp(
            data.properties.min_temperature.values[0].value.clone(),
            metric,
        ) as f32,
        max_temp: convert_temp(
            data.properties.max_temperature.values[0].value.clone(),
            metric,
        ) as f32,
        wind: WindData {
            speed: convert_speed(data.properties.wind_speed.values[0].value.clone(), metric),
            heading: data.properties.wind_direction.values[0].value.clone() as i16,
        },
        raw_data: "".to_string(),
        dewpoint: convert_temp(data.properties.dewpoint.values[0].value.clone(), metric) as f32,
        feels_like: convert_temp(
            data.properties.apparent_temperature.values[0].value.clone(),
            metric,
        ) as f32,
        aqi: 0,
        cloud_cover,
        conditions: vec![],
        condition_sentence: get_conditions_sentence(conditions),
    }
}