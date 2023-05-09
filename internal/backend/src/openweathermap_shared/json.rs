use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct OpenWeatherMapConditionJson {
    pub id: u16,
    pub main: String,
    pub description: String,
    pub icon: String,
}
