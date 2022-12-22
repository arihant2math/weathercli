use crate::wind_data::WindData;
use pyo3::prelude::*;

#[pyclass]
pub struct WeatherData {
    temperature: i16,
    min_temp: i16,
    max_temp: i16,
    region: String,
    wind: WindData,
    raw_data: String
}
