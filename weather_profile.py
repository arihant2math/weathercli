from cli import Layout
from cli.backend.meteo.meteo_forecast import MeteoForecast
from cli.location import get_location

data = MeteoForecast(get_location(False), False)
l = Layout()
print(l.to_string(data, False))
