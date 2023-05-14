use crate::nws::json::{NWSPointJSON, NWSJSON};
use local::location::Coordinates;
use networking;

mod current;
pub mod forecast;
mod json;

fn get_api_url(location: Coordinates, _metric: bool) -> crate::Result<String> {
    let mut get_point = networking::get_url(
        format!(
            "https://api.weather.gov/points/{},{}",
            location.latitude, location.longitude
        ),
        None,
        None,
        None,
    )?
    .text;
    let point_json: NWSPointJSON = unsafe { simd_json::from_str(&mut get_point) }?;
    Ok(point_json.properties.forecast_grid_data)
}

pub fn get_combined_data_formatted(location: Coordinates, metric: bool) -> crate::Result<NWSJSON> {
    let mut raw_data = networking::get_url(get_api_url(location, metric)?, None, None, None)?;
    let data: NWSJSON = unsafe { simd_json::from_str(&mut raw_data.text) }?;
    Ok(data)
}
