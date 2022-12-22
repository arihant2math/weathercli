use crate::wind_data::WindData;
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
pub struct WeatherData {
    #[pyo3(get, set)]
    temperature: i16,
    #[pyo3(get, set)]
    min_temp: i16,
    #[pyo3(get, set)]
    max_temp: i16,
    #[pyo3(get, set)]
    region: String,
    #[pyo3(get, set)]
    wind: WindData,
    #[pyo3(get, set)]
    raw_data: String
}


