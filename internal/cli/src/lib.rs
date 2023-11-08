use crate::layout::LayoutFile;
use backend::WeatherForecast;
use weather_error;
use weather_error::LayoutErr;

pub mod arguments;
pub mod commands;
pub mod layout;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

#[derive(Clone, Eq, PartialEq)]
pub enum Datasource {
    Meteo,
    Openweathermap,
    OpenweathermapOneCall,
    NWS,
    Other(String),
}

pub fn datasource_from_str(s: &str) -> Datasource {
    match &*s.to_lowercase() {
        "nws" => Datasource::NWS,
        "openweathermap" => Datasource::Openweathermap,
        "openweathermap_onecall" => Datasource::OpenweathermapOneCall,
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
        println!(
            "{}",
            out.map_err(|e| weather_error::Error::LayoutError(LayoutErr {
                message: e.to_string(),
                row: None,
                item: None
            }))?
            .to_string(data, metric)?
        );
    }
    Ok(())
}
