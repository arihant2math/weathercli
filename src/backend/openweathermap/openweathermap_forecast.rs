use crate::backend;
use crate::backend::openweathermap::openweathermap_current::get_openweathermap_current;
use crate::backend::openweathermap::openweathermap_future::get_openweathermap_future;
use crate::backend::status::Status;
use crate::backend::weather_data::WeatherDataRS;
use crate::backend::weather_forecast::WeatherForecastRS;
use crate::local::settings::Settings;

fn get_forecast_sentence(forecast: Vec<WeatherDataRS>) -> String {
    let data = forecast;
    let mut rain: Vec<bool> = Vec::new();
    let mut snow: Vec<bool> = Vec::new();
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
    } else {
        let mut t = 0;
        for period in rain {
            if period {
                return format!("It will rain in {} hours", t * 3);
            }
            t += 1
        }
        t = 0;
        for period in snow {
            if period {
                return format!("It will snow in {} hours", t * 3);
            }
            t += 1
        }
    }
    "Conditions are predicted to be clear for the next 3 days.".to_string()
}

pub fn get_openweathermap_forecast(
    coordinates: Vec<String>,
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
        status: Status::OK,
        region: data.weather.name,
        country: data.weather.sys.country,
        forecast: forecast.clone(),
        current_weather: forecast.into_iter().next().unwrap(),
        forecast_sentence,
        raw_data: None,
    })
}
