use serde::{Deserialize, Serialize};

pub mod meteo;
pub mod nws;
pub mod openweathermap;
pub mod openweathermap_onecall;
mod openweathermap_shared;
mod weather_condition;
mod weather_data;
mod weather_forecast;
pub use weather_condition::WeatherCondition;
pub use weather_data::{get_conditions_sentence, WeatherData};
pub use weather_forecast::WeatherForecast;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

#[derive(Clone, Copy, Serialize, Deserialize, Debug, PartialEq)]
pub struct WindData {
    pub speed: f64,
    pub heading: u16,
}
