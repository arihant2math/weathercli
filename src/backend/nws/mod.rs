use crate::backend::nws::nws_json::{NWSJSON, NWSPointJSON};
use crate::networking;

mod nws_current;
pub mod nws_forecast;
mod nws_json;

pub fn nws_get_api_urls(location: [&str; 2], _metric: bool) -> crate::Result<String> {
    let get_point = networking::get_url(
        "https://api.weather.gov/points/".to_string() + &location[0] + "," + &location[1],
        None,
        None,
        None,
    )?
    .text;
    let point_json: NWSPointJSON =
        serde_json::from_str(&get_point).expect("Deserialization of json failed");
    Ok(point_json.properties.forecast_grid_data)
}

pub fn nws_get_combined_data_formatted(location: [&str; 2], metric: bool) -> crate::Result<NWSJSON> {
    let raw_data = networking::get_url(nws_get_api_urls(location, metric)?, None, None, None)?;
    let data: NWSJSON =
        serde_json::from_str(&raw_data.text).expect("Deserialization of json failed");
    Ok(data)
}
