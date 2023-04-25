pub mod meteo;
pub mod nws;
pub mod openweathermap;
pub mod weather_condition;
pub mod weather_data;
pub mod weather_forecast;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub enum Status {
    OK = 0,
    ServerError = 1,
    InvalidApiKey = 2,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindData {
    pub speed: f64,
    pub heading: i16,
}

