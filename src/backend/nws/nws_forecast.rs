use crate::backend::nws::nws_current::get_nws_current;
use crate::backend::nws::nws_get_combined_data_formatted;
use crate::backend::status::Status;
use crate::backend::weather_forecast::WeatherForecastRS;
use crate::local::settings::Settings;

pub fn get_nws_forecast(coordinates: Vec<String>, settings: Settings) -> WeatherForecastRS {
    let data = nws_get_combined_data_formatted(
        coordinates.clone(),
        settings.internal.metric_default.unwrap(),
    );
    let current = get_nws_current(data, settings.internal.metric_default.unwrap());
    WeatherForecastRS {
        status: Status::OK,
        region: "WIP".to_string(),
        country: "WIP".to_string(),
        forecast: vec![current.clone()],
        current_weather: current,
        forecast_sentence: "".to_string(),
        raw_data: None,
    }
}
