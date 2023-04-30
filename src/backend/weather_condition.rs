use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherCondition {
    pub condition_id: u16,
    pub image_url: String,
    pub sentence: String,
    pub image_ascii: String,
}


impl WeatherCondition {
    pub fn new(condition_id: u16, weather_codes: &HashMap<String, Vec<String>>) -> crate::Result<Self> {
        let code = weather_codes
            .get(&condition_id.to_string()).ok_or("No such condition")?;
        let sentence = code[3].clone();
        let image_url = String::from("https://openweathermap.org/img/wn/")
            + &code[2]
            + "@4x.png";
        let image_ascii = code[4].clone();
        Ok(Self {
            condition_id,
            image_url,
            sentence,
            image_ascii,
        })
    }
}
