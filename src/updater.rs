use pyo3::prelude::*;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Write;

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

#[pyfunction]
fn get_updater(path: String) {
    if cfg!(windows) {
        let data = reqwest::blocking::get("https://arihant2math.github.io/weathercli/updater.exe")
            .expect("url expected")
            .bytes()
            .expect("bytes expected");
        let mut file = OpenOptions::new()
            .create_new(true)
            .write(true)
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
            .create_new(true)
            .write(true)
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
    parent_module.add_submodule(child_module)?;
    Ok(())
}
