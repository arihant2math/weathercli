use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use pyo3::prelude::*;

#[pyfunction]
pub fn is_update_available() -> String {
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/index.json").expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    let version_server = json.get("version").expect("").to_string();
    return version_server;
}

#[pyfunction]
pub fn is_updater_update_available() -> String {
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/index.json").expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    let version_server = json.get("updater-version").expect("").to_string();
    return version_server;
}

#[pyfunction]
pub fn get_updater(path: String) {
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/updater.exe").expect("url expected").bytes().expect("bytes expected");
    let mut file = OpenOptions::new().create_new(true).write(true).open(path).unwrap();
    file.write_all(&data).expect("Something went wrong opening the file");
}
