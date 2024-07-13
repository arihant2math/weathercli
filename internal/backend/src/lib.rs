use log::{error, trace};
use thiserror::Error;

pub use datasource::Backend;
use local::settings::Settings;
use weather_structs::Coordinates;
pub use weather_structs::weather_condition::WeatherCondition;
pub use weather_structs::weather_data::{get_conditions_sentence, PrecipitationData, WeatherData};
pub use weather_structs::weather_forecast::WeatherForecast;
pub use weather_structs::wind_data::WindData;

pub mod meteo;
pub mod nws;
pub mod openweathermap;
pub mod openweathermap_onecall;
pub mod openweathermap_shared;

pub mod datasource;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[error("JSON Error: {0}")]
    JSONError(#[from] shared_deps::simd_json::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Failed to retrieve weather condition details: {0}")]
    WeatherConditionError(#[from] weather_structs::weather_condition::WeatherConditionError),
    #[error("Reverse Geocode Error: {0}")]
    ReverseGeocodeError(#[from] local::location::ReverseGeocodeError),
    #[error("Weather file Error: {0}")]
    WeatherFileError(#[from] local::weather_file::Error),
    #[error("Chrono Parse Error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),
    #[error("Bincode Error: {0}")]
    BincodeError(Box<shared_deps::bincode::ErrorKind>),
    #[error("Other Backend Error: {0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

impl From<Box<shared_deps::bincode::ErrorKind>> for Error {
    fn from(b: Box<shared_deps::bincode::ErrorKind>) -> Self {
        Self::BincodeError(b)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn run<T>(backend: Box<&dyn Backend<T>>, coordinates: &Coordinates, settings: &Settings) -> Result<WeatherForecast> {
    let urls = backend.get_api_urls(coordinates, &settings);
    let raw_data = networking::gets!(&urls);
    match raw_data {
        Ok(raw_data) => {
            let data = backend.parse_data(raw_data.clone(), coordinates, &settings);
            match data {
                Ok(data) => backend.process_data(data, coordinates, &settings),
                Err(e) => {
                    error!("Failed to parse data: {:?}", e);
                    trace!("Writing raw data to logs");
                    for d in raw_data {
                        trace!("{}", d.text);
                    }
                    Err(e)
                }
            }
        }
        Err(e) => {
            error!("Failed to get data from backend due to network error");
            Err(Error::NetworkError(e))
        }
    }
}
