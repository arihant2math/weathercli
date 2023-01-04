import core
from cli import print_out
from cli.backend.openweathermap import OpenWeatherMap

data = OpenWeatherMap(core.get_location(False), False)
print_out(data, False, False, False)
