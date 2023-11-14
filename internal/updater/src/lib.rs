pub mod component;
pub mod resource;
mod update_server_json;


use shared_deps::simd_json;

use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

pub fn get_latest_version() -> crate::Result<String> {
    let mut data = networking::get!("https://arihant2math.github.io/weathercli/index.json")?;
    unsafe {
        let json: HashMap<String, String> = simd_json::from_str(&mut data.text)?;
        Ok(json
            .get("version")
            .ok_or("getting version key failed")?
            .to_string())
    }
}

pub struct Config<'a> {
    pub weather_file_name: &'a str,
    pub weather_d_file_name: &'a str,
    pub updater_file_name: &'a str,
}

#[cfg(target_os = "windows")]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather.exe",
    weather_d_file_name: "weatherd.exe",
    updater_file_name: "updater.exe",
};

#[cfg(not(target_os = "windows"))]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather",
    weather_d_file_name: "weatherd",
    updater_file_name: "updater",
};
