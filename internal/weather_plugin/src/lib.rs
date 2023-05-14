pub use custom_backend::export_plugin;
pub use custom_backend;
pub use networking;
pub use local::location;
pub use local::now;
pub use local::settings;
pub use local::weather_file;
pub use backend::{WeatherData, WeatherForecast, WeatherCondition, WindData};

pub type Result<T> = std::result::Result<T, weather_error::Error>;
