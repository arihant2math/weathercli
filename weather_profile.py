from cli import LayoutFile
from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.location import get_location

data = MeteoForecast(get_location(True), False)
layout = LayoutFile()
print(layout.to_string(data, False))
