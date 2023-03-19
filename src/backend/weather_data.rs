use pyo3::prelude::*;

use crate::backend::weather_condition::WeatherCondition;
use crate::backend::wind_data::WindData;

#[pyclass(subclass)]
#[derive(Clone)]
pub struct WeatherData {
    #[pyo3(get, set)]
    time: i128,
    #[pyo3(get, set)]
    temperature: f32,
    #[pyo3(get, set)]
    min_temp: f32,
    #[pyo3(get, set)]
    max_temp: f32,
    #[pyo3(get, set)]
    wind: WindData,
    #[pyo3(get, set)]
    raw_data: String,
    #[pyo3(get, set)]
    dewpoint: f32,
    #[pyo3(get, set)]
    feels_like: f32,
    #[pyo3(get, set)]
    aqi: u8,
    #[pyo3(get, set)]
    cloud_cover: u8,
    #[pyo3(get, set)]
    conditions: Vec<WeatherCondition>,
    #[pyo3(get, set)]
    condition_sentence: String,
}

#[pymethods]
impl WeatherData {
    #[new]
    pub(crate) fn new() -> Self {
        WeatherData {
            time: 0,
            temperature: 0.0,
            min_temp: 0.0,
            max_temp: 0.0,
            wind: WindData {
                speed: 0.0,
                heading: 0,
            },
            raw_data: String::new(),
            dewpoint: 0.0,
            feels_like: 0.0,
            aqi: 0,
            cloud_cover: 0,
            conditions: Vec::new(),
            condition_sentence: String::new(),
        }
    }

    fn get_condition_ids(&self) -> Vec<u16> {
        let mut ids: Vec<u16> = Vec::new();
        for condition in self.conditions.clone() {
            ids.push(condition.condition_id)
        }
        ids
    }
    pub fn get_conditions_sentence(&self) -> String {
        let mut data = self.conditions.clone();
        let conditions_match = data
            .get(0)
            .expect("0th element expected")
            .sentence
            .to_string();
        let mut conditions_sentences = conditions_match;
        data.remove(0);
        for condition in data {
            conditions_sentences += ". Also, ";
            conditions_sentences += &*condition.sentence.to_lowercase();
            conditions_sentences += ".";
        }
        conditions_sentences
    }
}
