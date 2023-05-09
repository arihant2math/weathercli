use std::mem::discriminant;

use clap::Parser;

use cli::{Datasource, datasource_from_str};
use cli::arguments::{App, Command};
use cli::commands::{backend_commands, cache, credits, layout_commands, open_settings_app, settings, weather};
use cli::commands::util::{setup, update};
use custom_backend::dynamic_library_loader::ExternalBackends;
use local::settings::Settings;
use terminal::color;
use weather_core::{init_logging, load_custom_backends, version};
use weather_dirs::custom_backends_dir;

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
    let custom_backends = if settings_s.enable_custom_backends
        && discriminant(&datasource) == discriminant(&Datasource::Other(String::new()))
        && custom_backends_dir()?.exists()
    {
        load_custom_backends()?
    } else {
        ExternalBackends::default()
    };
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
                Command::Backend(arg) => backend_commands::subcommand(arg, settings_s)?,
                Command::Cache(arg) => cache(arg)?,
                Command::Config(opts) => cli::commands::config(opts.key, opts.value)?,
                Command::Credits => credits(),
                Command::Settings => settings()?,
                Command::GuiSettings => open_settings_app(),
                Command::Layout(arg) => layout_commands::subcommand(arg, settings_s)?,
                Command::Setup => setup(settings_s)?,
                Command::Update(opts) => update(opts.force, version())?,
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
