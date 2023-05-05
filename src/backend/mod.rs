use serde::{Deserialize, Serialize};

pub mod meteo;
pub mod nws;
pub mod openweathermap;
pub mod openweathermap_onecall;
mod openweathermap_shared;
pub mod weather_condition;
pub mod weather_data;
pub mod weather_forecast;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct WindData {
    pub speed: f64,
    pub heading: u16,
}
