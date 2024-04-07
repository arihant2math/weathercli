// TODO: use
trait Datasource {
    fn get(&self, coordinates: &Coordinates, settings: Settings) -> crate::Result<WeatherForecast>;
}
