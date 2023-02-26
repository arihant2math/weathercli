use pyo3::prelude::*;

use crate::wind_data::WindData;

#[pyclass(subclass)]
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
    raw_data: String,
}

#[pymethods]
impl WeatherData {
    #[new]
    fn new(
        temperature: i16,
        min_temp: i16,
        max_temp: i16,
        region: String,
        wind: WindData,
        raw_data: String,
    ) -> Self {
        WeatherData {
            temperature,
            min_temp,
            max_temp,
            region,
            wind,
            raw_data,
        }
    }
}
