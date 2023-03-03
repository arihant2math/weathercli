use pyo3::prelude::*;

use crate::backend::weather_data::WeatherData;
use crate::status::Status;

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
    raw_data: Option<Vec<String>>
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
            raw_data: None
        }
    }
    #[setter]
    fn set_forecast(&mut self, value: Vec<WeatherData>) -> PyResult<()> {
        self.forecast = value;
        self.current_weather = self.forecast[0].clone();
        Ok(())
    }
}
