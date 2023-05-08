use log::{debug, trace};
use serde_json::Value;

use crate::local::weather_file::WeatherFile;
use crate::util::hash_file;
use networking;
use ansi as color;

struct WebResource {
    local_path: &'static str,
    pretty_name: &'static str,
    hash_name: &'static str,
    url: &'static str,
}

const weather_codes: WebResource = WebResource {
    local_path: "resources/weather_codes.res",
    pretty_name: "weather codes",
    hash_name: "weather-ascii-images-hash",
    url: "weather_codes.res",
};

const weather_ascii_images: WebResource = WebResource {
    local_path: "resources/weather_ascii_images.res",
    pretty_name: "ascii images",
    hash_name: "weather-ascii-images-hash",
    url: "weather_ascii_images.res",
};

const default_layout: WebResource = WebResource {
    local_path: "layouts/default.res",
    pretty_name: "default layout",
    hash_name: "default-layout-hash",
    url: "default_layout.res",
};
/// Updates the web resource at `$weathercli_dir/$local_path` if the hash of the local file does not match with
/// the hash at index.json of the index name, if the hashes do not match it download a copy and replaces the existing file
/// :param dev: if true the hashes will be printed if they do not match
fn update_web_resource(
    resource: WebResource,
    web_resp: Value,
    server: String,
    quiet: bool,
) -> crate::Result<()> {
    let name = resource.hash_name; // TODO: remove stop-gap variables
    let out_name = resource.pretty_name;
    let web_path = server + resource.url;
    trace!("Checking for update for {name} ");
    let mut f = WeatherFile::new(resource.local_path)?;
    let file_hash = hash_file(&f.path.display().to_string())?;
    let web_json: Value = web_resp;
    let web_hash: String = web_json[name]
        .as_str()
        .ok_or("Failed to get hash from web")?
        .to_string();
    if web_hash != file_hash {
        debug!("updating {name} web: {web_hash} file: {file_hash}");
        if !quiet {
            if f.exists {
                println!("{}Downloading update for {out_name}", color::FORE_YELLOW);
            } else {
                println!("{}Downloading {out_name}", color::FORE_YELLOW);
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
pub fn update_web_resources(server: String, quiet: Option<bool>) -> crate::Result<()> {
    debug!("updating web resources");
    let real_quiet = quiet.unwrap_or(false);
    let resp = networking::get_url(format!("{server}index.json"), None, None, None)?;
    unsafe {
        if resp.status == 200 {
            let mut web_text = resp.text;
            let web_json: Value = simd_json::from_str(&mut web_text)?; // Real unsafe here
            update_web_resource(weather_codes, web_json.clone(), server.clone(), real_quiet)?;
            update_web_resource(
                weather_ascii_images,
                web_json.clone(),
                server.clone(),
                real_quiet,
            )?;
            update_web_resource(
                weather_ascii_images,
                web_json.clone(),
                server.clone(),
                real_quiet,
            )?;
            return Ok(());
        }
    }
    Err(crate::error::Error::NetworkError(
        "Status not 200".to_string(),
    ))
}
