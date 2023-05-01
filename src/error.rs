use std::fmt;
use std::fmt::Debug;
use bincode::ErrorKind;

use crate::backend::custom_backend::InvocationError;

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
            Self::NetworkError(e) => write!(f, "Network Error: {e}"),
            Self::SerializationError(e) => write!(f, "Serialization Error: {e}"), // TODO: Fix
            Self::IoError(e) => write!(f, "I/O Error: {e}"),    // TODO: Fix
            Self::InvocationError(_e) => write!(f, "Custom Backend Invocation failed"), // TODO: Fix
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
        Self::SerializationError(format!(
            "JSON parsing error at line {}, column {}",
            error.line(),
            error.column()
        ))
    }
}

impl From<Box<bincode::ErrorKind>> for Error {
    fn from(value: Box<ErrorKind>) -> Self {
        Self::SerializationError(format!("Bincode error")) // TODO: Use value
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

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for Error {
    fn from(error: windows::core::Error) -> Self {
        Self::Other(error.message().to_string_lossy())
    }
}
