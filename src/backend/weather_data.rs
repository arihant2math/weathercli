use serde::{Deserialize, Serialize};

use crate::backend::weather_condition::WeatherCondition;
use crate::backend::WindData;

pub fn get_conditions_sentence(conditions: Vec<WeatherCondition>) -> String {
    let mut data = conditions;
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

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub time: i128,
    pub temperature: f32,
    pub min_temp: f32,
    pub max_temp: f32,
    pub wind: WindData,
    pub raw_data: String,
    pub dewpoint: f32,
    pub feels_like: f32,
    pub aqi: u8,
    pub cloud_cover: u8,
    pub conditions: Vec<WeatherCondition>,
    pub condition_sentence: String,
}
