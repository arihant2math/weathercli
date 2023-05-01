pub mod resource_updater;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use log::{debug, trace};
use serde_json::Value;

use crate::local::weather_file::WeatherFile;
use crate::util::hash_file;
use crate::{networking, CONFIG};

pub fn get_latest_version() -> crate::Result<String> {
    let data = networking::get_url(
        "https://arihant2math.github.io/weathercli/index.json",
        None,
        None,
        None,
    );
    let json: HashMap<String, String> = serde_json::from_str(&data?.text)?;
    Ok(json
        .get("version")
        .ok_or("getting version key failed")?
        .to_string())
}

pub fn get_latest_updater_version() -> crate::Result<String> {
    let data = networking::get_url(
        "https://arihant2math.github.io/weathercli/index.json",
        None,
        None,
        None,
    );
    let json: HashMap<String, String> = serde_json::from_str(&data?.text)?;
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
