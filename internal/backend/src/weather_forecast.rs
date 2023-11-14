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
    pub fn get_best_forecast(&self, time: DateTime<Utc>) -> crate::Result<WeatherData> {
        let best_forecast = self.forecast.iter().min_by_key(|&d| (time - d.time).abs().num_seconds());
        Ok(match best_forecast {
            Some(forecast) => forecast.clone(),
            None => self.forecast.get(0).ok_or("No forecast (forecast has length zero). There is likely an issue with the backend, try --json for more info.")?.clone(),
        })
    }
}
