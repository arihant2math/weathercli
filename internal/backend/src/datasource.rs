use weather_structs::WeatherForecast;

use local::location::Coordinates;
use local::settings::Settings;

// TODO: use
pub trait Datasource {
    fn get(&self, coordinates: &Coordinates, settings: Settings) -> crate::Result<WeatherForecast>;
}
