use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;
use pyo3::prelude::*;

#[pyfunction]
fn get_latest_version() -> String {
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/index.json").expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    let version_server = json.get("version").expect("").to_string();
    return version_server;
}

#[pyfunction]
fn get_latest_updater_version() -> String {
    let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/index.json").expect("");
    let json = data.json::<HashMap<String, String>>().expect("");
    let version_server = json.get("updater-version").expect("").to_string();
    return version_server;
}

#[pyfunction]
fn get_updater(path: String) {
    if cfg!(windows) {
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/updater.exe").expect("url expected").bytes().expect("bytes expected");
        let mut file = OpenOptions::new().create_new(true).write(true).open(path).unwrap();
        file.write_all(&data).expect("Something went wrong opening the file");
    }
    else if cfg!(unix) {
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/updater").expect("url expected").bytes().expect("bytes expected");
        let mut file = OpenOptions::new().create_new(true).write(true).open(path).unwrap();
        file.write_all(&data).expect("Something went wrong opening the file");
    }
    else {
        println!("OS unsupported");
    }
}

pub fn register_update_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "update")?;
    child_module.add_function(wrap_pyfunction!(update::get_latest_version, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(update::get_latest_updater_version, child_module)?)?;
    child_module.add_function(wrap_pyfunction!(update::get_updater, child_module)?)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
