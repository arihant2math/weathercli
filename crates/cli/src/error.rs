use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[error("JSON Error: {0}")]
    JSONError(#[from] shared_deps::simd_json::Error),
    #[error("Serde JSON Error: {0}")]
    SerdeJSONError(#[from] shared_deps::serde_json::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Weather dirs Error: {0}")]
    WeatherDirsError(#[from] weather_dirs::Error),
    #[error("Weather file Error: {0}")]
    WeatherFileError(#[from] local::weather_file::Error),
    #[error("Settings Error: {0}")]
    SettingsError(#[from] local::settings::Error),
    #[error("Backend Error: {0}")]
    BackendError(#[from] backend::Error),
    #[error("Layout Error: {0}")]
    LayoutError(#[from] layout::Error),
    #[error("Custom Backend Error: {0}")]
    CustomBackendError(#[from] custom_backend::Error),
    #[error("Coordinate Error: {0}")]
    CoordinateError(#[from] local::location::CoordinateError),
    #[error("Geocode Error: {0}")]
    GeocodeError(#[from] local::location::GeocodeError),
    #[error("Update Error: {0}")]
    UpdaterError1(#[from] updater::component::UpdateError),
    #[error("Update Error: {0}")]
    UpdaterError2(#[from] updater::resource::UpdateError),
    #[error("Update Error: {0}")]
    UpdaterError3(#[from] updater::LatestVersionError),
    #[error("Other: {0}")]
    Other(String),
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::Other(s.to_string())
    }
}
