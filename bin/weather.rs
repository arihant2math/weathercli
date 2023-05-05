use std::{fs, io};
use std::mem::discriminant;

use clap::Parser;
use log::debug;

use weather_core::{init_logging, location};
use weather_core::cli::{Datasource, datasource_from_str};
use weather_core::cli::arguments::{App, Command};
use weather_core::cli::commands::{backend, cache, credits, layout_commands, weather};
use weather_core::cli::commands::util::{setup, update};
use weather_core::custom_backend::dynamic_library_loader::{ExternalBackends, is_valid_ext};
use weather_core::local::dirs::custom_backends_dir;
use weather_core::local::settings::Settings;

fn is_ext(f: &io::Result<fs::DirEntry>) -> bool {
    match f {
        Err(_e) => false,
        Ok(dir) => {
            if dir.metadata().is_ok()
                && dir.metadata().unwrap().is_file()
                && is_valid_ext(dir.file_name().to_str().unwrap())
            {
                return true;
            }
            false
        }
    }
}

fn main() -> weather_core::Result<()> {
    let args = App::parse();
    let settings = Settings::new()?;
    if settings.internal.debug || args.global_opts.debug {
        let _handle = init_logging();
    }
    let mut true_metric = settings.internal.metric_default;
    if args.global_opts.metric {
        true_metric = true;
    }
    if args.global_opts.imperial {
        true_metric = false;
    }
    let datasource = datasource_from_str(
        &args
            .global_opts
            .datasource
            .unwrap_or_else(|| settings.internal.default_backend.clone()),
    );
    let mut custom_backends = ExternalBackends::default();
    if settings.internal.enable_custom_backends
        && discriminant(&datasource) == discriminant(&Datasource::Other(String::new()))
    {
        debug!("Detecting external dlls");
        let path = custom_backends_dir()?;
        if path.exists() {
            let plugins: Vec<String> = path
                .read_dir()
                .expect("Reading the custom_backends dir failed")
                .filter(is_ext) // We only care about files
                .map(|f| f.unwrap().path().display().to_string())
                .collect();
            debug!("Loading: {plugins:?}");
            custom_backends = weather_core::custom_backend::dynamic_library_loader::load(plugins);
        }
    }
    match args.command {
        Some(command) => {
            match command {
                Command::Place(opts) => weather(
                    datasource,
                    location::geocode(opts.query, settings.internal.bing_maps_api_key.clone())
                        .expect("Location not found"),
                    settings,
                    true_metric,
                    args.global_opts.json,
                    custom_backends,
                )?,
                Command::Backend(arg) => backend::subcommand(arg, settings)?,
                Command::Cache(arg) => cache(arg)?,
                Command::Config(opts) => weather_core::cli::commands::config(opts.key, opts.value)?,
                Command::Credits => credits(),
                Command::Settings => weather_core::open_settings_app(),
                Command::Layout(arg) => layout_commands::subcommand(arg, settings)?,
                Command::Setup => setup(settings)?,
                Command::Update(opts) => update(opts.force)?,
            };
        }
        None => weather(
            datasource,
            location::get(
                args.global_opts.no_sys_loc,
                settings.internal.constant_location,
            )?,
            settings,
            true_metric,
            args.global_opts.json,
            custom_backends,
        )?,
    };
    Ok(())
}
