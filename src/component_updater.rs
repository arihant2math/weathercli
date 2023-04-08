use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

use pyo3::prelude::*;
use pyo3::pyfunction;
use serde_json::Value;

use crate::hash_file;
use crate::local::weather_file::WeatherFile;

/// Updates the web resource at $weathercli_dir/$local_path if the hash of the local file does not match with
/// the hash at index.json of the index name, if the hashes do not match it download a copy and replaces the existing file
/// :param dev: if true the hashes will be printed if they do not match
fn update_web_resource(
    local_path: String,
    web_resp: Value,
    web_path: &str,
    name: &str,
    out_name: &str,
    dev: bool,
    quiet: bool,
) {
    let mut f = WeatherFile::new(&local_path);
    let file_hash = hash_file(&f.path.display().to_string());
    let web_json: Value = web_resp;
    let web_hash: String = web_json[name].as_str().unwrap().to_string();
    if web_hash != file_hash {
        if dev {
            println!("web: {} file: {}", web_hash, file_hash)
        }
        if !quiet {
            if f.exists {
                println!("\x1b[33mDownloading {} update", out_name);
            } else {
                println!("\x1b[33mDownloading {}", out_name);
            }
        }
        let data = reqwest::blocking::get(web_path).unwrap().text().unwrap();
        f.data = data;
        f.write();
    }
}

/// Updates all the web resources, run on a separate thread as there is no return value
/// :param dev: gets passed update_web_resource, if true update_web_resource will print the hashes if they don't match
#[pyfunction]
pub fn update_web_resources(dev: bool, quiet: Option<bool>) {
    let real_quiet = quiet.unwrap_or(false);
    let resp =
        reqwest::blocking::get("https://arihant2math.github.io/weathercli/index.json").expect("");
    if resp.status().as_u16() == 200 {
        let web_text = resp.text().unwrap();
        let web_json: Value = serde_json::from_str(&web_text).expect("");
        update_web_resource(
            String::from("resources/weather_codes.json"),
            web_json.clone(),
            "https://arihant2math.github.io/weathercli/weather_codes.json",
            "weather-codes-hash",
            "weather codes",
            dev,
            real_quiet,
        );
        update_web_resource(
            "resources/weather_ascii_images.json".to_string(),
            web_json,
            "https://arihant2math.github.io/weathercli/weather_ascii_images.json",
            "weather-ascii-images-hash",
            "ascii images",
            dev,
            real_quiet,
        );
    }
}
#[pyfunction]
fn get_latest_version() -> String {
    let data =
        reqwest::blocking::get("https://arihant2math.github.io/weathercli/index.json").expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    json.get("version").expect("").to_string()
}

#[pyfunction]
fn get_latest_updater_version() -> String {
    let data =
        reqwest::blocking::get("https://arihant2math.github.io/weathercli/index.json").expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    json.get("updater-version").expect("").to_string()
}

/// Downloads the OS specific updater
#[pyfunction]
fn get_updater(path: String) {
    if cfg!(windows) {
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/updater.exe")
            .expect("url expected")
            .bytes()
            .expect("bytes expected");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        file.write_all(&data)
            .expect("Something went wrong opening the file");
    } else if cfg!(unix) {
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/updater")
            .expect("url expected")
            .bytes()
            .expect("bytes expected");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        file.write_all(&data)
            .expect("Something went wrong opening the file");
    } else {
        println!("OS unsupported");
    }
}

pub fn register_updater_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "updater")?;
    child_module.add_function(wrap_pyfunction!(get_latest_version, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(get_latest_updater_version, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(get_updater, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(update_web_resources, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
