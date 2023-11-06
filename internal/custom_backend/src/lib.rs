pub mod dynamic_library_loader;
pub mod loader;
pub mod wasm_loader;

use std::{fs, io};

use backend::WeatherForecast;
use local::settings::Settings;

use weather_dirs::custom_backends_dir;

use log::{debug, info};
use local::location::Coordinates;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

pub static CORE_VERSION: &str = "0.0";

pub trait WeatherForecastPlugin {
    fn call(&self, coordinates: &Coordinates, settings: Settings) -> crate::Result<WeatherForecast>;

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
        .read_dir().map_err(|e| weather_error::Error::IoError("Reading custom backends dir failed: ".to_string() + &e.to_string()))?
        .filter(is_ext) // We only care about files
        .map(|f| f.unwrap().path().display().to_string())
        .collect();
    debug!("Loading: {plugins:?}");
    let custom_backends = dynamic_library_loader::load(plugins);
    Ok(custom_backends)
}
