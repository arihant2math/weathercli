use pyo3::prelude::*;
use serde_json::Value;

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
    pub image_ascii: String,
}

#[pymethods]
impl WeatherCondition {
    #[new]
    pub fn new(condition_id: u16, weather_codes: &str) -> Self {
        let mut data: Value = serde_json::from_str(weather_codes).expect("Json expected");
        let code = data[condition_id.to_string()].as_array_mut().unwrap();
        let sentence = code[3]
            .clone()
            .as_str()
            .expect("String expected")
            .to_string();
        let image_url = String::from("https://openweathermap.org/img/wn/")
            + code[2].clone().as_str().expect("String expected")
            + "@4x.png";
        let image_ascii = code[4].clone().as_str().unwrap().to_string();
        WeatherCondition {
            condition_id,
            image_url,
            sentence,
            image_ascii,
        }
    }
}
