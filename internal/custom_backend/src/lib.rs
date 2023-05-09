pub mod dynamic_library_loader;
pub mod loader;
pub mod wasm_loader;

use backend::WeatherForecast;
use local::settings::Settings;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

pub static CORE_VERSION: &str = "0.0";

pub trait WeatherForecastPlugin {
    fn call(&self, coordinates: [&str; 2], settings: Settings) -> crate::Result<WeatherForecast>;

    fn name(&self) -> Option<&str> {
        None
    }
    // TODO: Use all this data
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
        pub static plugin_declaration: weather_core::backend::custom_backend::PluginDeclaration =
            custom_backend::PluginDeclaration {
                core_version: weather_core::backend::custom_backend::CORE_VERSION,
                register: $register,
            };
    };
}
