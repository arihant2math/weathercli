use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapConditionJson {
    pub id: u16,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct PrecipitationJson {
    #[serde(rename = "1h", default)]
    pub one_hour: f32,
    #[serde(rename = "3h", default)]
    pub three_hour: f32,
}

impl Default for PrecipitationJson {
    fn default() -> Self {
        Self {
            one_hour: 0.0,
            three_hour: 0.0,
        }
    }
}
