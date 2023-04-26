pub mod meteo;
pub mod nws;
pub mod openweathermap;
pub mod weather_condition;
pub mod weather_data;
pub mod weather_forecast;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindData {
    pub speed: f64,
    pub heading: i16,
}

