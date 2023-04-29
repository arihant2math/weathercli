use crate::backend::nws::nws_current::get_nws_current;
use crate::backend::nws::nws_get_combined_data_formatted;
use crate::backend::weather_forecast::WeatherForecast;
use crate::local::settings::Settings;
use crate::location;

pub fn get_nws_forecast(
    coordinates: [&str; 2],
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    let data = nws_get_combined_data_formatted(coordinates, settings.internal.metric_default)?;
    let current = get_nws_current(data, settings.internal.metric_default)?;
    let region_country = location::reverse_geocode(coordinates[0], coordinates[1])?;
    Ok(WeatherForecast {
        region: region_country[0].clone(),
        country: region_country[1].clone(),
        forecast: vec![current.clone()],
        current_weather: current,
        forecast_sentence: String::new(),
        raw_data: None,
    })
}
