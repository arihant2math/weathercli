use std::fmt::{Display, Formatter};

use log::warn;

pub use error::Error;
use layout::layout_input::LayoutInput;
use layout::LayoutErr;
use layout::LayoutFile;
use serde_json;

pub mod arguments;
pub mod commands;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Datasource {
    Meteo,
    Openweathermap,
    OpenweathermapOneCall,
    NWS,
    Other(String),
}

impl From<&str> for Datasource {
    fn from(s: &str) -> Self {
        match &*s.to_lowercase() {
            "nws" => Datasource::NWS,
            "openweathermap" => Datasource::Openweathermap,
            "openweathermap_onecall" => Datasource::OpenweathermapOneCall,
            "meteo" => Datasource::Meteo,
            _ => Datasource::Other(s.to_string()),
        }
    }
}

impl Display for Datasource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Datasource::Meteo => write!(f, "Meteo"),
            Datasource::Openweathermap => write!(f, "OpenWeatherMap"),
            Datasource::OpenweathermapOneCall => write!(f, "OpenWeatherMap OneCall"),
            Datasource::NWS => write!(f, "NWS"),
            Datasource::Other(s) => write!(f, "{}", s),
        }
    }
}

fn print_out(layout_file: &str, data: LayoutInput, json: bool, metric: bool) -> crate::Result<()> {
    if json {
        println!("{}", serde_json::to_string_pretty(&data)?);
    } else {
        let mut out = LayoutFile::new(layout_file);
        if out.is_err() {
            warn!("Layout file had errors, defaulting to default.res.");
            out = LayoutFile::new("default.res");
        }
        println!(
            "{}",
            out.map_err(|e| layout::Error::LayoutError(LayoutErr {
                message: e.to_string(),
                row: None,
                item: None
            }))?
            .to_string(data, metric)?
        );
    }
    Ok(())
}
