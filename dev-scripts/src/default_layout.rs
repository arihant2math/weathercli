pub fn get_default_layout() -> String {
"NAME = DEFAULT
VERSION = 22
LAYOUT_VERSION = 5
--------------------------------
Weather for {@location.city}, {@location.state}, {@location.country}
{$FORE_LIGHTMAGENTA$@weather.condition_sentence}
{$FORE_LIGHTMAGENTA$@forecast_sentence}
{$BOLD$}Temperature{$RESET$}: {@weather.temperature|° F|° C} with a low of {@weather.min_temp|° F|° C}, and a high of {@weather.max_temp|° F|° C}, feels like {@weather.feels_like|° F|° C}
{$BOLD$}Wind{$RESET$}: {@weather.wind.speed| mph| km/h} at {@weather.wind.heading|°}
{$BOLD$}Cloud Cover{$RESET$}: {@weather.cloud_cover|%}
{$BOLD$}Dew Point{$RESET$}: {@weather.dewpoint|° F|° C}
{$BOLD$}AQI{$RESET$}: {#color_aqi|@weather.aqi}".to_string()
}
