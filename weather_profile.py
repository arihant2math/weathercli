from cli import Layout
from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.location import get_location

data = MeteoForecast(get_location(True), False)
layout = Layout()
print(layout.to_string(data, False))
