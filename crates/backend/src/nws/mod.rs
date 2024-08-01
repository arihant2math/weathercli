use local::location;
use local::location::Coordinates;
use local::settings::Settings;
use networking;
use networking::Resp;
use shared_deps::simd_json;
use weather_structs::WeatherForecast;

use crate::nws::current::get_current;
use crate::nws::json::{NWSPointJSON, NWSJSON};
use crate::Backend;

mod current;
mod json;

fn get_api_url(location: &Coordinates, _metric: bool) -> crate::Result<String> {
    let mut get_point = networking::get!(format!(
        "https://api.weather.gov/points/{},{}",
        location.latitude, location.longitude
    ))?
    .text;
    let point_json: NWSPointJSON = unsafe { simd_json::from_str(&mut get_point) }?;
    Ok(point_json.properties.forecast_grid_data)
}

#[derive(Copy, Clone, Debug, Default)]
pub struct NWS;

impl Backend<NWSJSON> for NWS {
    fn get_api_urls(&self, coordinates: &Coordinates, _settings: &Settings) -> Vec<String> {
        vec![get_api_url(coordinates, true).unwrap()]
    }

    fn parse_data(&self, data: Vec<Resp>, _: &Coordinates, _: &Settings) -> crate::Result<NWSJSON> {
        let mut data = data;
        let data: NWSJSON = unsafe { simd_json::from_str(&mut data[0].text) }?;
        Ok(data)
    }

    fn process_data(
        &self,
        data: NWSJSON,
        coordinates: &Coordinates,
        settings: &Settings,
    ) -> crate::Result<WeatherForecast> {
        let current = get_current(data, settings.metric_default)?;
        let loc = location::reverse_geocode(coordinates)?;
        Ok(WeatherForecast {
            datasource: String::from("National Weather Service"),
            location: loc,
            forecast: vec![current], // TODO: Implement future forecasts
            raw_data: None,
        })
    }
}
