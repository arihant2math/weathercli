pub use backend::get_conditions_sentence;
use backend::WeatherForecast;
use local::location::Coordinates;
use local::settings::Settings;
use log::debug;
use std::{fs, io};
use weather_dirs::custom_backends_dir;

use thiserror::Error;

pub mod dynamic_library_loader;
pub mod loader;
pub mod wasm_loader;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Weather Dirs Error: {0}")]
    WeatherDirsError(#[from] weather_dirs::Error),
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Bincode Error: {0}")]
    BincodeError(Box<shared_deps::bincode::ErrorKind>),
    #[error("Function not found")]
    FunctionNotFound, // TODO: Include name
    #[error("Other Error: {0}")]
    Other(String)
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::Other(s)
    }
}

impl From<Box<shared_deps::bincode::ErrorKind>> for Error {
    fn from(b: Box<shared_deps::bincode::ErrorKind>) -> Self {
        Self::BincodeError(b)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub static CORE_VERSION: &str = "0.0";

pub trait WeatherForecastPlugin {
    fn call(&self, coordinates: &Coordinates, settings: Settings)
        -> crate::Result<WeatherForecast>;

    fn name(&self) -> Option<&str> {
        None
    }
    fn aliases(&self) -> Option<Vec<&str>> {
        None
    }
    /// Help text that may be used to display information about this function.
    fn help(&self) -> Option<&str> {
        None
    }
}

#[derive(Clone)]
pub struct PluginDeclaration {
    pub core_version: &'static str,
    #[allow(improper_ctypes_definitions)] // TODO: Remove this once we have a proper solution
    pub register: unsafe extern "C" fn(&mut dyn PluginRegistrar),
}

pub trait PluginRegistrar {
    fn register_function(&mut self, name: &str, function: Box<dyn WeatherForecastPlugin>);
}

#[macro_export]
macro_rules! export_plugin {
    ($register:expr) => {
        #[no_mangle]
        pub static plugin_declaration: weather_plugin::custom_backend::PluginDeclaration =
            weather_plugin::custom_backend::PluginDeclaration {
                core_version: weather_plugin::custom_backend::CORE_VERSION,
                register: $register,
            };
    };
}

fn is_ext(f: &io::Result<fs::DirEntry>) -> bool {
    match f {
        Err(_e) => false,
        Ok(dir) => {
            if let Ok(metadata) = dir.metadata() {
                if metadata.is_file()
                    && dynamic_library_loader::is_valid_ext(dir.file_name().to_str().unwrap_or(""))
                {
                    return true;
                }
            }
            false
        }
    }
}

pub fn load_custom_backends() -> crate::Result<dynamic_library_loader::ExternalBackends> {
    debug!("Detecting external dlls");
    let path = custom_backends_dir()?;
    let plugins: Vec<String> = path
        .read_dir()
        .map_err(|e| {
            Error::Other(
                "Reading custom backends dir failed: ".to_string() + &e.to_string(),
            )
        })?
        .filter(is_ext) // We only care about files
        .map(|f| f.unwrap().path().display().to_string())
        .collect();
    debug!("Loading: {plugins:?}");
    let custom_backends = dynamic_library_loader::load(plugins);
    Ok(custom_backends)
}

pub fn is_valid_ext(f: &str) -> bool {
    wasm_loader::is_valid_ext(f) || dynamic_library_loader::is_valid_ext(f)
}

pub fn is_valid_file(f: &str) -> bool {
    true // TODO: fix
}
