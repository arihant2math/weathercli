use pyo3::prelude::*;
use crate::wind_data::WindData;

#[pyclass]
pub struct WeatherData {
    temperature: i16,
    region: String,
    wind: WindData
}


#[pymethods]
impl WeatherData {
    #[new]
    fn new(temperature: i16, region: String, wind: WindData) -> Self {
        WeatherData { temperature, region, wind }
    }


     #[getter(temperature)]
     fn temperature(&self) -> PyResult<i16> {
        Ok(self.temperature)
     }

     #[getter(region)]
     fn region(&self) -> PyResult<String> {
        Ok(self.region.to_string())
     }

    #[getter(wind)]
     fn wind(&self) -> PyResult<WindData> {
        Ok(self.wind)
     }
}