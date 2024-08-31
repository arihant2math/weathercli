use log::{debug, trace};
use thiserror::Error;

use local::hash_file;
use local::weather_file::WeatherFile;
use networking;
use serde_json::Value;
use simd_json;
use terminal::color;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Network error: {0}")]
    NetworkError(#[from] networking::Error),
    #[error("Reqwuest error: {0}")]
    ReqwuestError(String), // TODO: Store actual error
    #[error("JSON Error: {0}")]
    JSONError(#[from] simd_json::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Weather file Error: {0}")]
    WeatherFileError(#[from] local::weather_file::Error),
    #[error("Server Error: {0}")]
    ServerError(String),
}

struct WebResource {
    local_path: &'static str,
    pretty_name: &'static str,
    hash_name: &'static str,
    url: &'static str,
}

const WEATHER_CODES: WebResource = WebResource {
    local_path: "resources/weather_codes.res",
    pretty_name: "weather codes",
    hash_name: "weather-ascii-images-hash",
    url: "weather_codes.res",
};

const WEATHER_ASCII_IMAGES: WebResource = WebResource {
    local_path: "resources/weather_ascii_images.res",
    pretty_name: "ascii images",
    hash_name: "weather-ascii-images-hash",
    url: "weather_ascii_images.res",
};

const DEFAULT_LAYOUT: WebResource = WebResource {
    local_path: "layouts/default.res",
    pretty_name: "default layout",
    hash_name: "default-layout-hash",
    url: "default_layout.res",
};
/// Updates the web resource at `$weathercli_dir/$local_path` if the hash of the local file does not match with
/// the hash at index.json of the index name, if the hashes do not match it download a copy and replaces the existing file
/// # Arguments
/// * `dev` if true the hashes will be printed if they do not match
fn update_web_resource(
    resource: &WebResource,
    web_resp: Value,
    server: &str,
    quiet: bool,
) -> Result<(), UpdateError> {
    let web_path = format!("{server}{}", resource.url);
    trace!("Checking for update for {} ", resource.hash_name);
    let mut f = WeatherFile::new(resource.local_path)?;
    let file_hash = hash_file(&f.path.display().to_string())?;
    let web_json: Value = web_resp;
    let web_hash: String = web_json[resource.hash_name]
        .as_str()
        .ok_or(UpdateError::ServerError(
            "Failed to get hash from web".to_string(),
        ))?
        .to_string();
    if web_hash != file_hash {
        debug!(
            "Updating {} web: {web_hash} file: {file_hash}",
            resource.hash_name
        );
        if !quiet {
            if f.exists {
                println!(
                    "{}Downloading update for {}",
                    color::FORE_YELLOW,
                    resource.pretty_name
                );
            } else {
                println!("{}Downloading {}", color::FORE_YELLOW, resource.pretty_name);
            }
        }
        let data = reqwest::blocking::get(web_path)
            .map_err(|_| UpdateError::ReqwuestError("Failed to download file".to_string()))?
            .text()
            .map_err(|_| UpdateError::ReqwuestError("Failed to download file".to_string()))?;
        f.data = Vec::from(data);
        f.write()?;
    }
    Ok(())
}

/// Updates all the web resources, run on a separate thread as there is no return value
/// # Arguments
/// * `dev` gets passed `update_web_resource`, if true `update_web_resource` will print the hashes if they don't match
pub fn update_web_resources(server: &str, quiet: Option<bool>) -> Result<(), UpdateError> {
    debug!("Updating web resources");
    let real_quiet = quiet.unwrap_or(false); // TODO: Fix naming (don't call it real_)
    let formatted_server = if server.ends_with('/') {
        server.to_string()
    } else {
        server.to_string() + "/"
    };
    let resp = networking::get!(format!("{formatted_server}index.json"))?;
    unsafe {
        if resp.status == 200 {
            let mut web_text = resp.text;
            let web_json: Value = simd_json::from_str(&mut web_text)?; // Real unsafe stuff here
            update_web_resource(&WEATHER_CODES, web_json.clone(), server, real_quiet)?;
            update_web_resource(&WEATHER_ASCII_IMAGES, web_json.clone(), server, real_quiet)?;
            update_web_resource(&DEFAULT_LAYOUT, web_json, server, real_quiet)?;
            return Ok(());
        }
    }
    Err(UpdateError::ServerError("Status not 200".to_string()))
}
