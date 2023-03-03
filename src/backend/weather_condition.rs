use pyo3::prelude::*;
use serde_json::Value;

use crate::weather_file::WeatherFile;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct WeatherCondition {
    #[pyo3(get, set)]
    pub(crate) condition_id: u16,
    #[pyo3(get, set)]
    pub sentence: String,
}

#[pyfunction]
pub fn get_sentence(id: u16) -> String {
    let f = WeatherFile::new("weather_codes.json".to_string());
    let data: Value = serde_json::from_str(&f.data).expect("Json expected");
    let code = &data[id.to_string()];
    let sentence = code[3].clone();
    sentence.as_str().expect("String expected").to_string()
}

#[pymethods]
impl WeatherCondition {
    #[new]
    fn new(condition_id: u16) -> Self {
        WeatherCondition {
            condition_id,
            sentence: get_sentence(condition_id),
        }
    }
}
