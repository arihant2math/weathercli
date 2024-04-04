use chrono::{Duration, Utc};
use extism_pdk::*;
use weather_structs::LocationData;
use weather_structs::{
    PrecipitationData, WasmPluginInput, WeatherCondition, WeatherData, WeatherForecast, WindData,
};

#[plugin_fn]
pub fn name() -> FnResult<String> {
    Ok(format!("wasm_test"))
}

#[plugin_fn]
pub fn version() -> FnResult<String> {
    Ok(format!("0.1.0"))
}

#[plugin_fn]
pub fn about() -> FnResult<String> {
    Ok(format!(
        "This is a wasm test plugin, all data returned is meaningless."
    ))
}

#[plugin_fn]
pub fn get_forecast(input: Vec<u8>) -> FnResult<Vec<u8>> {
    let input: WasmPluginInput = bincode::deserialize(&input)?;
    let loc = LocationData {
        village: None,
        suburb: None,
        city: None,
        county: None,
        state: None,
        country: "Nan".to_string(),
    };
    let forecast = WeatherForecast {
        location: loc,
        datasource: "wasm_test".into(),
        forecast: vec![WeatherData {
            time: Utc::now(),
            temperature: 5.0,
            min_temp: -1.0,
            max_temp: 11.0,
            wind: WindData {
                speed: 5.0,
                heading: 30,
            },
            raw_data: String::new(),
            dewpoint: 60.0,
            feels_like: 10.0,
            aqi: 5,
            cloud_cover: 63,
            conditions: vec![WeatherCondition {
                condition_id: 71,
                image_url: "https://openweathermap.org/img/wn/04d@4x".to_string(),
                sentence: "Blank Condition".to_string(),
                image_ascii: "No Ascii".to_string(),
            }],
            condition_sentence: "Blank".to_string(),
            rain_data: PrecipitationData {
                amount: 251.0,
                time: Duration::hours(1),
                probability: 52,
            },
            snow_data: PrecipitationData {
                amount: 392.0,
                time: Duration::hours(1),
                probability: 52,
            },
        }],
        raw_data: None,
    };
    let bytes = bincode::serialize(&forecast)?;
    Ok(bytes)
}
