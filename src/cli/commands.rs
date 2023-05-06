pub mod backend;
pub mod layout_commands;
pub mod util;

use std::str::FromStr;

use log::debug;
use serde_json::Value;

use crate::cli::arguments::CacheOpts;
use crate::cli::{print_out, Datasource};
use crate::custom_backend::dynamic_library_loader::ExternalBackends;
use crate::{get_data_from_datasource, prompt};
use crate::local::cache::prune;
use crate::local::settings::Settings;
use crate::local::weather_file::WeatherFile;
use crate::location::Coordinates;

pub fn weather(
    datasource: Datasource,
    coordinates: Coordinates,
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
    s.internal.metric_default = true_metric;
    let data = get_data_from_datasource(datasource, coordinates, s, custom_backends)?;
    print_out(settings.internal.layout_file, data, json, true_metric)?;
    Ok(())
}

pub fn config(key_name: String, value: Option<String>) -> crate::Result<()> {
    match value {
        None => {
            let f = WeatherFile::settings()?;
            unsafe {
                let data: Value = simd_json::serde::from_str(&mut f.get_text()?)?;
                println!("{}: {}", &key_name, data[&key_name]);
            }
        }
        Some(real_value) => {
            println!("Writing {}={} ...", key_name.to_lowercase(), &real_value);
            let mut f = WeatherFile::settings()?;
            unsafe {
                let mut data: Value = simd_json::serde::from_str(&mut f.get_text()?)?;
                data[key_name.to_uppercase()] = Value::from_str(&real_value)?;
                f.data = Vec::from(simd_json::serde::to_string(&data)?);
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

pub fn credits() {
    println!(
        "Backends:
    Meteo - https://open-meteo.com
    Open Weather Map - https://openweathermap.org/
    NWS - https://weather.gov"
    );
    if cfg!(feature = "gui") || cfg!(windows) {
        println!("Icons from Icons8: https://icons8.com/");
    }
}

pub fn settings() -> crate::Result<()> {
    prompt::multiselect(&["test1", "test2", "test3"], &[false, true, false], None)?;
    Ok(())
}
