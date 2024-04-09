use backend::datasource::Datasource as DatasourceTrait;
use backend::WeatherForecast;
use chrono::{DateTime, Duration, Utc};
use custom_backend::dynamic_library_loader;
use custom_backend::dynamic_library_loader::ExternalBackends;
use custom_backend::wasm_loader::WasmLoader;
use layout::layout_input::LayoutInput;
use local::cache::info as cache_info;
use local::cache::prune;
use local::location::Coordinates;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use log::{debug, error, warn};
use parse_duration::parse as parse_duration;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;
use terminal::color::*;
use weather_dirs::resources_dir;

use crate::arguments::CacheOpts;
use crate::{print_out, Datasource};

pub mod backend_commands;
pub mod layout_commands;
pub mod saved_commands;
pub mod settings_commands;
pub mod util;

fn get_requested_time(time: Option<String>) -> DateTime<Utc> {
    match time {
        Some(t) => {
            let time = chrono::Utc::now()
                + chrono::Duration::from_std(
                    parse_duration(&t).unwrap_or(std::time::Duration::new(0, 0)),
                )
                .unwrap_or(Duration::zero());
            return time;
        }
        None => {
            return chrono::Utc::now();
        }
    }
}

fn get_datasource_class(
    datasource: Datasource,
    wasm_loader: Arc<Mutex<WasmLoader>>,
    custom_backends: dynamic_library_loader::ExternalBackends,
    enable_wasm_backends: bool,
    enable_custom_backends: bool,
) -> Box<dyn DatasourceTrait> {
    match datasource {
        Datasource::Openweathermap => Box::new(backend::openweathermap::OpenWeatherMap {}),
        Datasource::OpenweathermapOneCall => {
            Box::new(backend::openweathermap_onecall::OpenWeatherMapOneCall {})
        }
        Datasource::NWS => Box::new(backend::nws::NWS {}),
        Datasource::Meteo => Box::new(backend::meteo::Meteo {}),
        Datasource::Other(name) => Box::new(custom_backend::CustomBackend::new(
            name,
            wasm_loader,
            custom_backends,
            enable_wasm_backends,
            enable_custom_backends,
        )),
    }
}

fn get_data(
    datasource: Datasource,
    coordinates: Coordinates,
    settings: Settings,
    custom_backends: ExternalBackends,
    wasm_loader: Arc<Mutex<WasmLoader>>,
) -> crate::Result<WeatherForecast> {
    let attem = get_datasource_class(
        datasource.clone(),
        wasm_loader,
        custom_backends,
        settings.enable_wasm_backends,
        settings.enable_custom_backends,
    );
    Ok(attem.get(&coordinates, settings)?)
}

pub fn get_data_from_datasource(
    datasource: Datasource,
    coordinates: Coordinates,
    settings: Settings,
    custom_backends: ExternalBackends,
    wasm_loader: Arc<Mutex<WasmLoader>>,
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
    get_data(
        datasource,
        coordinates,
        settings,
        custom_backends,
        wasm_loader,
    )
}

pub fn weather(
    datasource: Datasource,
    coordinates: Coordinates,
    time: Option<String>,
    settings: Settings,
    true_metric: bool,
    json: bool,
    custom_backends: ExternalBackends,
    wasm_backends: Arc<Mutex<WasmLoader>>,
) -> crate::Result<()> {
    debug!(
        "Coordinates: {} {}",
        coordinates.latitude, coordinates.longitude
    );
    debug!("Metric: {true_metric}");
    debug!("json: {json}");
    let mut s = settings.clone();
    s.metric_default = true_metric;
    let data = get_data_from_datasource(datasource, coordinates, s, custom_backends, wasm_backends)
        .map_err(|e| {
            error!("Error getting data from datasource: {e}");
            e
        })?;
    print_out(
        &settings.layout_file,
        LayoutInput::from_forecast(data, get_requested_time(time))?,
        json,
        true_metric,
    )?;
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
        CacheOpts::Info => cache_info()?,
    }
    Ok(())
}

pub fn about() {
    println!("{BOLD}{UNDERLINE}{FORE_LIGHTRED}Weather, in your terminal{RESET}");
    println!(
        "{BOLD}{FORE_LIGHTBLUE}Version{RESET} {FORE_LIGHTGREEN}{version}{RESET}",
        version = env!("CARGO_PKG_VERSION")
    );
    println!("{FORE_LIGHTBLUE} view {RESET}weather credits{FORE_LIGHTBLUE} for more info{RESET}");
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

#[cfg(feature = "gui")]
pub fn open_settings_app() {
    settings_app::run_settings_app().expect("App Failed");
}

#[cfg(not(feature = "gui"))]
pub fn open_settings_app() {
    panic!("GUI support not enabled!");
}
