mod layout_commands;
mod backend;

use std::str::FromStr;
use std::{fs, thread};
use std::time::Duration;

use log::debug;
use serde_json::Value;

use crate::cli::{print_out, Datasource};
use crate::cli::arguments::{CacheOpts, LayoutOpts, BackendOpts};
use crate::dynamic_loader::ExternalBackends;
use crate::local::settings::Settings;
use crate::local::weather_file::WeatherFile;
use crate::{updater, get_data_from_datasource, networking, version};
use crate::local::cache::prune;
use crate::location::Coordinates;

pub fn weather(
    datasource: Datasource,
    coordinates: Coordinates,
    settings: Settings,
    true_metric: bool,
    json: bool,
    custom_backends: ExternalBackends,
) -> crate::Result<()> {
    debug!("Coordinates: {} {}", coordinates.latitude, coordinates.longitude);
    debug!("Metric: {}", true_metric);
    debug!("json: {}", json);
    let mut s = settings.clone();
    s.internal.metric_default = true_metric;
    let data = get_data_from_datasource(datasource, coordinates, s, custom_backends)?;
    print_out(settings.internal.layout_file, data, json, true_metric)?;
    Ok(())
}

pub fn config(key_name: String, value: Option<String>) -> crate::Result<()> {
    match value {
        None => {
            let f = WeatherFile::settings()?;
            let data: Value = serde_json::from_str(&f.get_text()?)?;
            println!("{}: {}", &key_name, data[&key_name]);
        }
        Some(real_value) => {
            println!("Writing {}={} ...", key_name.to_lowercase(), &real_value);
            let mut f = WeatherFile::settings()?;
            let mut data: Value = serde_json::from_str(&f.get_text()?)?;
            data[key_name.to_uppercase()] = Value::from_str(&real_value)?;
            f.data = Vec::from(serde_json::to_string(&data)?);
            f.write()?;
        }
    };
    Ok(())
}

pub fn setup(settings_s: Settings) -> crate::Result<()> {
    let mut settings = settings_s;
    println!("{}===== Weather CLI Setup =====", crate::color::FORE_CYAN);
    updater::resource_updater::update_web_resources(None)?;
    println!(
        "{}Choose the default weather backend: ",
        crate::color::FORE_RED
    );
    let options = [
        "Meteo",
        "Open Weather Map",
        "National Weather Service",
        "The Weather Channel",
    ];
    let mut default = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"]
        .iter()
        .position(|&x| x == settings.internal.default_backend.clone())
        .unwrap_or(0);
    let current = crate::prompt::choice(&options, default, None)?;
    let weather_backend_setting = ["METEO", "OPENWEATHERMAP", "NWS", "THEWEATHERCHANNEL"][current];
    settings.internal.default_backend = weather_backend_setting.to_string();
    settings.write()?;
    thread::sleep(Duration::from_millis(100));
    println!(
        "{}Is your location constant (i.e. is this computer stationary at all times)?",
        crate::color::FORE_RED
    );
    if settings.internal.constant_location {
        default = 0;
    } else {
        default = 1;
    }
    let constant_location_setting =
        [true, false][crate::prompt::choice(&["yes", "no"], default, None)?];
    settings.internal.constant_location = constant_location_setting;
    settings.write()?;
    thread::sleep(Duration::from_millis(100));
    println!(
        "{}Should static resources (ascii art, weather code sentences, etc.) be auto-updated?",
        crate::color::FORE_RED
    );
    if settings.internal.auto_update_internet_resources {
        default = 0;
    } else {
        default = 1;
    }
    let auto_update_setting = [true, false][crate::prompt::choice(&["yes", "no"], default, None)?];
    settings.internal.auto_update_internet_resources = auto_update_setting;
    settings.write()?;
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

pub fn update(force: bool) -> crate::Result<()> {
    println!("Checking for updates ...");
    let latest_version = updater::get_latest_version()?;
    let application_path = std::env::current_exe()?;
    println!("Latest Version: {latest_version}");
    println!("Current Version: {}", version());
    if latest_version != version() || force {
        println!("Updating weather.exe at {}", application_path.display());
        let mut updater_location = application_path
            .parent()
            .expect("no parent dir")
            .to_path_buf().join("components");
        fs::create_dir_all(&updater_location)?;
        if cfg!(windows) {
            updater_location.push("updater.exe");
        } else {
            updater_location.push("updater");
        }
        if !updater_location.exists() {
            println!("Updater not found, downloading updater");
            updater::get_updater(updater_location.display().to_string())?;
            let resp: Value = serde_json::from_str(
                &networking::get_url(
                    "https://arihant2math.github.io/weathercli/index.json",
                    None,
                    None,
                    None,
                )?
                .text,
            )?;
            let mut web_hash = resp["updater-exe-hash-unix"]
                .as_str()
                .expect("updater-exe-hash-unix key not found in index.json");
            if cfg!(windows) {
                web_hash = resp["updater-exe-hash-windows"]
                    .as_str()
                    .expect("updater-exe-hash-windows key not found in index.json");
            }
            if crate::util::hash_file(&updater_location.display().to_string())? != web_hash || force
            {
                updater::get_updater(updater_location.display().to_string())?;
            }
            drop(resp); // Dropping to reduce memory leakage
            println!("Starting updater and exiting");
            let mut command = std::process::Command::new(updater_location.as_os_str());
            if force {
                command.arg("--force").spawn()?;
            }
            else {
                command.spawn()?;
            }
            std::process::abort();
        }
    }
    Ok(())
}

pub fn credits() {
    println!("Backends:
    Meteo - https://open-meteo.com
    Open Weather Map - https://openweathermap.org/
    NWS - https://weather.gov");
    if cfg!(feature = "gui") || cfg!(windows) {
        println!("Icons from Icons8: https://icons8.com/");
    }
}

pub fn layout(arg: LayoutOpts, settings: Settings) -> crate::Result<()> {
    match arg {
        LayoutOpts::Install(opts) => layout_commands::install(opts.path)?,
        LayoutOpts::List => layout_commands::list(settings)?,
        LayoutOpts::Select => layout_commands::select(settings)?,
        LayoutOpts::Delete => layout_commands::delete(settings)?
    };
    Ok(())
}

pub fn custom_backend(arg: BackendOpts, settings: Settings) -> crate::Result<()> {
    match arg {
        BackendOpts::Install(opts) => backend::install(opts.path)?,
        BackendOpts::List => backend::list(settings)?,
        BackendOpts::Delete => backend::delete()?
    }
    Ok(())
}

