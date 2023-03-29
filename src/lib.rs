use pyo3::prelude::*;
use sha256::try_digest;
use std::path::Path;

use crate::local::cache;

pub mod backend;
pub mod bin_common;
pub mod component_updater;
mod layout;
pub mod local;
pub mod location;
pub mod networking;
mod prompt;
mod settings_app;

/// returns the sha-256 of the file
#[pyfunction]
pub fn hash_file(filename: &str) -> String {
    let input = Path::new(filename);
    try_digest(input).unwrap()
}

#[pyfunction]
pub fn open_settings_app() {
    settings_app::run_settings_app().unwrap();
}

/// corelib module for weather cli, implemented in Rust.
#[pymodule]
fn weather_core(py: Python, module: &PyModule) -> PyResult<()> {
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
