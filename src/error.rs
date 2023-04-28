use std::fmt;
use std::fmt::Debug;

use crate::custom_backend::InvocationError;

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
    JsonError(String),
    IoError(String),
    InvocationError(InvocationError),
    Other(String)
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LayoutError(e) => write!(f, "Layout Error: {}", e),
            Error::NetworkError(e) => write!(f, "Network Error: {e}"),
            Error::JsonError(e) => write!(f, "JSON Error: {e}"), // TODO: Fix
            Error::IoError(e) => write!(f, "I/O Error: {e}"), // TODO: Fix
            Error::InvocationError(_e) => write!(f, "Custom Backend Invocation failed"), // TODO: Fix
            Error::Other(s) => write!(f, "{s}")
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error::JsonError(format!("JSON parsing error at line {}, column {}", error.line(), error.column()))
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Other(value)
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::Other(value.to_string())
    }
}


impl From<LayoutErr> for Error {
    fn from(error: LayoutErr) -> Self {
        Error::LayoutError(error)
    }
}

#[cfg(target_os = "windows")]
impl From<windows::core::Error> for Error {
    fn from(error: windows::core::Error) -> Self {
        Error::Other(error.message().to_string_lossy())
    }
}
