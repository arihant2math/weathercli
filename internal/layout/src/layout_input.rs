use serde::{Deserialize, Serialize};
use backend::{WeatherData, WeatherForecast};
use local::location::LocationData;

use chrono::{DateTime, Utc};

#[derive(Clone, Serialize, Deserialize)]
pub struct LayoutInput {
    pub datasource: String,
    pub location: LocationData,
    pub weather: WeatherData,
    pub forecast_sentence: String,
}

impl LayoutInput {
    pub fn from_forecast(forecast: WeatherForecast, time: DateTime<Utc>) -> weather_error::Result<Self> {
        Ok(LayoutInput {
            datasource: forecast.datasource.clone(),
            location: forecast.location.clone(),
            weather: forecast.get_best_forecast(time)?,
            forecast_sentence: forecast.get_forecast_sentence(time)?
        })
    }
}
