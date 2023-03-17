use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

use pyo3::prelude::*;
use pyo3::pyfunction;
use serde_json::Value;

use crate::{hash_file, networking};
use crate::local::weather_file::WeatherFile;

/// Updates the web resource at $weathercli_dir/$local_path if the hash of the local file does not match with
/// the hash at index.json of the index name, if the hashes do not match it download a copy and replaces the existing file
/// :param dev: if true the file at ./$local_path will be copied to the web resource location
#[pyfunction]
fn update_web_resource(local_path: String, web_path: String, name: String, dev: bool) {
    if !dev {
        let mut f = WeatherFile::new(local_path);
        let file_hash = hash_file(f.path.display().to_string());
        let web_text =
            reqwest::blocking::get("https://arihant2math.github.io/weathercli/docs/index.json")
                .expect("")
                .text()
                .unwrap();
        let web_json: Value = serde_json::from_str(&web_text).expect("");
        let web_hash: String = web_json[&name].as_str().unwrap().to_string();
        if web_hash != file_hash {
            println!("\x1b[33mDownloading {} update", &name);
            let data = networking::get_url(web_path, None, None, None).text;
            f.data = data;
            f.write();
        }
    } else {
        let mut f = WeatherFile::new(local_path);
        let d = fs::read_to_string(f.path.display().to_string())
            .unwrap()
            .parse()
            .unwrap();
        f.data = d;
        f.write();
    }
}

/// Updates all the web resources, run on a thread as there is no return value
/// :param dev: gets passed update_web_resource, if true update_web_resource will copy files from the working dir instead
#[pyfunction]
fn update_web_resources(dev: bool) {
    update_web_resource(
        "weather_codes.json".to_string(),
        "https://arihant2math.github.io/weathercli/weather_codes.json".to_string(),
        "weather-codes-hash".to_string(),
        dev,
    );
    update_web_resource(
        "weather_ascii_images.json".to_string(),
        "https://arihant2math.github.io/weathercli/weather_ascii_images.json".to_string(),
        "weather-ascii-images-hash".to_string(),
        dev,
    );
}

#[pyfunction]
fn get_latest_version() -> String {
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/docs/index.json")
        .expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    json.get("version").expect("").to_string()
}

#[pyfunction]
fn get_latest_updater_version() -> String {
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/docs/index.json")
        .expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    json.get("updater-version").expect("").to_string()
}

/// Downloads the OS specific updater
#[pyfunction]
fn get_updater(path: String) {
    if cfg!(windows) {
        let data =
            reqwest::blocking::get("https://arihant2math.github.io/weathercli/docs/updater.exe")
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
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/docs/updater")
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
    child_module.add_function(wrap_pyfunction!(update_web_resource, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(update_web_resources, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
