use local::location::Coordinates;
use local::settings::Settings;
use networking::Resp;
use weather_structs::WeatherForecast;

pub trait Backend<T> {
    fn get_api_urls(&self, coordinates: &Coordinates, settings: &Settings) -> Vec<String>;
    fn parse_data(
        &self,
        data: Vec<Resp>,
        coordinates: &Coordinates,
        settings: &Settings,
    ) -> crate::Result<T>;
    fn process_data(
        &self,
        data: T,
        coordinates: &Coordinates,
        settings: &Settings,
    ) -> crate::Result<WeatherForecast>;
}
