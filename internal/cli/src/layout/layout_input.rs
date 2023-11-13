use serde::{Deserialize, Serialize};
use backend::WeatherData;
use local::location::LocationData;

#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutInput {
    pub datasource: String,
    pub location: LocationData,
    pub current_weather: WeatherData, // For legacy access purposes :(
    pub forecast_sentence: String,
}