use chrono::{DateTime, Utc};
use crate::WeatherData;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct WeatherForecast {
    pub datasource: String,
    pub location: local::location::LocationData,
    pub forecast: Vec<WeatherData>,
    pub forecast_sentence: String,
    pub raw_data: Option<Vec<String>>,
}

impl WeatherForecast {
    pub fn get_best_forecast(&self, time: DateTime<Utc>) -> WeatherData {
        let best_forecast = self.forecast.iter().min_by_key(|&d| (time - d.time).abs().num_seconds());
        return match best_forecast {
            Some(forecast) => forecast.clone(),
            None => self.forecast[0].clone(),
        };
    }
}
