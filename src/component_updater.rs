use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use log::{debug, trace};
use serde_json::Value;

use crate::local::weather_file::WeatherFile;
use crate::util::hash_file;
use crate::{networking, CONFIG};

/// Updates the web resource at $`weathercli_dir/$local_path` if the hash of the local file does not match with
/// the hash at index.json of the index name, if the hashes do not match it download a copy and replaces the existing file
/// :param dev: if true the hashes will be printed if they do not match
fn update_web_resource(
    local_path: String,
    web_resp: Value,
    web_path: &str,
    name: &str,
    out_name: &str,
    quiet: bool,
) -> crate::Result<()> {
    trace!("updating {} ", name);
    let mut f = WeatherFile::new(&local_path)?;
    let file_hash = hash_file(&f.path.display().to_string())?;
    let web_json: Value = web_resp;
    let web_hash: String = web_json[name]
        .as_str()
        .ok_or("Failed to get hash from web")?
        .to_string();
    if web_hash != file_hash {
        debug!("name: {name} web: {web_hash} file: {file_hash}");
        if !quiet {
            if f.exists {
                println!("\x1b[33mDownloading {out_name} update");
            } else {
                println!("\x1b[33mDownloading {out_name}");
            }
        }
        let data = networking::get_url(web_path, None, None, None)?.text;
        f.data = Vec::from(data);
        f.write()?;
    }
    Ok(())
}

/// Updates all the web resources, run on a separate thread as there is no return value
/// :param dev: gets passed `update_web_resource`, if true `update_web_resource` will print the hashes if they don't match
pub fn update_web_resources(quiet: Option<bool>) -> crate::Result<()> {
    debug!("updating web resources");
    let real_quiet = quiet.unwrap_or(false);
    let resp = networking::get_url(
        "https://arihant2math.github.io/weathercli/index.json",
        None,
        None,
        None,
    )?;
    if resp.status == 200 {
        let web_text = resp.text;
        let web_json: Value = serde_json::from_str(&web_text)?;
        update_web_resource(
            String::from("resources/weather_codes.json"),
            web_json.clone(),
            "https://arihant2math.github.io/weathercli/weather_codes.res",
            "weather-codes-hash",
            "weather codes",
            real_quiet,
        )?;
        update_web_resource(
            "resources/weather_ascii_images.json".to_string(),
            web_json.clone(),
            "https://arihant2math.github.io/weathercli/weather_ascii_images.res",
            "weather-ascii-images-hash",
            "ascii images",
            real_quiet,
        )?;
        update_web_resource(
            "layouts/default.json".to_string(),
            web_json,
            "https://arihant2math.github.io/weathercli/default_layout.json",
            "default-layout-hash",
            "default layout",
            real_quiet,
        )?;
        return Ok(());
    }
    Err(crate::error::Error::NetworkError(
        "Status not 200".to_string(),
    ))
}

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
