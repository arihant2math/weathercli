use std::collections::HashMap;

use serde_json::json;
use serde_json::Value;

use weather_plugin::{WeatherData, WeatherForecast, WindData};
use weather_plugin::custom_backend::PluginRegistrar;
use weather_plugin::custom_backend::WeatherForecastPlugin;
use weather_plugin::export_plugin;
use weather_plugin::location::Coordinates;
use weather_plugin::now;
use weather_plugin::settings::Settings;

fn get_the_weather_channel_current(data: &Value) -> WeatherData {
    WeatherData {
        time: now() as i128,
        temperature: 0.0,
        min_temp: 0.0,
        max_temp: 0.0,
        wind: WindData {
            speed: 0.0,
            heading: 0,
        },
        raw_data: String::new(),
        dewpoint: 0.0,
        feels_like: 0.0,
        aqi: 0,
        cloud_cover: 0,
        conditions: vec![],
        condition_sentence: "WIP".to_string(),
    }
}

fn get_the_weather_channel_forecast(coordinates: [&str; 2], settings: Settings) -> weather_plugin::Result<WeatherForecast> {
    let region_country = weather_plugin::location::reverse_geocode(Coordinates {
        latitude: coordinates[0].parse().unwrap(),
        longitude: coordinates[1].parse().unwrap()
    })?;
    let mut headers = HashMap::new();
    if !settings.metric_default {
        headers.insert("unitOfMeasurement".to_string(), "e".to_string());
    } else {
        headers.insert("unitOfMeasurement".to_string(), "m".to_string());
    }
    let request_args = json!([{"name":"getSunWeatherAlertHeadlinesUrlConfig","params":{"geocode":"37.35,-121.95","units":"e"}},{"name":"getSunV3CurrentObservationsUrlConfig","params":{"geocode":"37.35,-121.95","units":"e"}},{"name":"getSunV3DailyForecastWithHeadersUrlConfig","params":{"duration":"7day","geocode":"37.35,-121.95","units":"e"}}]);
    // TODO: Browser user agent
    let resp = weather_plugin::networking::post_url("",
                                                    Some(serde_json::to_string(&request_args)?),
                                                    Some(""), None, None)?;
    let current = get_the_weather_channel_current(weather_soup);
    let forecast = vec![current.clone()];
    let region = &region_country.clone()[0];
    let country = &region_country.clone()[1];
    Ok(WeatherForecast {
        datasource: String::from("theweatherchannel"),
        region: region.to_string(),
        country: country.to_string(),
        forecast,
        current_weather: current,
        forecast_sentence: String::from("WIP"),
        raw_data: None,
    })
}

export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function("theweatherchannel", Box::new(TheWeatherChannel));
}

#[derive(Debug, Clone, PartialEq)]
pub struct TheWeatherChannel;

impl WeatherForecastPlugin for TheWeatherChannel {
    fn call(&self, coordinates: [&str; 2], settings: Settings) -> weather_plugin::Result<WeatherForecast> {
        get_the_weather_channel_forecast(coordinates, settings)
    }

    fn name(&self) -> Option<&str> {
        Some("theweatherchannel")
    }

    fn help(&self) -> Option<&str> {
        Some("A weather channel api backend (weather.com)")
    }
}