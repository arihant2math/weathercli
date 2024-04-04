use backend::{meteo, nws, openweathermap, openweathermap_onecall, WeatherForecast};
use chrono::{DateTime, Duration, Utc};
use custom_backend::dynamic_library_loader::ExternalBackends;
use custom_backend::wasm_loader::WasmLoader;
use layout::layout_input::LayoutInput;
use local::cache::prune;
use local::location::Coordinates;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use log::{debug, error, warn};
use parse_duration::parse as parse_duration;
use shared_deps::serde_json::Value;
use shared_deps::simd_json;
use std::path::Path;
use std::str::FromStr;
use std::thread;
use terminal::color::*;
use terminal::prompt;
use weather_dirs::resources_dir;

use crate::{Datasource, print_out};
use crate::arguments::CacheOpts;

pub mod backend_commands;
pub mod layout_commands;
pub mod util;
pub mod saved_commands;

fn get_requested_time(time: Option<String>) -> DateTime<Utc> {
    match time {
        Some(t) => {
            let time = chrono::Utc::now() + chrono::Duration::from_std(parse_duration(&t).unwrap_or(std::time::Duration::new(0, 0))).unwrap_or(Duration::zero());
            return time;
        }
        None => {
            return chrono::Utc::now();
        }
    }
}

pub fn get_data_from_datasource(
    datasource: Datasource,
    coordinates: Coordinates,
    settings: Settings,
    custom_backends: ExternalBackends,
    wasm_loader: &mut WasmLoader
) -> crate::Result<WeatherForecast> {
    let dir = resources_dir()?;
    let f1 = dir.join("weather_codes.res");
    let f2 = dir.join("weather_ascii_images.res");
    let update_server = settings.update_server.clone();
    if !(Path::exists(&dir) && Path::exists(&f1) && Path::exists(&f2)) {
        warn!("Forcing downloading of web resources");
        updater::resource::update_web_resources(&update_server, None)?;
    } else if settings.auto_update_internet_resources {
        thread::spawn(move || {
            updater::resource::update_web_resources(&update_server, None).unwrap_or(());
        });
    }
    debug!("Getting data from datasource: {datasource:?}");
    match datasource {
        Datasource::Openweathermap => {
            Ok(openweathermap::forecast::get_forecast(&coordinates, settings)?)
        }
        Datasource::OpenweathermapOneCall => {
            Ok(openweathermap_onecall::forecast::get_forecast(&coordinates, settings)?)
        }
        Datasource::NWS => Ok(nws::forecast::get_forecast(&coordinates, settings)?),
        Datasource::Meteo => Ok(meteo::forecast::get_forecast(&coordinates, settings)?),
        Datasource::Other(s) => {
            if settings.enable_wasm_backends {
                Ok(wasm_loader.call(&s, coordinates, settings)?)
            }
            else if settings.enable_custom_backends {
                Ok(custom_backends.call(&s, &coordinates, settings)?)
            } else {
                return Err(backend::Error::Other(
                    "Custom backends are disabled. Enable them in the settings.".to_string(), // TODO: more help (specifically which commands to run)
                ))?;
            }
        }
    }
}

pub fn weather(
    datasource: Datasource,
    coordinates: Coordinates,
    time: Option<String>,
    settings: Settings,
    true_metric: bool,
    json: bool,
    custom_backends: ExternalBackends,
    wasm_backends: &mut WasmLoader
) -> crate::Result<()> {
    debug!(
        "Coordinates: {} {}",
        coordinates.latitude, coordinates.longitude
    );
    debug!("Metric: {true_metric}");
    debug!("json: {json}");
    let mut s = settings.clone();
    s.metric_default = true_metric;
    let data = get_data_from_datasource(datasource, coordinates, s, custom_backends, wasm_backends).map_err(|e| {
        error!("Error getting data from datasource: {e}");
        e
    })?;
    print_out(&settings.layout_file, LayoutInput::from_forecast(data, get_requested_time(time))?, json, true_metric)?;
    Ok(())
}

pub fn config(key_name: String, value: Option<String>) -> crate::Result<()> {
    match value {
        None => {
            let f = WeatherFile::settings()?;
            unsafe {
                let data: Value = simd_json::from_str(&mut f.get_text()?)?;
                println!("{}: {}", &key_name, data[&key_name]);
            }
        }
        Some(real_value) => {
            println!("Writing {}={} ...", key_name.to_lowercase(), &real_value);
            let mut f = WeatherFile::settings()?;
            unsafe {
                let mut data: Value = simd_json::from_str(&mut f.get_text()?)?;
                data[key_name.to_uppercase()] = Value::from_str(&real_value)?;
                f.data = Vec::from(simd_json::to_string(&data)?);
                f.write()?;
            }
        }
    };
    Ok(())
}

pub fn cache(opts: CacheOpts) -> crate::Result<()> {
    match opts {
        CacheOpts::Clear => {
            let mut f = WeatherFile::new("d.cache")?;
            f.data = Vec::new();
            f.write()?;
            let mut f = WeatherFile::new("f.cache")?;
            f.data = Vec::new();
            f.write()?;
        }
        CacheOpts::Prune => prune()?,
    }
    Ok(())
}

pub fn about() {
    println!("Weather, in your terminal");
    println!("{BOLD}{FORE_LIGHTBLUE}Version{RESET} {FORE_GREEN}{}", env!("CARGO_PKG_VERSION"));
}

pub fn credits() {
    println!(
        "Backends:
Meteo - https://open-meteo.com
Open Weather Map - https://openweathermap.org/
NWS - https://weather.gov"
    );
    println!("App icons from Icons8: https://icons8.com/");
}

pub fn settings() -> crate::Result<()> {
    let mut settings = Settings::new()?;
    let result = prompt::multiselect(
        &[
            "Metric",
            "Show Alerts",
            "Constant Location",
            "Auto Update Resources",
        ],
        &[
            settings.metric_default,
            settings.show_alerts,
            settings.constant_location,
            settings.auto_update_internet_resources,
        ],
        None,
    )?;
    settings.metric_default = result[0];
    settings.show_alerts = result[1];
    settings.constant_location = result[2];
    settings.auto_update_internet_resources = result[3];
    settings.write()?;
    Ok(())
}

#[cfg(feature = "gui")]
pub fn open_settings_app() {
    settings_app::run_settings_app().expect("App Failed");
}

#[cfg(not(feature = "gui"))]
pub fn open_settings_app() {
    panic!("GUI support not enabled!");
}
