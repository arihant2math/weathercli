use std::rc::Rc;

use pyo3::prelude::*;

use crate::backend::status::Status;
use crate::backend::weather_data::{WeatherData, WeatherDataRS};
use crate::local::settings::Settings;
use crate::location;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct WeatherForecast {
    #[pyo3(get, set)]
    status: Status,
    #[pyo3(get, set)]
    region: String,
    #[pyo3(get, set)]
    country: String,
    #[pyo3(get)]
    forecast: Vec<WeatherData>,
    #[pyo3(get)]
    current_weather: WeatherData,
    #[pyo3(get, set)]
    forecast_sentence: String,
    #[pyo3(get, set)]
    raw_data: Option<Vec<String>>,
}

#[pymethods]
impl WeatherForecast {
    #[new]
    fn new() -> Self {
        WeatherForecast {
            status: Status::OK,
            region: "".to_string(),
            country: "".to_string(),
            forecast: Vec::new(),
            current_weather: WeatherData::new(),
            forecast_sentence: "".to_string(),
            raw_data: None,
        }
    }
    #[setter]
    fn set_forecast(&mut self, value: Vec<WeatherData>) -> PyResult<()> {
        self.forecast = value;
        self.current_weather = self.forecast[0].clone();
        Ok(())
    }
}

pub fn get_location(loc: Vec<String>) -> [String; 2] {
    location::reverse_location(&loc[0], &loc[1])
}

pub trait WeatherForecastRS {
    fn new(coordinates: Vec<String>, settings: Settings) -> Self;
    fn get_status(&self) -> Status;
    fn get_region(&self) -> String;
    fn get_country(&self) -> String;
    fn get_forecast(&self) -> Vec<Rc<dyn WeatherDataRS>>;
    fn get_current_weather(&self) -> Rc<dyn WeatherDataRS>;
    fn get_forecast_sentence(&self) -> String;
    fn get_raw_data(&self) -> Option<Vec<String>>;
}
