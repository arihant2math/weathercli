# encoding: utf-8
# module core
# manually generated
"""The rust portion of the code, documentation should be added to the core.pyi file in the root directory."""

# functions

def get_location(no_sys_loc: bool) -> list[str]:
    """:param no_sys_loc If true the location will always be gotten from the web, if not, the location will be gotten via the win32 api if possible"""
    pass

def get_combined_data_unformatted(
    open_weather_map_api_url: str,
    open_weather_map_api_key: str,
    coordinates: list[str],
    metric: bool,
) -> list[str]:
    pass

def get_urls(urls: list[str]) -> list[str]:
    pass

def is_update_available() -> str:
    pass

def get_updater(location: str):
    pass

def hash_file(path: str) -> str:
    pass

# classes

class WindData(object):
    """Represents the wind"""

    def __init__(self, speed: str, heading: int):
        self.speed = speed
        self.heading = heading
