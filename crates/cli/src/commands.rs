use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

use chrono::{DateTime, Duration, Utc};
use log::{debug, error, warn};
use parse_duration::parse as parse_duration;
use firedbg_lib::fire;

use backend::WeatherForecast;
use custom_backend::dynamic_library_loader::ExternalBackends;
use custom_backend::wasm_loader::WasmLoader;
use layout::layout_input::LayoutInput;
use local::location::Coordinates;
use local::settings::Settings;
use terminal::color::*;
use weather_dirs::cache_dir;
use weather_dirs::resources_dir;

use crate::{Datasource, print_out};
use crate::arguments::CacheOpts;

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


fn get_data(
    datasource: Datasource,
    coordinates: Coordinates,
    settings: Settings,
    custom_backends: ExternalBackends,
    wasm_loader: Arc<Mutex<WasmLoader>>,
) -> crate::Result<WeatherForecast> {
    // TODO: cleanup
    Ok(match datasource {
        Datasource::Openweathermap => backend::run(Box::new(&backend::openweathermap::OpenWeatherMap), &coordinates, &settings)?,
        Datasource::OpenweathermapOneCall => {
            backend::run(Box::new(&backend::openweathermap_onecall::OpenWeatherMapOneCall), &coordinates, &settings)?
        }
        Datasource::NWS => backend::run(Box::new(&backend::nws::NWS), &coordinates, &settings)?,
        Datasource::Meteo => backend::run(Box::new(&backend::meteo::Meteo), &coordinates, &settings)?,
        Datasource::Other(name) => custom_backend::CustomBackend::new(
            name,
            wasm_loader,
            custom_backends,
            &settings
        ).get(&coordinates, &settings)?,
    })
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
    fire::dbg!("coordinates", &coordinates);
    debug!("Metric: {true_metric}");
    debug!("json: {json}");
    let mut s = settings.clone();
    s.metric_default = true_metric;
    let data = get_data_from_datasource(datasource.clone(), coordinates, s, custom_backends, wasm_backends)
        .map_err(|e| {
            error!("Error getting data from {datasource}: {e}");
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
            std::fs::remove_file(cache_dir()?)?;
        },
        CacheOpts::Info => println!("Coming soon!"), // TODO
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
