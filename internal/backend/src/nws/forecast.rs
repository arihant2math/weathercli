use crate::nws::current::get_current;
use crate::nws::get_combined_data_formatted;
use crate::WeatherForecast;
use local::location;
use local::settings::Settings;
use location::Coordinates;

pub fn get_forecast(
    coordinates: Coordinates,
    settings: Settings,
) -> crate::Result<WeatherForecast> {
    let data = get_combined_data_formatted(coordinates, settings.metric_default)?;
    let current = get_current(data, settings.metric_default)?;
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
