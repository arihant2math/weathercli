use backend::{WeatherData, WeatherForecast};
use local::location::LocationData;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutInput {
    pub datasource: String,
    pub location: LocationData,
    pub weather: WeatherData,
    pub forecast_sentence: String,
}

impl LayoutInput {
    pub fn from_forecast(forecast: WeatherForecast, time: DateTime<Utc>) -> crate::Result<Self> {
        Ok(LayoutInput {
            datasource: forecast.datasource.clone(),
            location: forecast.location.clone(),
            weather: forecast
                .get_best_forecast(time)
                .ok_or("Failed to get best forecast".to_string())?,
            forecast_sentence: forecast.get_forecast_sentence(time)?,
        })
    }
}
