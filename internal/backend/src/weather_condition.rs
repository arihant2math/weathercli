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
    if cloud_cover == 0 {
        return Ok(WeatherCondition::new(800, weather_codes)?);
    } else if cloud_cover < 25 {
        return Ok(WeatherCondition::new(801,weather_codes)?);
    } else if cloud_cover < 50 {
        return Ok(WeatherCondition::new(802, weather_codes)?);
    } else if cloud_cover < 85 {
        return Ok(WeatherCondition::new(803, weather_codes)?);
    } else {
        return Ok(WeatherCondition::new(804, &weather_codes)?);
    }
}
