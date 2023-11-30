use chrono::{DateTime, Utc};
use crate::WeatherData;

#[derive(Clone, Debug, PartialEq)]
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
    pub fn get_forecast_sentence(&self, time: DateTime<Utc>) -> crate::Result<String> {
        // let future_forecasts: Vec<WeatherData> = self.forecast.iter().filter(|&d| (d.time - time).num_seconds() > 0).collect();
        let current = self.get_best_forecast(time)?;
        // if future_forecasts.len() < 2 {
        //     return Err("No future forecasts".into());
        // }
        todo!("Implement get_forecast_sentence");
    }
}
