use serde::{Deserialize, Serialize};
use crate::backend::weather_data::WeatherDataRS;

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherForecastRS {
    pub region: String,
    pub country: String,
    pub forecast: Vec<WeatherDataRS>,
    pub current_weather: WeatherDataRS,
    pub forecast_sentence: String,
    pub raw_data: Option<Vec<String>>,
}
