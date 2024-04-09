use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log4rs::Handle;
use log::LevelFilter;

use weather_dirs::weathercli_dir;

pub fn init_logging() -> crate::Result<Handle> {
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
