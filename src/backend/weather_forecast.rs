use serde::{Deserialize, Serialize};

use crate::backend::Status;
use crate::backend::weather_data::WeatherDataRS;
use crate::location;

pub fn get_location(loc: Vec<String>) -> crate::Result<[String; 2]> {
    location::reverse_location(&loc[0], &loc[1])
}

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherForecastRS {
    pub status: Status,
    pub region: String,
    pub country: String,
    pub forecast: Vec<WeatherDataRS>,
    pub current_weather: WeatherDataRS,
    pub forecast_sentence: String,
    pub raw_data: Option<Vec<String>>,
}
