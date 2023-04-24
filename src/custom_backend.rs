use crate::backend::weather_forecast::WeatherForecastRS;
use crate::local::settings::Settings;

pub static CORE_VERSION: &str = "0";
pub static RUSTC_VERSION: &str = "TODO";

pub trait WeatherForecastPlugin {
    fn call(
        &self,
        coordinates: Vec<String>,
        settings: Settings,
    ) -> Result<WeatherForecastRS, InvocationError>;

    /// Help text that may be used to display information about this function.
    fn help(&self) -> Option<&str> {
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InvocationError {
    CoordinatesError,
    NotFound,
    Other { msg: String },
}

impl<S: ToString> From<S> for InvocationError {
    fn from(other: S) -> InvocationError {
        InvocationError::Other {
            msg: other.to_string(),
        }
    }
}

#[derive(Copy, Clone)]
pub struct PluginDeclaration {
    pub rustc_version: &'static str,
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
        pub static plugin_declaration: weather_core::custom_backend::PluginDeclaration = weather_core::custom_backend::PluginDeclaration {
            rustc_version: weather_core::custom_backend::RUSTC_VERSION,
            core_version: weather_core::custom_backend::CORE_VERSION,
            register: $register,
        };
    };
}
