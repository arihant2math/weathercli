use backend::{WeatherData, WeatherForecast};
use weather_error;
use weather_error::LayoutErr;

use crate::layout::LayoutFile;
use crate::layout::layout_input::LayoutInput;

pub mod arguments;
pub mod commands;
pub mod layout;

pub type Result<T> = std::result::Result<T, weather_error::Error>;

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Datasource {
    Meteo,
    Openweathermap,
    OpenweathermapOneCall,
    NWS,
    Other(String),
}

impl Datasource {
    pub fn from_str(s: &str) -> Datasource {
        match &*s.to_lowercase() {
            "nws" => Datasource::NWS,
            "openweathermap" => Datasource::Openweathermap,
            "openweathermap_onecall" => Datasource::OpenweathermapOneCall,
            "meteo" => Datasource::Meteo,
            _ => Datasource::Other(s.to_string()),
        }
    }
}

fn print_out(
    layout_file: &str,
    data: WeatherForecast,
    requested_weather: WeatherData,
    json: bool,
    metric: bool,
) -> crate::Result<()> {
    if json {
        println!("{:#?}", data.raw_data.expect("No raw data to print"));
    } else {
        let mut out = LayoutFile::new(layout_file);
        if out.is_err() {
            out = LayoutFile::new("default.res");
        }
        let datasource = data.datasource.clone();
        let location = data.location.clone();
        println!(
            "{}",
            out.map_err(|e| weather_error::Error::LayoutError(LayoutErr {
                message: e.to_string(),
                row: None,
                item: None
            }))?
            .to_string(LayoutInput {
                datasource,
                location,
                weather: requested_weather,
                forecast_sentence: data.get_forecast_sentence(chrono::offset::Utc::now())?,
            }, metric)?
        );
    }
    Ok(())
}
