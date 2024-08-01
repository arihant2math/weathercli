use std::fmt;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct LayoutErr {
    pub message: String,
    pub row: Option<u64>,
    pub item: Option<u64>,
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for LayoutErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.row {
            Some(row) => match self.item {
                Some(item) => write!(f, "Error at row {}, item {}: {}", row, item, self.message),
                None => write!(f, "Error at row {}: {}", row, self.message),
            },
            None => write!(f, "Error: {}", &self.message),
        }
    }
}

impl std::error::Error for LayoutErr {}

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
    #[error("Layout Error: {0}")]
    LayoutError(#[from] LayoutErr),
    #[error("Weather dris Error: {0}")]
    WeatherDirsError(#[from] weather_dirs::Error),
    #[error("Weather file Error: {0}")]
    WeatherFileError(#[from] local::weather_file::Error),
    #[error("Chrono Parse Error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),
    #[error("Bincode Error: {0}")]
    BincodeError(Box<shared_deps::bincode::ErrorKind>),
    #[error("Other Error: {0}")]
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

impl From<Box<shared_deps::bincode::ErrorKind>> for Error {
    fn from(b: Box<shared_deps::bincode::ErrorKind>) -> Self {
        Self::BincodeError(b)
    }
}
