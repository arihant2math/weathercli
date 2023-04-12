use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use pyo3::prelude::*;
use sha2::Digest;

use crate::local::cache;

#[cfg(feature = "support")]
pub mod autolaunch;
pub mod backend;
#[cfg(feature = "support")]
pub mod bin_common;
pub mod component_updater;
#[cfg(feature = "python")]
mod layout;
pub mod local;
pub mod location;
pub mod networking;
mod prompt;
#[cfg(feature = "gui")]
mod settings_app;

pub fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect(
        "Time went backwards :( or there is an overflow error of some sort and stuff broke",
    );
    since_the_epoch.as_millis()
}

#[pyfunction]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// returns the sha-256 of the file
#[pyfunction]
pub fn hash_file(filename: &str) -> String {
    let input = Path::new(filename);
    let bytes = fs::read(input).expect("File read failed");
    hex::encode(sha2::Sha256::digest(bytes))
}

#[pyfunction]
#[cfg(feature = "gui")]
pub fn open_settings_app() {
    settings_app::run_settings_app().unwrap();
}

#[pyfunction]
#[cfg(not(feature = "gui"))]
pub fn open_settings_app() {
    println!("GUI support not enabled!");
}

/// corelib module for weather cli, implemented in Rust.
#[pymodule]
fn weather_core(py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(version, module)?)?;
    module.add_function(wrap_pyfunction!(hash_file, module)?)?;
    module.add_function(wrap_pyfunction!(prompt::choice, module)?)?;
    module.add_function(wrap_pyfunction!(open_settings_app, module)?)?;
    module.add_class::<local::weather_file::WeatherFile>()?;
    module.add_class::<local::settings::Settings>()?;
    backend::register_backend_module(py, module)?;
    cache::register_caching_module(py, module)?;
    networking::register_networking_module(py, module)?;
    location::register_location_module(py, module)?;
    component_updater::register_updater_module(py, module)?;
    py.run(
        "\
        import sys\
        ;sys.modules['weather_core.backend'] = backend\
        ;sys.modules['weather_core.caching'] = caching\
        ;sys.modules['weather_core.networking'] = networking\
        ;sys.modules['weather_core.location'] = location\
        ;sys.modules['weather_core.updater'] = updater",
        None,
        Some(module.dict()),
    )?;
    Ok(())
}
