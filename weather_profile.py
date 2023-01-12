import core
from cli.backend.openweathermap.openweathermap import OpenWeatherMap
from cli.layout import Layout

data = OpenWeatherMap(core.get_location(False), False)
l = Layout()
print(l.to_string(data, False))
