use std::collections::HashMap;

use log::warn;
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

use crate::meteo::json::{MeteoAirQualityJson, MeteoForecastJson};
use crate::meteo::weather_data::get_weather_data;
use crate::Backend;

mod json;
mod weather_data;

#[derive(Clone, Serialize, Deserialize)]
pub struct MeteoFormattedData {
    pub weather: MeteoForecastJson,
    pub air_quality: MeteoAirQualityJson,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct Meteo;

impl Backend<MeteoFormattedData> for Meteo {
    fn get_api_urls(&self, coordinates: &Coordinates, settings: &Settings) -> Vec<String> {
        let latitude = coordinates.latitude;
        let longitude = coordinates.longitude;
        let base_forecast_url = "https://api.open-meteo.com/v1/forecast";
        let base_air_quaility_url = "https://air-quality-api.open-meteo.com/v1/air-quality";
        let hourly = "temperature_2m,rain,showers,snowfall,cloudcover,dewpoint_2m,apparent_temperature,pressure_msl,visibility,windspeed_10m,winddirection_10m,precipitation_probability";
        let daily = "temperature_2m_max,temperature_2m_min";
        let units = if settings.metric_default {
            ""
        } else {
            "&temperature_unit=fahrenheit&windspeed_unit=mph&precipitation_unit=inch"
        };
        Vec::from([format!("{base_forecast_url}?latitude={latitude}&longitude={longitude}&current_weather=true&hourly={hourly}&daily={daily}&timezone=auto{units}"),
            format!("{base_air_quaility_url}?latitude={latitude}&longitude={longitude}&hourly=european_aqi")])
    }

    fn parse_data(
        &self,
        data: Vec<Resp>,
        _: &Coordinates,
        _: &Settings,
    ) -> crate::Result<MeteoFormattedData> {
        let mut data = data;
        unsafe {
            let r1: MeteoForecastJson = simd_json::from_str(&mut data[0].text)?;
            let r2: MeteoAirQualityJson = simd_json::from_str(&mut data[1].text)?;
            Ok(MeteoFormattedData {
                weather: r1,
                air_quality: r2,
            })
        }
    }

    fn process_data(
        &self,
        data: MeteoFormattedData,
        coordinates: &Coordinates,
        settings: &Settings,
    ) -> crate::Result<WeatherForecast> {
        let mut forecast: Vec<WeatherData> = Vec::new();
        let now_option = data
            .weather
            .hourly
            .time
            .iter()
            .position(|r| *r == data.weather.current_weather.time);

        let now = match now_option {
            Some(n) => n,
            None => {
                warn!("Could not find current time in weather data");
                0
            }
        };
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
        let loc = location::reverse_geocode(coordinates)?;
        let f = WeatherForecast {
            datasource: String::from("meteo"),
            location: loc,
            forecast: forecast.clone(),
            raw_data: None,
        };
        Ok(f)
    }
}
