use clap::{Args, Parser, Subcommand};

use crate::backend::weather_forecast::WeatherForecastRS;
use crate::layout::LayoutFile;

pub mod commands;

#[derive(Clone, Parser)]
#[command(version, author, about, name = "weathercli")]
pub struct App {
    #[command(flatten)]
    pub global_opts: GlobalOpts,
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Clone, Subcommand)]
pub enum Command {
    #[command(about = "Get the weather for a specific place")]
    Place(PlaceOpts),
    #[command(about = "Open the gui settings editor")]
    Settings,
    #[command(about = "Set a config variable via weather config [key] [value]")]
    Config(ConfigOpts),
    #[command(subcommand)]
    Cache(CacheOpts),
    #[command(about = "Run the interactive terminal setup")]
    Setup,
    #[command(about = "Update weathercli")]
    Update(UpdateOpts),
    #[command(about = "Various Credits")]
    Credits,
}

#[derive(Clone, Subcommand)]
pub enum CacheOpts {
    #[command(about = "Trim the size of the cache")]
    Prune,
    #[command(about = "Delete the cache")]
    Clear,
}

#[derive(Clone, Args)]
pub struct ConfigOpts {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Clone, Args)]
pub struct PlaceOpts {
    pub query: String,
}

#[derive(Clone, Copy, Args)]
pub struct UpdateOpts {
    #[arg(long, short, action, help = "Forces a reinstall of weathercli")]
    pub force: bool,
}

#[derive(Clone, Args)]
pub struct GlobalOpts {
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "Print raw json output, useful for debugging"
    )]
    pub json: bool,
    #[arg(
        long,
        short,
        global = true,
        value_enum,
        help = "Which datasource to use, note that openweathermap requires an API key"
    )]
    pub datasource: Option<String>,
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "the output will be in metric"
    )]
    pub metric: bool,
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "the output will be in imperial, overrides --metric"
    )]
    pub imperial: bool,
    #[arg(
        long,
        short,
        action,
        global = true,
        help = "If used, the location will not be gotten from the win32 api, if applicable"
    )]
    pub no_sys_loc: bool,
    #[arg(long, action, global = true, help = "Enables debugging")]
    pub debug: bool,
}

fn print_out(
    layout_file: String,
    data: WeatherForecastRS,
    json: bool,
    metric: bool,
) -> crate::Result<()> {
    if json {
        println!("{:#?}", data.raw_data.expect("No raw data to print"));
    } else {
        let mut out = LayoutFile::new(layout_file);
        if out.is_err() {
            out = LayoutFile::new("default.json".to_string());
        }
        println!("{}", out?.to_string(data, metric)?);
    }
    Ok(())
}

#[derive(Clone, Eq, PartialEq)]
pub enum Datasource {
    Meteo,
    Openweathermap,
    NWS,
    Other(String),
}

pub fn datasource_from_str(s: &str) -> Datasource {
    match &*s.to_lowercase() {
        "nws" => Datasource::NWS,
        "openweathermap" => Datasource::Openweathermap,
        "meteo" => Datasource::Meteo,
        _ => Datasource::Other(s.to_string()),
    }
}
