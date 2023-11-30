use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use weather_plugin::{chrono, WeatherData, WeatherForecast, WindData};
use weather_plugin::custom_backend::PluginRegistrar;
use weather_plugin::custom_backend::WeatherForecastPlugin;
use weather_plugin::export_plugin;
use weather_plugin::location::Coordinates;
use weather_plugin::settings::Settings;

mod json;

fn get_the_weather_channel_current(data: &Value) -> weather_plugin::Result<WeatherData> {
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    let current_data_total: &Map<String, Value> = data["dal"]["getSunV3CurrentObservationsUrlConfig"].as_object().unwrap();
    let key = current_data_total.keys().find(|_| true).unwrap();
    let current_data: &Map<String, Value> = current_data_total[key]["data"].as_object().unwrap();
    Ok(WeatherData {
        time: chrono::offset::Utc::now(),
        temperature: current_data["temperature"].as_f64().unwrap() as f32,
        min_temp: current_data["temperatureMin24Hour"].as_f64().unwrap() as f32,
        max_temp: current_data["temperatureMax24Hour"].as_f64().unwrap() as f32,
        wind: WindData {
            speed: current_data["windSpeed"].as_f64().unwrap(),
            heading: current_data["windDirection"].as_i64().unwrap() as u16,
        },
        raw_data: serde_json::to_string_pretty(&data).unwrap(),
        dewpoint: current_data["temperatureDewPoint"].as_f64().unwrap() as f32,
        feels_like: current_data["temperatureFeelsLike"].as_f64().unwrap() as f32,
        aqi: 0,
        cloud_cover: 0,
        conditions: vec![],
        condition_sentence: "WIP".to_string(),
    })
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct RequestArg {
    name: String,
    value: HashMap<String, String>,
}

fn get_the_weather_channel_forecast(coordinates: &Coordinates, settings: Settings) -> weather_plugin::Result<WeatherForecast> {
    let mut headers = HashMap::new();
    if !settings.metric_default {
        headers.insert("unitOfMeasurement".to_string(), "e".to_string());
    } else {
        headers.insert("unitOfMeasurement".to_string(), "m".to_string());
    }
    let str_coordinates = format!("{},{}", coordinates.latitude, coordinates.longitude);
    let mut request_args: Vec<RequestArg> = Vec::new();
    let mut default_hashmap = HashMap::new();
    if !settings.metric_default {
        default_hashmap.insert(String::from("units"), String::from("e"));
    } else {
        default_hashmap.insert(String::from("units"), String::from("m"));
    }
    default_hashmap.insert(String::from("geocode"), str_coordinates);
    request_args.push(RequestArg {
        name: String::from("getSunWeatherAlertHeadlinesUrlConfig"),
        value: default_hashmap.clone(),
    });
    request_args.push(RequestArg {
        name: String::from("getSunV3CurrentObservationsUrlConfig"),
        value: default_hashmap.clone(),
    });
    // All this to make it look legit
    let mut headers = HashMap::new();
    headers.insert(String::from("Accept"), String::from("application/json"));
    headers.insert(String::from("Content-Type"), String::from("application/json"));
    headers.insert(String::from("Host"), String::from("weather.com"));
    headers.insert(String::from("Origin"), String::from("https://weather.com"));
    headers.insert(String::from("Referer"), String::from("https://weather.com/"));
    headers.insert(String::from("DNT"), String::from("1"));
    headers.insert(String::from("Sec-Fetch-Dest"), String::from("empty"));
    headers.insert(String::from("Sec-Fetch-Mode"), String::from("cors"));
    headers.insert(String::from("Sec-Fetch-Site"), String::from("same-origin"));
    headers.insert(String::from("Sec-GPC"), String::from("1"));
    headers.insert(String::from("TE"), String::from("trailers"));
    let mut cookies = HashMap::new();
    cookies.insert(String::from("__adblocker"), String::from("false"));
    cookies.insert(String::from("wxu-user-poll"), String::from("skip"));
    cookies.insert(String::from("fv"), String::from("3"));
    let default_data = r#"[{"name":"getSunV3CurrentObservationsUrlConfig","params":{"geocode":"37.35,-121.95","units":"e"}},{"name":"getSunV3DailyForecastWithHeadersUrlConfig","params":{"duration":"7day","geocode":"37.35,-121.95","units":"e"}}]"#;
    let resp = weather_plugin::networking::post_url("https://weather.com/api/v1/p/redux-dal",
                                                    // Some(serde_json::to_string(&request_args)?),
                                                    Some(default_data.to_string()),
                                                    Some(weather_plugin::networking::SNEAK_USER_AGENT),
                                                    Some(headers),
                                                    Some(cookies))?;
    let j = serde_json::from_str(&resp.text)?;
    let current = get_the_weather_channel_current(&j)?;

    let forecast = vec![current.clone()];
    let loc = weather_plugin::location::reverse_geocode(coordinates)?;
    Ok(WeatherForecast {
        location: loc,
        datasource: String::from("theweatherchannel"),
        forecast,
        forecast_sentence: String::from("WIP"),
        raw_data: Some(vec![resp.text]), // TODO: Fix (pretty print with serde maybe)
    })
}

export_plugin!(register);

extern "C" fn register(registrar: &mut dyn PluginRegistrar) {
    registrar.register_function("theweatherchannel", Box::new(TheWeatherChannel));
}

#[derive(Debug, Clone, PartialEq)]
pub struct TheWeatherChannel;

impl WeatherForecastPlugin for TheWeatherChannel {
    fn call(&self, coordinates: &Coordinates, settings: Settings) -> weather_plugin::Result<WeatherForecast> {
        get_the_weather_channel_forecast(coordinates, settings)
    }

    fn name(&self) -> Option<&str> {
        Some("theweatherchannel")
    }

    fn aliases(&self) -> Option<Vec<&str>> {
        // Some(vec!["weatherchannel", "weather.com", "theweather.com", "theweatherchannel.com"])
        None
    }

    fn help(&self) -> Option<&str> {
        Some("A weather channel api backend (weather.com)")
    }
}



#[cfg(test)]
mod tests {
    use weather_plugin::location;

    use crate::get_the_weather_channel_forecast;

    #[test]
    fn test_main() {
        let coordinates = location::Coordinates {
            latitude: 37.354,
            longitude: -121.955,
        };
        get_the_weather_channel_forecast(&coordinates, weather_plugin::settings::Settings::new().unwrap()).unwrap(); // TODO: Bad bc it uses actual settings when testing
    }
}
