use std::{fs, io};
use std::mem::discriminant;

use clap::Parser;
use log::{debug, LevelFilter};
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;

use weather_core::cli::{Datasource, datasource_from_str};
use weather_core::cli::arguments::{App, Command};
use weather_core::cli::commands::{cache, credits, custom_backend, layout, setup, update, weather};
use weather_core::dynamic_loader::ExternalBackends;
use weather_core::local::dirs::weathercli_dir;
use weather_core::local::settings::Settings;
use weather_core::location;
use weather_core::now;

#[cfg(target_family = "windows")]
fn is_valid_ext(f: &str) -> bool {
    let len = f.len();
    &f[len - 4..] == ".dll"
}

#[cfg(target_family = "unix")]
fn is_valid_ext(f: &str) -> bool {
    let len = f.len();
    &f[len - 3..] == ".so"
}

fn is_ext(f: &io::Result<fs::DirEntry>) -> bool {
    match f {
        Err(_e) => false, // TODO: Re-emit error if needed
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
        let level = LevelFilter::Info;
        let mut file_path = weathercli_dir()?.join("logs");
        file_path.push(format!("{}.log", now()));
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

        // Log Trace level output to file where trace is the default level
        // and the programmatically specified level to stderr.
        let config = Config::builder()
            .appender(Appender::builder().build("logfile", Box::new(logfile)))
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(
                Root::builder()
                    .appender("logfile")
                    .appender("stderr")
                    .build(LevelFilter::Trace),
            )
            .unwrap();

        // Use this to change log levels at runtime.
        // This means you can change the default log level to trace
        // if you are trying to debug an issue and need more logs on then turn it off
        // once you are done.
        let _handle = log4rs::init_config(config).unwrap();
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
        let path = weathercli_dir()?.join("custom_backends");
        if path.exists() {
            let plugins: Vec<String> = path
                .read_dir()
                .expect("Reading the custom_backends dir failed")
                .filter(|f| is_ext(f)) // We only care about files
                .map(|f| f.unwrap().path().display().to_string())
                .collect();
            debug!("Loading: {:?}", plugins);
            custom_backends = weather_core::dynamic_loader::load(plugins);
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
                Command::Config(opts) => weather_core::cli::commands::config(opts.key, opts.value)?,
                Command::Settings => weather_core::open_settings_app(),
                Command::Cache(arg) => cache(arg)?,
                Command::Layout(arg) => layout(arg)?,
                Command::CustomBackend(arg) => custom_backend(arg)?,
                Command::Setup => setup(settings)?,
                Command::Update(opts) => update(opts.force)?,
                Command::Credits => credits(),
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
    return Ok(());
}
