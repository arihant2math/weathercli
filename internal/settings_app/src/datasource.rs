use std::fmt;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Clone, Copy)]
pub(crate) enum DataSource {
    Meteo,
    OpenWeatherMap,
    OpenWeatherMapOneCall,
    Nws,
}

impl Debug for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DataSource::Meteo => "Meteo".to_string(),
            DataSource::OpenWeatherMap => "OpenWeatherMap".to_string(),
            DataSource::OpenWeatherMapOneCall => "OpenWeatherMap OneCall".to_string(),
            DataSource::Nws => "NWS".to_string(),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for DataSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            DataSource::Meteo => "Meteo".to_string(),
            DataSource::OpenWeatherMap => "OpenWeatherMap".to_string(),
            DataSource::OpenWeatherMapOneCall => "OpenWeatherMap OneCall".to_string(),
            DataSource::Nws => "NWS".to_string(),
        };
        write!(f, "{s}")
    }
}
