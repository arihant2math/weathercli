use pyo3::prelude::*;
use serde_json::Value;

use crate::weather_file::WeatherFile;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct WeatherCondition {
    #[pyo3(get)]
    pub condition_id: u16,
    #[pyo3(get)]
    pub image_url: String,
    #[pyo3(get)]
    pub sentence: String,
    #[pyo3(get)]
    pub image_ascii: String
}


#[pymethods]
impl WeatherCondition {
    #[new]
    fn new(condition_id: u16) -> Self {
        let f = WeatherFile::new("weather_codes.json".to_string());
        let mut data: Value = serde_json::from_str(&f.data).expect("Json expected");
        let code = data[condition_id.to_string()].as_array_mut().unwrap();
        let sentence = code[3].clone().as_str().expect("String expected").to_string();
        let image_url = "https://openweathermap.org/img/wn/".to_string() + code[2].clone().as_str().expect("String expected") + "@4x.png";
        let mut image_ascii = "".to_string();
        if code.len() > 4 {
            image_ascii = code[4].clone().as_str().unwrap().to_string();
        }
        WeatherCondition {
            condition_id,
            image_url,
            sentence,
            image_ascii
        }
    }
}
