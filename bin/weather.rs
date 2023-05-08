use std::mem::discriminant;

use clap::Parser;

use ansi as color;
use weather_core::{init_logging, load_custom_backends, location};
use weather_core::cli::{Datasource, datasource_from_str};
use weather_core::cli::arguments::{App, Command};
use weather_core::cli::commands::{backend, cache, credits, layout_commands, settings, weather};
use weather_core::cli::commands::util::{setup, update};
use weather_core::custom_backend::dynamic_library_loader::ExternalBackends;
use weather_core::local::dirs::custom_backends_dir;
use weather_core::local::settings::Settings;

fn main() {
    let r = run();
    match r {
        Ok(()) => {}
        Err(e) => {
            println!("{}{e}", color::FORE_RED);
        }
    };
}

fn run() -> weather_core::Result<()> {
    let args = App::parse();
    let settings_s = Settings::new()?;
    if settings_s.debug || args.global_opts.debug {
        let _handle = init_logging();
    }
    let true_metric = if args.global_opts.metric {
        true
    } else if args.global_opts.imperial {
        false
    } else {
        settings_s.metric_default
    };
    let datasource = datasource_from_str(
        &args
            .global_opts
            .datasource
            .unwrap_or_else(|| settings_s.default_backend.clone()),
    );
    let mut custom_backends = ExternalBackends::default();
    if settings_s.enable_custom_backends
        && discriminant(&datasource) == discriminant(&Datasource::Other(String::new()))
        && custom_backends_dir()?.exists()
    {
        custom_backends = load_custom_backends()?;
    }
    match args.command {
        Some(command) => {
            match command {
                Command::Place(opts) => weather(
                    datasource,
                    location::geocode(opts.query, settings_s.bing_maps_api_key.clone())?,
                    settings_s,
                    true_metric,
                    args.global_opts.json,
                    custom_backends,
                )?,
                Command::Backend(arg) => backend::subcommand(arg, settings_s)?,
                Command::Cache(arg) => cache(arg)?,
                Command::Config(opts) => weather_core::cli::commands::config(opts.key, opts.value)?,
                Command::Credits => credits(),
                Command::Settings => settings()?,
                Command::GuiSettings => weather_core::open_settings_app(),
                Command::Layout(arg) => layout_commands::subcommand(arg, settings_s)?,
                Command::Setup => setup(settings_s)?,
                Command::Update(opts) => update(opts.force)?,
            };
        }
        None => weather(
            datasource,
            location::get(args.global_opts.no_sys_loc, settings_s.constant_location)?,
            settings_s,
            true_metric,
            args.global_opts.json,
            custom_backends,
        )?,
    };
    Ok(())
}
