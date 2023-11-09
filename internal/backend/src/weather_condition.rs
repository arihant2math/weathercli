use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub condition_id: u16,
    pub image_url: String,
    pub sentence: String,
    pub image_ascii: String,
}

impl WeatherCondition {
    pub fn new(
        condition_id: u16,
        weather_codes: &HashMap<String, Vec<String>>,
    ) -> crate::Result<Self> {
        let code = weather_codes
            .get(&condition_id.to_string())
            .ok_or("No such condition")?;
        let sentence = code[3].clone();
        let image_url = format!("https://openweathermap.org/img/wn/{}@4x.png", &code[2]);
        let image_ascii = code[4].clone();
        Ok(Self {
            condition_id,
            image_url,
            sentence,
            image_ascii,
        })
    }
}


pub fn get_clouds_condition(cloud_cover: u8, weather_codes: &HashMap<String, Vec<String>>) -> crate::Result<WeatherCondition> {
    match cloud_cover {
        0..=2 => {
            WeatherCondition::new(800, weather_codes)
        }
        3..=25 => {
            WeatherCondition::new(801, weather_codes)
        }
        26..=50 => {
            WeatherCondition::new(802, weather_codes)
        }
        51..=85 => {
            WeatherCondition::new(803, weather_codes)
        }
        86..=100 => {
            WeatherCondition::new(804, weather_codes)
        }
        _ => {
            Err("Cloud cover out of range".into())
        }
    }
}
