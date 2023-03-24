from core.location import get_location

from cli import LayoutFile
from cli.backend.meteo.meteo_forecast import MeteoForecast

data = MeteoForecast(get_location(True), False)
layout = LayoutFile()
print(layout.to_string(data, False))
