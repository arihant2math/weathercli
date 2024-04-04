use log::warn;

use layout::LayoutErr;

use layout::layout_input::LayoutInput;
use layout::LayoutFile;
use shared_deps::serde_json;

pub mod arguments;
pub mod commands;
pub mod error;

pub use error::Error;

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
