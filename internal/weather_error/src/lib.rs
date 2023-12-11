use chrono::ParseError;
use shared_deps::bincode::ErrorKind;
use shared_deps::{serde_json, simd_json, wasmer, windows};
use std::fmt;
use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvocationError {
    CoordinatesError,
    NotFound,
    Other { msg: String },
}

impl fmt::Display for InvocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CoordinatesError => write!(f, "Invalid Coordinates"),
            Self::NotFound => write!(f, "Not found"),
            Self::Other { msg } => write!(f, "{msg}"),
        }
    }
}

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

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    LayoutError(LayoutErr),
    BackendError(String),
    NetworkError(String),
    SerializationError(String),
    IoError(String),
    InvocationError(InvocationError),
    Other(String),
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LayoutError(e) => write!(f, "Layout Error: {e}"),
            Self::BackendError(e) => write!(f, "Backend Error: {e}"),
            Self::NetworkError(e) => write!(f, "Network Error: {e}"),
            Self::SerializationError(e) => write!(f, "Serialization Error: {e}"),
            Self::IoError(e) => write!(f, "I/O Error: {e}"),
            Self::InvocationError(e) => write!(f, "Custom Backend Invocation failed, {e}"),
            Self::Other(s) => write!(f, "{s}"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::IoError(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerializationError(format!("JSON parsing error: {error}"))
    }
}

impl From<simd_json::Error> for Error {
    fn from(error: simd_json::Error) -> Self {
        Self::SerializationError(format!("JSON parsing error: {error}"))
    }
}

impl From<wasmer::CompileError> for Error {
    fn from(error: wasmer::CompileError) -> Self {
        Self::Other(format!("Failed to compile wasm: {error}"))
    }
}

impl From<Box<ErrorKind>> for Error {
    fn from(value: Box<ErrorKind>) -> Self {
        match *value {
            ErrorKind::Io(i) => Self::IoError(i.to_string()),
            ErrorKind::InvalidUtf8Encoding(e) => {
                Self::SerializationError(format!("Bincode Error: Invalid Utf8 Encoding, {e}"))
            }
            ErrorKind::InvalidBoolEncoding(_e) => {
                Self::SerializationError("Bincode Error: Invalid bool encoding".to_string())
            }
            ErrorKind::InvalidCharEncoding => {
                Self::SerializationError("Bincode Error: Invalid char encoding".to_string())
            }
            ErrorKind::InvalidTagEncoding(_u) => {
                Self::SerializationError("Bincode Error: Invalid Tag encoding (enum)".to_string())
            }
            ErrorKind::DeserializeAnyNotSupported => Self::SerializationError(
                "Bincode Error: Attempted to deserialize object with deserialize_any ".to_string(),
            ),
            ErrorKind::SizeLimit => {
                Self::SerializationError("Bincode Error: Size Limit Exceeded".to_string())
            }
            ErrorKind::SequenceMustHaveLength => {
                Self::SerializationError("Bincode Error: Sequence must have length".to_string())
            }
            ErrorKind::Custom(s) => Self::SerializationError(format!("Bincode Error: {s}")),
        }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Self::Other(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<LayoutErr> for Error {
    fn from(error: LayoutErr) -> Self {
        Self::LayoutError(error)
    }
}

impl From<ParseError> for Error {
    fn from(error: ParseError) -> Self {
        Self::SerializationError(format!("DateTime Parse Error: {error}"))
    }
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for Error {
    fn from(error: windows::core::Error) -> Self {
        Self::Other(format!(
            "Win32 Error: {}",
            &error.message().to_string_lossy()
        ))
    }
}
