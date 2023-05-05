use crate::backend::nws::current::get_current;
use crate::backend::nws::nws_get_combined_data_formatted;
use crate::backend::weather_forecast::WeatherForecast;
use crate::local::settings::Settings;
use crate::location;
use crate::location::Coordinates;

pub fn get_forecast(
    coordinates: Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    let data = nws_get_combined_data_formatted(coordinates, settings.internal.metric_default)?;
    let current = get_current(data, settings.internal.metric_default)?;
    let region_country = location::reverse_geocode(coordinates)?;
    Ok(WeatherForecast {
        region: region_country[0].clone(),
        country: region_country[1].clone(),
        forecast: vec![current.clone()],
        current_weather: current,
        forecast_sentence: String::new(),
        raw_data: None,
    })
}
