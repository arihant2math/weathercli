pub mod settings;
pub mod weather_file;
pub mod cache;

use std::fs;
use std::path::PathBuf;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

pub fn list_dir(dir: PathBuf) -> crate::Result<Vec<String>> {
    Ok(fs::read_dir(dir)?
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect())
}
