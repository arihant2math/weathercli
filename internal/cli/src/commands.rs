pub mod backend_commands;
pub mod layout_commands;
pub mod util;

use std::str::FromStr;
use std::thread;

use shared_deps::simd_json;

use crate::arguments::CacheOpts;
use crate::{print_out, Datasource};
use backend::{meteo, nws, openweathermap, openweathermap_onecall, WeatherForecast};
use custom_backend::dynamic_library_loader::ExternalBackends;
use local::cache::prune;
use local::location::Coordinates;
use local::settings::Settings;
use local::weather_file::WeatherFile;
use log::{debug, warn};
use shared_deps::serde_json::Value;
use std::path::Path;
use chrono::{DateTime, Duration, Utc};
use terminal::prompt;
use weather_dirs::resources_dir;

fn get_requested_time(time: Option<String>) -> DateTime<Utc> {
    match time {
        Some(t) => {
            let time = chrono::Utc::now() + Duration::hours(t.parse().unwrap());
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
) -> crate::Result<WeatherForecast> {
    let dir = resources_dir()?;
    let f1 = dir.join("weather_codes.res");
    let f2 = dir.join("weather_ascii_images.res");
    let update_server = settings.update_server.clone();
    if !(Path::exists(&dir) && Path::exists(&f1) && Path::exists(&f2)) {
        warn!("Forcing downloading of web resources");
        updater::resource::update_web_resources(update_server, None)?;
    } else if settings.auto_update_internet_resources {
        thread::spawn(move || {
            updater::resource::update_web_resources(update_server, None).unwrap_or(());
        });
    }
    debug!("Getting data from datasource: {datasource:?}");
    match datasource {
        Datasource::Openweathermap => {
            openweathermap::forecast::get_forecast(&coordinates, settings)
        }
        Datasource::OpenweathermapOneCall => {
            openweathermap_onecall::forecast::get_forecast(&coordinates, settings)
        }
        Datasource::NWS => nws::forecast::get_forecast(&coordinates, settings),
        Datasource::Meteo => meteo::forecast::get_forecast(&coordinates, settings),
        Datasource::Other(s) => {
            if settings.enable_custom_backends {
                custom_backends.call(&s, &coordinates, settings)
            } else {
                return Err(weather_error::Error::Other(
                    "Custom backends are disabled. Enable them in the settings.".to_string(), // TODO: more help
                ));
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
) -> crate::Result<()> {
    debug!(
        "Coordinates: {} {}",
        coordinates.latitude, coordinates.longitude
    );
    debug!("Metric: {true_metric}");
    debug!("json: {json}");
    let mut s = settings.clone();
    s.metric_default = true_metric;
    let data = get_data_from_datasource(datasource, coordinates, s, custom_backends)?;
    let requested_weather = data.get_best_forecast(get_requested_time(time))?;
    print_out(&settings.layout_file, data, requested_weather, json, true_metric)?;
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
    println!("Weather, in your terminal") // TODO: Incorperate command
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
