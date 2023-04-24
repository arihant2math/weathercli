use std::{fs, io};

use clap::Parser;

use weather_core::cli::{App, Command, datasource_from_str};
use weather_core::cli::commands::{config, credits, setup, update, weather};
use weather_core::dynamic_loader::ExternalBackends;
use weather_core::local::cache::prune_cache;
use weather_core::local::dirs::home_dir;
use weather_core::local::settings::Settings;
use weather_core::local::weather_file::WeatherFile;
use weather_core::location::{get_coordinates, get_location};

#[cfg(target_family = "windows")]
fn is_valid_ext(f: &str) -> bool {
    let len = f.len();
    &f[len-4 ..] == ".dll"
}

#[cfg(target_family = "unix")]
fn is_valid_ext(f: &String) -> bool {
    let len = f.len();
    &f[len-3 ..] == ".so"
}


fn is_ext(f: &io::Result<fs::DirEntry>) -> bool {
    match f {
        Err(_e) => false, // TODO: Re-emit error
        Ok(dir) => {
            if dir.metadata().is_ok() && dir.metadata().unwrap().is_file() && is_valid_ext(dir.file_name().to_str().unwrap()) {
                return true
            }
            return false;
        }
    }
}

fn main() {
    let args = App::parse();
    let settings = Settings::new();
    let mut true_metric = settings.internal.metric_default;
    if args.global_opts.metric {
        true_metric = true;
    }
    if args.global_opts.imperial {
        true_metric = false;
    }
    let datasource = datasource_from_str(&args.global_opts.datasource.unwrap_or(
        settings.internal.default_backend.clone(),
    ));
    let mut custom_backends = ExternalBackends::default();
    if settings.internal.enable_custom_backends { // TODO: make sure its a custom backend
        let mut path = home_dir().expect("expect home dir");
        path.push(".weathercli");
        path.push("custom_backends");
        if path.exists() {
            let plugins: Vec<String> = path.read_dir().expect("Reading the custom_backends dir failed")
                .filter(|f| is_ext(f)) // We only care about files
                .map(|f| f.unwrap().path().display().to_string())
                .collect();
            custom_backends = weather_core::dynamic_loader::load(plugins);
        }
    }
    match args.command {
        Some(command) => {
            match command {
                Command::Place(opts) => weather(
                    datasource,
                    get_coordinates(opts.query, settings.internal.bing_maps_api_key.clone())
                        .expect("Location not found"),
                    settings,
                    true_metric,
                    args.global_opts.json,
                    custom_backends
                ),
                Command::Config(opts) => config(opts.key, opts.value),
                Command::Settings => weather_core::open_settings_app(),
                Command::ClearCache => {
                    let mut f = WeatherFile::new("d.cache");
                    f.data = String::new();
                    f.write();
                }
                Command::PruneCache => prune_cache(),
                Command::Setup => setup(settings),
                Command::Update(opts) => update(opts.force),
                Command::Credits => credits(),
            };
        }
        None => weather(
            datasource,
            get_location(
                args.global_opts.no_sys_loc,
                settings.internal.constant_location,
            ),
            settings,
            true_metric,
            args.global_opts.json,
            custom_backends
        ),
    };
}
