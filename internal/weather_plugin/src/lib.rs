pub use backend;
pub use backend::{WeatherCondition, WeatherData, WeatherForecast, WindData};
pub use chrono;
pub use custom_backend;
pub use custom_backend::export_plugin;
pub use local::location;
pub use local::settings;
pub use local::weather_file;
pub use networking;

pub type Result<T> = std::result::Result<T, weather_error::Error>;
