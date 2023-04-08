use crate::backend::meteo::meteo_json::{MeteoAirQualityJson, MeteoForecastJson};
use crate::backend::weather_condition::WeatherCondition;
use crate::backend::weather_data::{get_conditions_sentence, WeatherDataRS};
use crate::backend::wind_data::WindData;
use crate::local::weather_file::WeatherFile;
use crate::now;

pub struct MeteoCurrent {
    data: MeteoForecastJson,
    aqi: MeteoAirQualityJson,
    pub index: usize,
    metric: bool,
}

impl MeteoCurrent {
    pub fn new(data: MeteoForecastJson, aqi: MeteoAirQualityJson, metric: bool) -> Self {
        let index = data.hourly.time.iter().position(|r| *r == data.current_weather.time).unwrap();
        MeteoCurrent {
            data,
            aqi,
            metric,
            index
        }
    }
}

impl WeatherDataRS for MeteoCurrent {
    fn get_time(&self) -> i128 {
        now() as i128
    }

    fn get_temperature(&self) -> f32 {
        self.data.current_weather.temperature
    }

    fn get_min_temp(&self) -> f32 {
        self.data.daily.temperature_2m_min[0]
    }

    fn get_max_temp(&self) -> f32 {
        self.data.daily.temperature_2m_max[0]
    }

    fn get_wind(&self) -> WindData {
        WindData {
            speed: self.data.current_weather.windspeed as f64,
            heading: self.data.current_weather.winddirection as i16,
        }
    }

    fn get_raw_data(&self) -> String {
        serde_json::to_string_pretty(&self.data).expect("dump to string failed")
    }

    fn get_dewpoint(&self) -> f32 {
        self.data.hourly.dewpoint_2m[self.index]
    }

    fn get_feels_like(&self) -> f32 {
        self.data.hourly.apparent_temperature[self.index]
    }

    fn get_aqi(&self) -> u8 {
        self.aqi.hourly.european_aqi[self.index]
    }

    fn get_cloud_cover(&self) -> u8 {
        self.data.hourly.cloudcover[self.index]
    }

    fn get_conditions(&self) -> Vec<WeatherCondition> {
        let weather_file = WeatherFile::weather_codes();
        let mut conditions: Vec<WeatherCondition> = Vec::new();
        let cloud_cover = self.get_cloud_cover();
        if cloud_cover == 0 {
            conditions.push(WeatherCondition::new(800, &weather_file.data));
        }
        else if cloud_cover < 25
        {
            conditions.push(WeatherCondition::new(801, &weather_file.data));
        }
        else if cloud_cover < 50 {
            conditions.push(WeatherCondition::new(802, &weather_file.data));
        }
        else if cloud_cover < 85 {
            conditions.push(WeatherCondition::new(803, &weather_file.data));
        }
        else {
            conditions.push(WeatherCondition::new(804, &weather_file.data));
        }
        if self.data.hourly.rain[self.index] != 0.0 {
            let rain = self.data.hourly.rain[self.index];
            let metric = self.metric;
            if (0.0 < rain && rain < 0.098 && !metric
            ) || (0.0 < rain && rain  < 2.5 && metric) {
                conditions.push(WeatherCondition::new(500, &weather_file.data));
            } else if (rain < 0.39
            && !metric) || (
                rain < 10.0 && metric
            )
            {
                conditions.push(WeatherCondition::new(501, &weather_file.data));
            }
            else if (rain < 2.0 && !metric) || (
                rain < 50.0 && metric
            )
            {
                conditions.push(WeatherCondition::new(502, &weather_file.data));
            }
            else if rain != 0.0 {
                conditions.push(WeatherCondition::new(503, &weather_file.data));
            }
        }
        if self.data.hourly.snowfall[self.index] != 0.0 {
            conditions.push(WeatherCondition::new(601, &weather_file.data));
        }
        return conditions;
    }

    fn get_condition_sentence(&self) -> String {
        get_conditions_sentence(self.get_conditions())
    }
}