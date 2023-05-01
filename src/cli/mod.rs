use crate::error;
use crate::backend::weather_forecast::WeatherForecast;
use crate::error::LayoutErr;
use crate::layout::LayoutFile;

pub mod commands;
pub mod arguments;

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

fn print_out(
    layout_file: String,
    data: WeatherForecast,
    json: bool,
    metric: bool,
) -> crate::Result<()> {
    if json {
        println!("{:#?}", data.raw_data.expect("No raw data to print"));
    } else {
        let mut out = LayoutFile::new(layout_file);
        if out.is_err() {
            out = LayoutFile::new("default.res".to_string());
        }
        println!("{}", out.map_err(|e| error::Error::LayoutError(LayoutErr {
            message: e.to_string(),
            row: None,
            item: None
        }))?.to_string(data, metric)?);
    }
    Ok(())
}
