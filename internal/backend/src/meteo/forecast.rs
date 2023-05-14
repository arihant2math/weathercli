use crate::meteo::get_combined_data_formatted;
use crate::meteo::json::MeteoForecastJson;
use crate::meteo::weather_data::get_weather_data;
use crate::WeatherData;
use crate::WeatherForecast;
use local::location;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use location::Coordinates;
use std::collections::HashMap;

fn get_forecast_sentence(
    data: Vec<WeatherData>,
    raw_data: MeteoForecastJson,
    start: usize,
) -> String {
    let mut rain = raw_data
        .hourly
        .rain
        .iter()
        .map(|x| (x - 0.0).abs() > f32::EPSILON)
        .collect::<Vec<bool>>();
    let mut snow = raw_data
        .hourly
        .snowfall
        .iter()
        .map(|x| (x - 0.0).abs() > f32::EPSILON)
        .collect::<Vec<bool>>();
    rain.drain(0..start);
    snow.drain(0..start);
    if data[0]
        .conditions
        .clone()
        .into_iter()
        .map(|condition| condition.condition_id / 100 == 5)
        .any(|x| x)
    {
        let mut t: u8 = 0;
        for i in rain {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue raining for {t} hours.");
    }
    if data[0]
        .conditions
        .clone()
        .into_iter()
        .map(|condition| condition.condition_id / 100 == 6)
        .any(|x| x)
    {
        let t = snow.iter().position(|&b| b).unwrap_or(0);
        return format!("It will continue snowing for {t} hours.");
    }
    let rain_start = rain.clone().into_iter().position(|x| x);
    let snow_start = snow.clone().into_iter().position(|x| x);

    if rain_start.is_none() && snow_start.is_none() {
        return "Conditions are predicted to be clear for the next 7 days.".to_string();
    }
    rain.reverse();
    snow.reverse();
    let rain_end = rain.into_iter().position(|x| x);
    let snow_end = snow.into_iter().position(|x| x);
    if rain_start.is_some() {
        return format!(
            "It will rain in {} hours for {} hours",
            rain_start.unwrap(),
            rain_end.unwrap() - rain_start.unwrap()
        );
    }
    if snow_start.is_some() {
        return format!(
            "It will snow in {} hours for {} hours",
            snow_start.unwrap(),
            snow_end.unwrap() - snow_start.unwrap()
        );
    }
    String::from("Conditions are predicted to be clear for the next 7 days.")
}

pub fn get_forecast(
    coordinates: Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    let data = get_combined_data_formatted(coordinates, settings.metric_default)?;
    let mut forecast: Vec<WeatherData> = Vec::new();
    let now = data
        .weather
        .hourly
        .time
        .iter()
        .position(|r| *r == data.weather.current_weather.time)
        .expect("now not found");
    let weather_file = WeatherFile::weather_codes()?;
    let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
    let current = get_weather_data(
        data.weather.clone(),
        data.air_quality.clone(),
        now,
        settings.metric_default,
        weather_codes.clone(),
    )?;
    forecast.push(current);
    for i in now + 1..data.weather.hourly.time.len() - 1 {
        forecast.push(get_weather_data(
            data.weather.clone(),
            data.air_quality.clone(),
            i,
            settings.metric_default,
            weather_codes.clone(),
        )?);
    }
    let region_country = location::reverse_geocode(coordinates)?;
    let forecast_sentence = get_forecast_sentence(forecast.clone(), data.weather, now);
    let f = WeatherForecast {
        region: region_country[0].clone(),
        country: region_country[1].clone(),
        forecast: forecast.clone(),
        current_weather: forecast.into_iter().next().unwrap(),
        forecast_sentence,
        raw_data: None,
    };
    Ok(f)
}
