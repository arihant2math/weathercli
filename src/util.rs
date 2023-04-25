use std::fs;
use std::path::Path;
use std::fmt;
use std::fmt::Debug;

use sha2::Digest;

#[derive(Debug, Clone)]
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
impl LayoutErr {
    fn to_string(&self) -> String {
        match self.row {
            Some(row) => match self.item {
                Some(item) => format!("Error at row {}, item {}: {}", row, item, self.message),
                None => format!("Error at row {}: {}", row, self.message),
            },
            None => format!("Error: {}", &self.message),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    LayoutError(LayoutErr),
    NetworkError(String),
    JsonError
}

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::LayoutError(e) => write!(f, "{}", e.to_string()),
            Error::NetworkError(e) => write!(f, "{}", e.to_string()),
            Error::JsonError => write!(f, "JSON error") // TODO: Fix
        }
    }
}

/// returns the sha-256 of the file
pub fn hash_file(filename: &str) -> String {
    let input = Path::new(filename);
    let bytes = fs::read(input).expect("File read failed");
    hex::encode(sha2::Sha256::digest(bytes))
}

pub struct Config<'a> {
    pub weather_file_name: &'a str,
    pub weather_dfile_name: &'a str,
    pub updater_file_name: &'a str,
}
