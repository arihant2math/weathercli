use std::rc::Rc;

use crate::backend;
use crate::backend::meteo::meteo_current::MeteoCurrent;
use crate::backend::meteo::meteo_get_combined_data_formatted;
use crate::backend::status::Status;
use crate::backend::weather_data::WeatherDataRS;
use crate::backend::weather_forecast::WeatherForecastRS;
use crate::local::settings::Settings;

struct MeteoForecast {
    status: Status,
    region: String,
    country: String,
    forecast: Vec<Rc<dyn WeatherDataRS>>,
    forecast_sentence: String,
}

impl WeatherForecastRS for MeteoForecast {
    fn new(coordinates: Vec<String>, settings: Settings) -> Self {
        let data = meteo_get_combined_data_formatted(coordinates, settings.internal.metric_default.unwrap());
        let mut forecast: Vec<Rc<dyn WeatherDataRS>> = Vec::new();
        let current = MeteoCurrent::new(
            data.weather.clone(),
            data.air_quality.clone(),
            settings.internal.metric_default,
        );
        let now = current.index;
        forecast.push(Rc::new(current));
        for item in data.forecast.list.into_iter() {
            forecast.push(Rc::new(OpenWeatherMapFuture::new(item)));
        }
        let forecast_sentence = get_forecast_sentence(forecast.clone());
        MeteoForecast {
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
