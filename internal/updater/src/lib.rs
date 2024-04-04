pub mod component;
pub mod resource;
mod update_server_json;


use shared_deps::simd_json;

use std::collections::HashMap;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LatestVersionError {
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[error("Json error: {0}")]
    JsonError(#[from] simd_json::Error),
    #[error("Version Key not found")]
    VersionKeyNotFound
}

pub fn get_latest_version() -> Result<String, LatestVersionError> {
    let mut data = networking::get!("https://arihant2math.github.io/weathercli/index.json")?;
    unsafe {
        let json: HashMap<String, String> = simd_json::from_str(&mut data.text)?;
        Ok(json
            .get("version")
            .ok_or(LatestVersionError::VersionKeyNotFound)?
            .to_string())
    }
}

pub struct Config<'a> {
    pub weather_file_name: &'a str,
    pub updater_file_name: &'a str,
}

#[cfg(target_os = "windows")]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather.exe",
    updater_file_name: "updater.exe",
};

#[cfg(not(target_os = "windows"))]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather",
    updater_file_name: "updater",
};
