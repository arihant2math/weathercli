pub use location_data::LocationData;
use serde::{Deserialize, Serialize};
pub use weather_condition::{get_clouds_condition, WeatherCondition};
pub use weather_data::{get_conditions_sentence, PrecipitationData, WeatherData};
pub use weather_forecast::WeatherForecast;
pub use wind_data::WindData;

pub mod weather_condition;
pub mod weather_data;
pub mod weather_forecast;
pub mod wind_data;
mod location_data;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WasmPluginInput {
    pub coordinates: Coordinates,
    pub metric: bool
}
