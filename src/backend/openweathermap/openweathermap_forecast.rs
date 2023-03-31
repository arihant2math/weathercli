use std::rc::Rc;

use crate::backend;
use crate::backend::openweathermap::openweathermap_current::OpenWeatherMapCurrent;
use crate::backend::openweathermap::openweathermap_future::OpenWeatherMapFuture;
use crate::backend::status::Status;
use crate::backend::weather_data::WeatherDataRS;
use crate::backend::weather_forecast::WeatherForecastRS;
use crate::local::settings::Settings;

struct OpenWeatherMapForecast {
    status: Status,
    region: String,
    country: String,
    forecast: Vec<Rc<dyn WeatherDataRS>>,
    forecast_sentence: String,
}

fn get_forecast_sentence(forecast: Vec<Rc<dyn WeatherDataRS>>) -> String {
    let data = forecast.clone();
    let mut rain: Vec<bool> = Vec::new();
    let mut snow: Vec<bool> = Vec::new();
    for period in &data {
        if period.get_conditions()[0].condition_id / 100 == 5 {
            rain.push(true);
            snow.push(false);
        } else if period.get_conditions()[0].condition_id / 100 == 6 {
            snow.push(true);
            rain.push(false);
        } else {
            rain.push(false);
            snow.push(false);
        }
    }
    if data[0].get_conditions()[0].condition_id / 100 == 5 {
        let mut t = 0;
        for i in rain {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue raining for {} hours.", t * 3);
    } else if data[0].get_conditions()[0].condition_id / 100 == 6 {
        let mut t = 0;
        for i in snow {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue snowing for {} hours.", t * 3);
    } else {
        let mut t = 0;
        for period in rain {
            if period {
                return format!("It will rain in {} hours", t * 3);
            }
            t += 1
        }
        t = 0;
        for period in snow {
            if period {
                return format!("It will snow in {} hours", t * 3);
            }
            t += 1
        }
    }
    "Conditions are predicted to be clear for the next 3 days.".to_string()
}

impl WeatherForecastRS for OpenWeatherMapForecast {
    fn new(coordinates: Vec<String>, settings: Settings) -> Self {
        if settings.internal.open_weather_map_api_key.is_none()
            || settings
                .internal
                .open_weather_map_api_key
                .clone()
                .unwrap_or("".to_string())
                != ""
        {
            panic!("Improper openweathermap api key")
        }
        let data = backend::openweathermap::open_weather_map_get_combined_data_formatted(
            "https://api.openweathermap.org/data/2.5/",
            settings.internal.open_weather_map_api_key.clone().unwrap(),
            coordinates,
            settings.internal.metric_default.unwrap(),
        );
        let mut forecast: Vec<Rc<dyn WeatherDataRS>> = Vec::new();
        forecast.push(Rc::new(OpenWeatherMapCurrent::new(
            data.weather.clone(),
            data.air_quality.clone(),
        )));
        for item in data.forecast.list.into_iter() {
            forecast.push(Rc::new(OpenWeatherMapFuture::new(item)));
        }
        let forecast_sentence = get_forecast_sentence(forecast.clone());
        OpenWeatherMapForecast {
            status: Status::OK,
            region: data.weather.name,
            country: data.weather.sys.country,
            forecast,
            forecast_sentence,
        }
    }

    fn get_status(&self) -> Status {
        self.status
    }

    fn get_region(&self) -> String {
        self.region.clone()
    }

    fn get_country(&self) -> String {
        self.country.clone()
    }

    fn get_forecast(&self) -> Vec<Rc<dyn WeatherDataRS>> {
        self.forecast.clone()
    }

    fn get_current_weather(&self) -> Rc<dyn WeatherDataRS> {
        let f = self.forecast.clone();
        let r = f.get(0);
        r.expect("0th element expected").clone()
    }

    fn get_forecast_sentence(&self) -> String {
        self.forecast_sentence.clone()
    }

    fn get_raw_data(&self) -> Option<Vec<String>> {
        None
    }
}
