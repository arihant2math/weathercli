use crate::backend;
use crate::backend::openweathermap::openweathermap_current::get_openweathermap_current;
use crate::backend::openweathermap::openweathermap_future::get_openweathermap_future;
use crate::backend::weather_data::WeatherDataRS;
use crate::backend::weather_forecast::WeatherForecastRS;
use crate::local::settings::Settings;

fn get_forecast_sentence(forecast: Vec<WeatherDataRS>) -> String {
    let data = forecast;
    let mut rain: Vec<bool> = Vec::with_capacity(16);
    let mut snow: Vec<bool> = Vec::with_capacity(16);
    for period in &data {
        if period.conditions[0].condition_id / 100 == 5 {
            rain.push(true);
            snow.push(false);
        } else if period.conditions[0].condition_id / 100 == 6 {
            snow.push(true);
            rain.push(false);
        } else {
            rain.push(false);
            snow.push(false);
        }
    }
    if data[0].conditions[0].condition_id / 100 == 5 {
        let mut t = 0;
        for i in rain {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue raining for {} hours.", t * 3);
    } else if data[0].conditions[0].condition_id / 100 == 6 {
        let mut t = 0;
        for i in snow {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue snowing for {} hours.", t * 3);
    }
    let t = rain.iter().position(|&b| b);
    if let Some(h) = t {
        return format!("It will rain in {} hours", h * 3);
    }
    let t_s = snow.iter().position(|&b| b);
    if let Some(h_s) = t_s {
        return format!("It will snow in {} hours", h_s * 3);
    }
    "Conditions are predicted to be clear for the next 3 days.".to_string()
}

pub fn get_openweathermap_forecast(
    coordinates: [&str; 2],
    settings: Settings,
) -> crate::Result<WeatherForecastRS> {
    if settings.internal.open_weather_map_api_key.is_empty() {
        panic!(
            "Improper openweathermap api key, {}",
            settings.internal.open_weather_map_api_key
        )
    }
    let data = backend::openweathermap::open_weather_map_get_combined_data_formatted(
        "https://api.openweathermap.org/data/2.5/",
        settings.internal.open_weather_map_api_key.clone(),
        coordinates,
        settings.internal.metric_default,
    )?;
    let mut forecast: Vec<WeatherDataRS> = Vec::new();
    forecast.push(get_openweathermap_current(
        data.weather.clone(),
        data.air_quality.clone(),
    )?);
    for item in data.forecast.list.into_iter() {
        forecast.push(get_openweathermap_future(item)?);
    }
    let forecast_sentence = get_forecast_sentence(forecast.clone());
    Ok(WeatherForecastRS {
        region: data.weather.name,
        country: data.weather.sys.country,
        forecast: forecast.clone(),
        current_weather: forecast.into_iter().next().unwrap(),
        forecast_sentence,
        raw_data: None,
    })
}
