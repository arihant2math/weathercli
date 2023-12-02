pub fn get_default_layout() -> String {
"NAME = DEFAULT
VERSION = 21
LAYOUT_VERSION = 5
--------------------------------
Weather for {@location.city}, {@location.state}, {@location.country}
{$FORE_LIGHTMAGENTA$@weather.condition_sentence}
{$FORE_LIGHTMAGENTA$@forecast_sentence}
Temperature: {@weather.temperature|° F|° C} with a low of {@weather.min_temp|° F|° C}, and a high of {@weather.max_temp|° F|° C}, feels like {@weather.feels_like|° F|° C}
Wind: {@weather.wind.speed| mph| km/h} at {@weather.wind.heading|°}
Cloud Cover: {@weather.cloud_cover|%}
Dew Point: {@weather.dewpoint|° F|° C}
AQI: {#color_aqi|@weather.aqi}".to_string()
}
