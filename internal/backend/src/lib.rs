pub use weather_structs::weather_condition::WeatherCondition;
pub use weather_structs::weather_data::{get_conditions_sentence, PrecipitationData, WeatherData};
pub use weather_structs::weather_forecast::WeatherForecast;
pub use weather_structs::wind_data::WindData;

pub mod meteo;
pub mod nws;
pub mod openweathermap;
pub mod openweathermap_onecall;
mod openweathermap_shared;

pub type Result<T> = std::result::Result<T, weather_error::Error>;
