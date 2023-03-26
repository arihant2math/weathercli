import weather_core
from weather_core.location import get_location

from cli import LayoutFile
from cli.backend.openweathermap.openweathermap_forecast import OpenWeatherMapForecast

data = OpenWeatherMapForecast(get_location(True, False), False, weather_core.Settings().internal)
layout = LayoutFile()
print(layout.to_string(data, False))
