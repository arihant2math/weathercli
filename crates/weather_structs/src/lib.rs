use serde::{Deserialize, Serialize};

pub use location_data::LocationData;
pub use weather_condition::{get_clouds_condition, WeatherCondition};
pub use weather_data::{get_conditions_sentence, PrecipitationData, WeatherData};
pub use weather_forecast::WeatherForecast;
pub use wind_data::WindData;

mod location_data;
pub mod weather_condition;
pub mod weather_data;
pub mod weather_forecast;
pub mod wind_data;

#[derive(Copy, Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

impl Eq for Coordinates {}

impl std::hash::Hash for Coordinates {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.latitude.to_le_bytes().hash(state);
        self.longitude.to_le_bytes().hash(state);
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WasmPluginInput {
    pub coordinates: Coordinates,
    pub metric: bool,
}
