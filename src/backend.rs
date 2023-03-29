use openweathermap::FormattedData;
use pyo3::prelude::*;

pub mod openweathermap;
pub mod status;
pub mod weather_condition;
pub mod weather_data;
pub mod weather_forecast;
pub mod wind_data;

pub fn register_backend_module(py: Python<'_>, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "backend")?;
    child_module.add_function(wrap_pyfunction!(
        openweathermap::open_weather_map_get_combined_data_formatted,
        child_module
    )?)?;
    child_module.add_function(wrap_pyfunction!(
        openweathermap::open_weather_map_get_api_urls,
        child_module
    )?)?;
    child_module.add_class::<FormattedData>()?;
    child_module.add_class::<wind_data::WindData>()?;
    child_module.add_class::<weather_data::WeatherData>()?;
    child_module.add_class::<weather_condition::WeatherCondition>()?;
    child_module.add_class::<weather_forecast::WeatherForecast>()?;
    child_module.add_class::<status::Status>()?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}
