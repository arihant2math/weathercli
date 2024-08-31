use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use bincode;
use local::location;
use local::location::Coordinates;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use networking;
use networking::Resp;
use simd_json;
use weather_structs::{WeatherData, WeatherForecast};

use crate::openweathermap::current::get_current;
use crate::openweathermap::future::get_future;
use crate::openweathermap::json::{
    OpenWeatherMapAirQualityJson, OpenWeatherMapForecastJson, OpenWeatherMapJson,
};

mod current;
mod future;
pub mod json;

#[derive(Clone, Serialize, Deserialize)]
pub struct OpenWeatherMapFormattedData {
    pub weather: OpenWeatherMapJson,
    pub air_quality: OpenWeatherMapAirQualityJson,
    pub forecast: OpenWeatherMapForecastJson,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct OpenWeatherMap;

impl crate::Backend<OpenWeatherMapFormattedData> for OpenWeatherMap {
    fn get_api_urls(&self, coordinates: &Coordinates, settings: &Settings) -> Vec<String> {
        let longitude = coordinates.longitude;
        let latitude = coordinates.latitude;
        let url = "https://api.openweathermap.org/data/2.5/";
        let api_key = settings.open_weather_map_api_key.clone();
        let mut weather_string =
            format!("{url}weather?lat={latitude}&lon={longitude}&appid={api_key}");
        let mut air_quality =
            format!("{url}air_pollution?lat={latitude}&lon={longitude}&appid={api_key}");
        let mut forecast = format!("{url}forecast?lat={latitude}&lon={longitude}&appid={api_key}");
        if settings.metric_default {
            weather_string += "&units=metric";
            air_quality += "&units=metric";
            forecast += "&units=metric";
        } else {
            weather_string += "&units=imperial";
            air_quality += "&units=imperial";
            forecast += "&units=imperial";
        }
        Vec::from([weather_string, air_quality, forecast])
    }

    fn parse_data(
        &self,
        data: Vec<Resp>,
        _: &Coordinates,
        settings: &Settings,
    ) -> crate::Result<OpenWeatherMapFormattedData> {
        // TODO: check before sending requests ...
        if settings.open_weather_map_api_key.is_empty() {
            Err(format!(
                "Improper openweathermap api key, {}",
                settings.open_weather_map_api_key
            ))?;
        }
        let mut n = data;
        let r1: OpenWeatherMapJson = unsafe { simd_json::from_str(&mut n[0].text) }?;
        let r2: OpenWeatherMapAirQualityJson = unsafe { simd_json::from_str(&mut n[1].text) }?;
        let r3: OpenWeatherMapForecastJson = unsafe { simd_json::from_str(&mut n[2].text) }?;
        Ok(OpenWeatherMapFormattedData {
            weather: r1,
            air_quality: r2,
            forecast: r3,
        })
    }

    fn process_data(
        &self,
        data: OpenWeatherMapFormattedData,
        coordinates: &Coordinates,
        _: &Settings,
    ) -> crate::Result<WeatherForecast> {
        let mut forecast: Vec<WeatherData> = Vec::new();
        let weather_file = WeatherFile::weather_codes()?;
        let weather_codes: HashMap<String, Vec<String>> = bincode::deserialize(&weather_file.data)?;
        forecast.push(get_current(
            data.weather.clone(),
            data.air_quality.clone(),
            weather_codes.clone(),
        )?);
        let mut futures = data
            .forecast
            .list
            .iter()
            .map(|item| get_future(item.clone(), weather_codes.clone()).unwrap())
            .collect();
        forecast.append(&mut futures);
        let loc = location::reverse_geocode(coordinates)?;
        Ok(WeatherForecast {
            datasource: String::from("Open Weather Map"),
            location: loc,
            forecast: forecast.clone(),
            raw_data: None,
        })
    }
}
