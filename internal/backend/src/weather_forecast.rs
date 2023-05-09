use crate::WeatherData;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub region: String,
    pub country: String,
    pub forecast: Vec<WeatherData>,
    pub current_weather: WeatherData,
    pub forecast_sentence: String,
    pub raw_data: Option<Vec<String>>,
}
