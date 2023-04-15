use std::rc::Rc;

use crate::backend::meteo::meteo_current::MeteoCurrent;
use crate::backend::meteo::meteo_future::MeteoFuture;
use crate::backend::meteo::meteo_get_combined_data_formatted;
use crate::backend::meteo::meteo_json::MeteoForecastJson;
use crate::backend::status::Status;
use crate::backend::weather_data::WeatherDataRS;
use crate::backend::weather_forecast::get_location;
use crate::backend::weather_forecast::WeatherForecastRS;
use crate::local::settings::Settings;

struct MeteoForecast {
    status: Status,
    region: String,
    country: String,
    forecast: Vec<Rc<dyn WeatherDataRS>>,
    forecast_sentence: String,
}

fn get_forecast_sentence(
    data: Vec<Rc<dyn WeatherDataRS>>,
    raw_data: MeteoForecastJson,
    start: usize,
) -> String {
    let mut rain = raw_data
        .hourly
        .rain
        .iter()
        .map(|x| x != &0.0)
        .collect::<Vec<bool>>();
    let mut snow = raw_data
        .hourly
        .snowfall
        .iter()
        .map(|x| x != &0.0)
        .collect::<Vec<bool>>();
    for _i in 0..start {
        rain.remove(0);
        snow.remove(0);
    } // TODO: Convert
    if data[0]
        .clone()
        .get_conditions()
        .into_iter()
        .map(|condition| condition.condition_id / 100 == 5)
        .collect::<Vec<bool>>()
        .contains(&true)
    {
        let mut t: u8 = 0;
        for i in rain {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue raining for {} hours.", t);
    }
    if data[0]
        .clone()
        .get_conditions()
        .into_iter()
        .map(|condition| condition.condition_id / 100 == 6)
        .collect::<Vec<bool>>()
        .contains(&true)
    {
        let mut t: u8 = 0;
        for i in snow {
            if !i {
                break;
            }
            t += 1;
        }
        return format!("It will continue snowing for {} hours.", t);
    } else {
        let rain_start = rain.clone().into_iter().position(|x| x == true);
        let snow_start = snow.clone().into_iter().position(|x| x == true);

        if rain_start.is_none() && snow_start.is_none() {
            return "Conditions are predicted to be clear for the next 7 days.".to_string();
        }
        rain.reverse();
        snow.reverse();
        let rain_end = rain.into_iter().position(|x| x == true);
        let snow_end = snow.into_iter().position(|x| x == true);
        if rain_start.is_some() {
            return format!(
                "It will rain in {} hours for {} hours",
                rain_start.unwrap(),
                rain_end.unwrap() - rain_start.unwrap()
            );
        }
        if snow_start.is_some() {
            return format!(
                "It will snow in {} hours for {} hours",
                snow_start.unwrap(),
                snow_end.unwrap() - snow_start.unwrap()
            );
        }
    }
    return String::from("Conditions are predicted to be clear for the next 7 days.");
}

impl WeatherForecastRS for MeteoForecast {
    fn new(coordinates: Vec<String>, settings: Settings) -> Self {
        let data = meteo_get_combined_data_formatted(
            coordinates.clone(),
            settings.internal.metric_default.unwrap(),
        );
        let mut forecast: Vec<Rc<dyn WeatherDataRS>> = Vec::new();
        let current = MeteoCurrent::new(
            data.weather.clone(),
            data.air_quality.clone(),
            settings.internal.metric_default.unwrap(),
        );
        let now = current.index;
        forecast.push(Rc::new(current));
        for i in now + 1..data.weather.hourly.time.clone().len() {
            forecast.push(Rc::new(MeteoFuture::new(
                data.weather.clone(),
                data.air_quality.clone(),
                settings.internal.metric_default.unwrap(),
                i,
            )));
        }
        let region_country = get_location(coordinates);
        let forecast_sentence = get_forecast_sentence(forecast.clone(), data.weather.clone(), now);
        MeteoForecast {
            status: Status::OK,
            region: region_country[0].clone(),
            country: region_country[1].clone(),
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
