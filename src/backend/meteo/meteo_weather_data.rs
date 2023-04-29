use crate::backend::meteo::meteo_json::{MeteoAirQualityJson, MeteoForecastJson};
use crate::backend::weather_condition::WeatherCondition;
use crate::backend::weather_data::{get_conditions_sentence, WeatherData};
use crate::backend::WindData;
use crate::local::weather_file::WeatherFile;
use crate::now;

pub fn get_meteo_weather_data(
    data: MeteoForecastJson,
    aqi: MeteoAirQualityJson,
    index: usize,
    metric: bool,
) -> crate::Result<WeatherData> {
    let cloud_cover = data.hourly.cloudcover[index];
    let conditions = get_conditions(data.clone(), metric, index, cloud_cover)?;
    let d = WeatherData {
        time: now() as i128,
        temperature: data.current_weather.temperature,
        min_temp: data.daily.temperature_2m_min[index / 24],
        max_temp: data.daily.temperature_2m_max[index / 24],
        wind: WindData {
            speed: data.current_weather.windspeed,
            heading: data.current_weather.winddirection as i16,
        },
        raw_data: serde_json::to_string_pretty(&data)?,
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
    };
    Ok(d)
}

fn get_conditions(
    data: MeteoForecastJson,
    metric: bool,
    index: usize,
    cloud_cover: u8,
) -> crate::Result<Vec<WeatherCondition>> {
    let weather_file = WeatherFile::weather_codes()?;
    let mut conditions: Vec<WeatherCondition> = Vec::new();
    if cloud_cover == 0 {
        conditions.push(WeatherCondition::new(800, &weather_file.data)?);
    } else if cloud_cover < 25 {
        conditions.push(WeatherCondition::new(801, &weather_file.data)?);
    } else if cloud_cover < 50 {
        conditions.push(WeatherCondition::new(802, &weather_file.data)?);
    } else if cloud_cover < 85 {
        conditions.push(WeatherCondition::new(803, &weather_file.data)?);
    } else {
        conditions.push(WeatherCondition::new(804, &weather_file.data)?);
    }
    if data.hourly.rain[index] != 0.0 {
        let rain = data.hourly.rain[index];
        let metric = metric;
        if (0.0 < rain && rain < 0.098 && !metric) || (0.0 < rain && rain < 2.5 && metric) {
            conditions.push(WeatherCondition::new(500, &weather_file.data)?);
        } else if (rain < 0.39 && !metric) || (rain < 10.0 && metric) {
            conditions.push(WeatherCondition::new(501, &weather_file.data)?);
        } else if (rain < 2.0 && !metric) || (rain < 50.0 && metric) {
            conditions.push(WeatherCondition::new(502, &weather_file.data)?);
        } else if rain != 0.0 {
            conditions.push(WeatherCondition::new(503, &weather_file.data)?);
        }
    }
    if data.hourly.snowfall[index] != 0.0 {
        conditions.push(WeatherCondition::new(601, &weather_file.data)?);
    }
    Ok(conditions)
}
