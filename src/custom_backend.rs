use crate::backend::weather_forecast::WeatherForecast;
use crate::local::settings::Settings;

pub static RUSTC_VERSION: &str = env!("RUSTC_VERSION");
pub static CORE_VERSION: &str = "0.1";

pub trait WeatherForecastPlugin {
    fn call(&self, coordinates: [&str; 2], settings: Settings) -> crate::Result<WeatherForecast>;

    fn name(&self) -> Option<&str> {
        None
    }

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
    fn from(other: S) -> Self {
        Self::Other {
            msg: other.to_string(),
        }
    }
}

#[derive(Clone)]
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
        pub static plugin_declaration: weather_core::custom_backend::PluginDeclaration =
            weather_core::custom_backend::PluginDeclaration {
                rustc_version: weather_core::custom_backend::RUSTC_VERSION,
                core_version: weather_core::custom_backend::CORE_VERSION,
                register: $register,
            };
    };
}
