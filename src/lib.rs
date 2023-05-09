// TODO: switch allocator to jebmalloc due to simd_json performance

use log::LevelFilter;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Handle;

use custom_backend::dynamic_library_loader::{is_valid_ext, ExternalBackends};
#[cfg(feature = "gui")]
use settings_app;
use weather_dirs::weathercli_dir;

use log::debug;
use std::{fs, io};
use weather_dirs::custom_backends_dir;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

pub fn init_logging() -> crate::Result<Handle> {
    let level = LevelFilter::Info;
    let mut file_path = weathercli_dir()?.join("logs");
    file_path.push(format!("{}.log", now::now()));
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
    let config = log4rs::Config::builder()
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
    Ok(log4rs::init_config(config).unwrap())
}

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

pub fn load_custom_backends() -> crate::Result<ExternalBackends> {
    debug!("Detecting external dlls");
    let path = custom_backends_dir()?;
    let plugins: Vec<String> = path
        .read_dir()
        .expect("Reading the custom_backends dir failed")
        .filter(is_ext) // We only care about files
        .map(|f| f.unwrap().path().display().to_string())
        .collect();
    debug!("Loading: {plugins:?}");
    let custom_backends = custom_backend::dynamic_library_loader::load(plugins);
    Ok(custom_backends)
}
