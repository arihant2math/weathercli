use serde::{Deserialize, Serialize};
use backend::WeatherData;
use local::location::LocationData;

#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutInput {
    pub datasource: String,
    pub location: LocationData,
    pub weather: WeatherData,
    pub forecast_sentence: String,
}