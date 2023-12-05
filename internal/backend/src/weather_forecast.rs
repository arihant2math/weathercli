use chrono::{DateTime, Utc};
use crate::WeatherData;

#[derive(Clone, Debug, PartialEq)]
pub struct WeatherForecast {
    pub datasource: String,
    pub location: local::location::LocationData,
    pub forecast: Vec<WeatherData>,
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
        let future_forecasts: Vec<&WeatherData> = self.forecast.iter().filter(|&d| (d.time - time).num_seconds() > 0).collect();
        let current = self.get_best_forecast(time)?;
        if future_forecasts.len() < 2 {
            return Ok("No future forecasts".into());
        }
        let is_raining = current.rain_data.amount > 0.01;
        let is_snowing = current.snow_data.amount > 0.01;
        let is_cloudy = current.cloud_cover > 25;
        let _is_windy = current.wind.speed > 5.0;
        let rain_start = if is_raining {
            Some(current.time)
        } else {
            future_forecasts.iter().find(|&d| d.rain_data.amount > 0.01).map(|d| d.time)
        };
        let rain_end = future_forecasts.iter().rfind(|&d| d.rain_data.amount > 0.01).map(|d| d.time);
        let snow_start = if is_snowing {
            Some(current.time)
        } else {
            future_forecasts.iter().find(|&d| d.snow_data.amount > 0.01).map(|d| d.time)
        };
        let snow_end = future_forecasts.iter().rfind(|&d| d.snow_data.amount > 0.01).map(|d| d.time);
        let cloud_start = if is_cloudy {
            Some(current.time)
        } else {
            future_forecasts.iter().find(|&d| d.cloud_cover > 25).map(|d| d.time)
        };
        let cloud_end = future_forecasts.iter().rfind(|&d| d.cloud_cover > 25).map(|d| d.time);
        let mut sentence = Vec::new();
        if is_raining {
            if let Some(end) = rain_end {
                sentence.push(format!("It will continue raining for {} hours.", (end - rain_start.unwrap()).num_hours()));
            } else {
                sentence.push(format!("It is currently raining and will be for atleast {} hours.", (future_forecasts.last().unwrap().time - current.time).num_hours()));
            }
        } else if let Some(start) = rain_start {
            if let Some(end) = rain_end {
                sentence.push(format!("It will start raining at {} for {} hours.", start.format("%H:%M"), (end - start).num_hours()));
            } else {
                sentence.push(format!("It will start raining at {} for atleast {} hours.", start.format("%H:%M"), (future_forecasts.last().unwrap().time - start).num_hours()));
            }
        }

        if is_snowing {
            if let Some(end) = snow_end {
                sentence.push(format!("It will continue snowing for {} hours.", (end - snow_start.unwrap()).num_hours()));
            } else {
                sentence.push(format!("It is currently snowing and will be for atleast {} hours.", (future_forecasts.last().unwrap().time - current.time).num_hours()));
            }
        } else if let Some(start) = snow_start {
            if let Some(end) = snow_end {
                sentence.push(format!("It will start snowing at {} for {} hours..", start.format("%H:%M"), (end - start).num_hours()));
            } else {
                sentence.push(format!("It will start snowing at {} for atleast {} hours.", start.format("%H:%M"), (future_forecasts.last().unwrap().time - start).num_hours()));
            }
        }

        if is_cloudy {
            if let Some(end) = cloud_end {
                sentence.push(format!("It will remain cloudy for {} hours.", (end - cloud_start.unwrap()).num_hours()));
            } else {
                sentence.push(format!("It is currently cloudy and will be for atleast {} hours.", (future_forecasts.last().unwrap().time - current.time).num_hours()));
            }
        } else if let Some(start) = cloud_start {
            if let Some(end) = cloud_end {
                sentence.push(format!("It will be cloudy at {} for {} hours.", start.format("%H:%M"), (end - start).num_hours()));
            } else {
                sentence.push(format!("It will be cloudy at {} for atleast {} hours.", start.format("%H:%M"), (future_forecasts.last().unwrap().time - start).num_hours()));
            }
        }
        if sentence.is_empty() {
            sentence.push("No precipitation".into());
        }
        Ok(sentence.join(". "))
    }
}
