use chrono::serde::ts_seconds;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

use crate::WeatherCondition;
use crate::WindData;

pub fn get_conditions_sentence(conditions: &[WeatherCondition]) -> String {
    let mut data = conditions;
    let conditions_match = data
        .get(0)
        .expect("0th element expected")
        .sentence
        .to_string();
    let mut conditions_sentences = conditions_match;
    data = &data[1..];
    for condition in data {
        conditions_sentences += ". Also, ";
        conditions_sentences += &*condition.sentence.to_lowercase();
        conditions_sentences += ".";
    }
    conditions_sentences
}

fn _default_duration() -> Duration {
    Duration::hours(1)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PrecipitationData {
    pub amount: f32,
    #[serde(skip_serializing, skip_deserializing, default = "_default_duration")] // TODO: Fix
    pub time: Duration,
    pub probability: u8,
}

impl Default for PrecipitationData {
    fn default() -> Self {
        Self {
            amount: 0.0,
            time: Duration::hours(1),
            probability: 0,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WeatherData {
    #[serde(with = "ts_seconds")]
    pub time: DateTime<Utc>,
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
    pub rain_data: PrecipitationData,
    pub snow_data: PrecipitationData,
}
