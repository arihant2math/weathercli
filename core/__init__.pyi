def get_location(no_sys_loc: bool) -> list[str]:
    pass

def get_combined_data_formatted(
    open_weather_map_api_url: str,
    open_weather_map_api_key: str,
    coordinates: list[str],
    metric: bool,
):
    pass

def hash_file(filename: str) -> str:
    pass

class WindData:
    def __init__(self, speed, heading):
        self.speed = speed
        self.heading = heading

class WeatherData:
    pass
