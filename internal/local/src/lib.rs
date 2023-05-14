pub mod cache;
pub mod location;
pub mod settings;
pub mod weather_file;

use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub type Result<T> = std::result::Result<T, weather_error::Error>;

pub fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect(
        "Time went backwards :( or there is an overflow error of some sort and stuff broke",
    );
    since_the_epoch.as_millis()
}

pub fn list_dir(dir: PathBuf) -> crate::Result<Vec<String>> {
    Ok(fs::read_dir(dir)?
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect())
}
