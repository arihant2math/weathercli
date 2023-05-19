pub mod component;
pub mod resource;
mod update_server_json;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

pub fn get_latest_version() -> crate::Result<String> {
    let mut data = networking::get_url(
        "https://arihant2math.github.io/weathercli/index.json",
        None,
        None,
        None,
    )?;
    unsafe {
        let json: HashMap<String, String> = simd_json::from_str(&mut data.text)?;
        Ok(json
            .get("version")
            .ok_or("getting version key failed")?
            .to_string())
    }
}

pub fn get_latest_updater_version() -> crate::Result<String> {
    let data = networking::get_url(
        "https://arihant2math.github.io/weathercli/index.json",
        None,
        None,
        None,
    );
    let json: HashMap<String, String> = unsafe { simd_json::from_str(&mut data?.text) }?;
    Ok(json
        .get("updater-version")
        .ok_or("getting updater-version key failed")?
        .to_string())
}

/// Downloads the OS specific updater
pub fn get_updater(path: String) -> crate::Result<()> {
    let url = format!(
        "https://arihant2math.github.io/weathercli/{}",
        CONFIG.updater_file_name
    );
    let data = networking::get_url(url, None, None, None)?.bytes;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(&data)?;
    Ok(())
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
