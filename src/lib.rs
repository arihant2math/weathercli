use std::path::Path;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};
use log::warn;

use crate::backend::meteo::meteo_forecast::get_meteo_forecast;
use crate::backend::nws::nws_forecast::get_nws_forecast;
use crate::backend::openweathermap::openweathermap_forecast::get_openweathermap_forecast;
use crate::backend::weather_forecast::WeatherForecastRS;
use crate::cli::Datasource;
use crate::dynamic_loader::ExternalBackends;
use crate::local::settings::Settings;
#[cfg(feature = "gui")]
use crate::local::settings_app;
use crate::util::Config;

#[cfg(feature = "support")]
pub mod autolaunch;
pub mod backend;
#[cfg(feature = "support")]
pub mod bin_common;
pub mod cli;
pub mod color;
pub mod component_updater;
pub mod custom_backend;
pub mod dynamic_loader;
mod layout;
pub mod local;
pub mod location;
pub mod networking;
pub mod prompt;
pub mod util;

pub type Result<T> = std::result::Result<T, crate::util::Error>;


pub fn now() -> u128 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect(
        "Time went backwards :( or there is an overflow error of some sort and stuff broke",
    );
    since_the_epoch.as_millis()
}

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[cfg(feature = "gui")]
pub fn open_settings_app() {
    settings_app::run_settings_app().expect("App Failed");
}

#[cfg(not(feature = "gui"))]
pub fn open_settings_app() {
    panic!("GUI support not enabled!");
}

#[cfg(target_os = "windows")]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather.exe",
    weather_dfile_name: "weatherd.exe",
    updater_file_name: "updater.exe",
};

#[cfg(not(target_os = "windows"))]
pub const CONFIG: Config<'static> = Config {
    weather_file_name: "weather",
    weather_dfile_name: "weatherd",
    updater_file_name: "updater",
};

pub fn get_data_from_datasource(
    datasource: Datasource,
    coordinates: [String; 2],
    settings: Settings,
    custom_backends: ExternalBackends
) -> crate::Result<WeatherForecastRS> {
    let mut dir = local::dirs::home_dir().expect("Home dir get failed");
    dir.push(".weathercli/resources");
    let mut f1 = dir.clone();
    f1.push("weather_codes.json");
    let mut f2 = dir.clone();
    f2.push("weather_ascii_images.json");
    if !(Path::exists(&dir) && Path::exists(&f1) && Path::exists(&f2)) {
        warn!("Forcing downloading of web resources");
        component_updater::update_web_resources(None)?;
    } else if settings.internal.auto_update_internet_resources {
        thread::spawn(move || {
            component_updater::update_web_resources(None).unwrap_or(());
        });
    }

    return Ok(match datasource {
        Datasource::Openweathermap => get_openweathermap_forecast(Vec::from(coordinates), settings)?,
        Datasource::NWS => get_nws_forecast(Vec::from(coordinates), settings)?,
        Datasource::Meteo => get_meteo_forecast(Vec::from(coordinates), settings)?,
        Datasource::Other(s) => custom_backends.call(s, Vec::from(coordinates), settings)?
    });
}
