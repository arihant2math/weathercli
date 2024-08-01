use std::collections::HashMap;

use local::location;
use local::location::Coordinates;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use networking;
use networking::Resp;
use shared_deps::{bincode, simd_json};
use weather_structs::{WeatherData, WeatherForecast};

use crate::openweathermap_onecall::current::get_current;
use crate::openweathermap_onecall::future::get_future;
use crate::openweathermap_onecall::json::MainJson;

mod current;
mod future;
pub mod json;

/// Gets the urls from the openweathermap api server
fn get_api_url(url: &str, api_key: &str, location: &Coordinates, metric: bool) -> String {
    let longitude = location.longitude;
    let latitude = location.latitude;
    let units = if metric { "metric" } else { "imperial" };
    format!("{url}onecall?lat={latitude}&lon={longitude}&units={units}&appid={api_key}")
}

#[derive(Copy, Clone, Debug, Default)]
pub struct OpenWeatherMapOneCall;

impl crate::Backend<MainJson> for OpenWeatherMapOneCall {
    fn get_api_urls(&self, coordinates: &Coordinates, settings: &Settings) -> Vec<String> {
        let key = if !settings.open_weather_map_one_call_key {
            &settings.open_weather_map_api_key
        } else {
            "439d4b804bc8187953eb36d2a8c26a02"
        };
        Vec::from([get_api_url(
            "https://openweathermap.org/data/2.5/",
            key,
            coordinates,
            settings.metric_default,
        )])
    }

    fn parse_data(
        &self,
        data: Vec<Resp>,
        _: &Coordinates,
        _: &Settings,
    ) -> crate::Result<MainJson> {
        let mut data = data;
        let r: MainJson = unsafe { simd_json::from_str(&mut data[0].text) }?;
        Ok(r)
    }

    fn process_data(
        &self,
        data: MainJson,
        coordinates: &Coordinates,
        _: &Settings,
    ) -> crate::Result<WeatherForecast> {
        let mut forecast: Vec<WeatherData> = Vec::new();
        let weather_file = WeatherFile::weather_codes()?;
        let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
        forecast.push(get_current(
            &data.current,
            &data.daily[0],
            weather_codes.clone(),
        )?);
        for (count, item) in data.hourly.iter().enumerate() {
            forecast.push(get_future(
                item,
                &data.daily[count / 24],
                weather_codes.clone(),
            )?);
        }
        let loc = location::reverse_geocode(coordinates)?;
        Ok(WeatherForecast {
            datasource: String::from("Open Weather Map OneCall"),
            location: loc,
            forecast: forecast.clone(),
            raw_data: None,
        })
    }
}
