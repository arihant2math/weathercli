use std::mem::discriminant;
use std::sync::{Arc, Mutex};

use clap::Parser;
use firedbg_lib::fire;

use cli::arguments::{App, Command};
use cli::commands::{
    about, backend_commands, cache, credits, layout_commands, saved_commands, settings_commands,
    weather,
};
use cli::commands::util::{setup, update};
use cli::Datasource;
use custom_backend::dynamic_library_loader::ExternalBackends;
use custom_backend::load_custom_backends;
use local::settings::Settings;
use terminal::color;
use weather_dirs::custom_backends_dir;

use crate::logging::init_logging;

mod logging;

pub type Result<T> = std::result::Result<T, cli::Error>;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn print_backtrace(error: Box<dyn std::error::Error>) {
    if let Some(source) = error.source() {
        println!("{:?}", source);
    }
}

fn main() {
    let r = run();
    match r {
        Ok(()) => {}
        Err(e) => {
            println!("{}{e}", color::FORE_RED);
            #[cfg(debug_assertions)]
            print_backtrace(Box::new(e));
        }
    };
}

fn run() -> Result<()> {
    let args = App::parse();
    let mut settings_s = Settings::new()?;
    if settings_s.debug || args.global_opts.debug {
        let _handle = init_logging()?;
    }

    if args.global_opts.metric && args.global_opts.imperial {
        Err("Cannot use both metric and imperial units at the same time.")?;
    }

    let true_metric = if args.global_opts.metric {
        true
    } else if args.global_opts.imperial {
        false
    } else {
        settings_s.metric_default
    };
    let datasource = Datasource::from(
        &*args
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
    let wasm_backends = Arc::new(Mutex::new(
        if settings_s.enable_wasm_backends
            && discriminant(&datasource) == discriminant(&Datasource::Other(String::new()))
            && custom_backends_dir()?.exists()
        {
            custom_backend::wasm_loader::WasmLoader::new()?
        } else {
            custom_backend::wasm_loader::WasmLoader::default()
        },
    ));
    fire::dbg!("settings", &settings_s);
    match args.command {
        Some(command) => {
            match command {
                Command::Place(opts) => {
                    if let Some(query) = opts.query {
                        weather(
                            datasource,
                            local::location::geocode(query, &settings_s.bing_maps_api_key.clone())?,
                            args.global_opts.future,
                            settings_s,
                            true_metric,
                            args.global_opts.json,
                            custom_backends,
                            Arc::clone(&wasm_backends),
                        )?
                    } else {
                        weather(
                            datasource,
                            saved_commands::select(&mut settings_s)?.into(),
                            args.global_opts.future,
                            settings_s,
                            true_metric,
                            args.global_opts.json,
                            custom_backends,
                            Arc::clone(&wasm_backends),
                        )?
                    }
                }
                Command::About => about(),
                Command::Backend(arg) => backend_commands::subcommand(arg, &mut settings_s)?,
                Command::Cache(arg) => cache(arg)?,
                Command::Credits => credits(),
                Command::Settings(arg) => settings_commands::subcommand(arg, &mut settings_s)?, // TODO: Ability to view/reset settings or specific key
                Command::Layout(arg) => layout_commands::subcommand(arg, &mut settings_s)?,
                Command::Setup => setup(settings_s)?,
                Command::Update(opts) => update(opts.force, opts.dry_run, version())?,
                Command::Saved(arg) => saved_commands::subcommand(arg, &mut settings_s)?,
            };
        }
        None => weather(
            datasource,
            local::location::get_location(args.global_opts.no_sys_loc, settings_s.constant_location)?,
            args.global_opts.future,
            settings_s,
            true_metric,
            args.global_opts.json,
            custom_backends,
            Arc::clone(&wasm_backends),
        )?,
    };
    Ok(())
}
