use clap::Parser;
use cli::arguments::{App, Command};
use cli::commands::{
    about, backend_commands, cache, credits, layout_commands, open_settings_app, saved_commands, settings, weather,
};
use cli::commands::util::{setup, update};
use cli::Datasource;
use custom_backend::dynamic_library_loader::ExternalBackends;
use custom_backend::load_custom_backends;
use local::settings::Settings;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Handle;
use log::LevelFilter;
use std::mem::discriminant;
use terminal::color;
use weather_dirs::{custom_backends_dir, weathercli_dir};

pub type Result<T> = std::result::Result<T, cli::Error>;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn init_logging() -> Result<Handle> {
    let level = LevelFilter::Info;
    let mut file_path = weathercli_dir()?.join("logs");
    file_path.push(format!("{}.log", local::now()));
    // Build a stderr logger.
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new("{m}\n")))
        .build();
    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(PatternEncoder::new("[{l} {M} {d}] {m}\n")))
        .build(file_path.as_os_str().to_str().unwrap())
        .unwrap();
    let logfile_appender = Appender::builder().build("logfile", Box::new(logfile));
    let stderr_appender = Appender::builder()
        .filter(Box::new(ThresholdFilter::new(level)))
        .build("stderr", Box::new(stderr));
    // Create builder for log file and stderr with Trace level.
    let builder = Root::builder()
        .appender("logfile")
        .appender("stderr")
        .build(LevelFilter::Trace);
    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    let config = log4rs::Config::builder()
        .appender(logfile_appender)
        .appender(stderr_appender)
        .logger(Logger::builder().build("ureq", LevelFilter::Info))
        .logger(Logger::builder().build("reqwest", LevelFilter::Info))
        .logger(Logger::builder().build("rustls", LevelFilter::Info))
        .logger(Logger::builder().build("wgpu_core", LevelFilter::Warn))
        .logger(Logger::builder().build("wgpu_hal", LevelFilter::Warn))
        .logger(Logger::builder().build("iced_wgpu", LevelFilter::Warn))
        .logger(Logger::builder().build("cosmic_text", LevelFilter::Warn))
        .logger(Logger::builder().build("naga", LevelFilter::Info))
        .logger(Logger::builder().build("wasmtime_jit", LevelFilter::Warn))
        .logger(Logger::builder().build("wasmtime_cache", LevelFilter::Warn))
        .logger(Logger::builder().build("wasmtime_cranelift", LevelFilter::Warn))
        .logger(Logger::builder().build("cranelift_codegen", LevelFilter::Warn))
        .logger(Logger::builder().build("cranelift_wasm", LevelFilter::Warn))
        .build(builder)
        .unwrap();

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    Ok(log4rs::init_config(config).expect("Logging init failed"))
}

fn main() {
    let r = run();
    match r {
        Ok(()) => {}
        Err(e) => {
            println!("{}{e}", color::FORE_RED);
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
    let mut wasm_backends = if settings_s.enable_wasm_backends
        && discriminant(&datasource) == discriminant(&Datasource::Other(String::new()))
        && custom_backends_dir()?.exists()
    {
        custom_backend::wasm_loader::WasmLoader::new()?
    } else {
        custom_backend::wasm_loader::WasmLoader::default()
    };
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
                            &mut wasm_backends
                        )?
                    } else {
                        weather(
                            datasource,
                            saved_commands::select(settings_s.clone())?.into(),
                            args.global_opts.future,
                            settings_s,
                            true_metric,
                            args.global_opts.json,
                            custom_backends,
                            &mut wasm_backends
                        )?
                    }
                },
                Command::About => about(),
                Command::Backend(arg) => backend_commands::subcommand(arg, &mut settings_s)?,
                Command::Cache(arg) => cache(arg)?,
                Command::Config(opts) => cli::commands::config(opts.key, opts.value)?,
                Command::Credits => credits(),
                Command::Settings => settings()?,
                Command::GuiSettings => open_settings_app(),
                Command::Layout(arg) => layout_commands::subcommand(arg, settings_s)?,
                Command::Setup => setup(settings_s)?,
                Command::Update(opts) => update(opts.force, opts.dry_run, version())?,
                Command::Saved(arg) => saved_commands::subcommand(arg, settings_s)?,
            };
        }
        None => weather(
            datasource,
            local::location::get(args.global_opts.no_sys_loc, settings_s.constant_location)?,
            args.global_opts.future,
            settings_s,
            true_metric,
            args.global_opts.json,
            custom_backends,
            &mut wasm_backends
        )?,
    };
    Ok(())
}
