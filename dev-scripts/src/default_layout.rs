pub fn get_default_layout() -> String {
    let s = "VERSION = 20
------------------------------
Weather for {@region}, {@country}
{$FORE_LIGHTMAGENTA$@current_weather.condition_sentence}
{$FORE_LIGHTMAGENTA$@forecast_sentence}
Temperature: {@current_weather.temperature|° F|° C} with a low of {@current_weather.min_temp|° F|° C}, and a high of {@current_weather.max_temp|° F|° C}, feels like {@current_weather.feels_like|° F|° C}
Wind: {@current_weather.wind.speed| mph| km/h} at {@current_weather.wind.heading|°}
Cloud Cover: {@current_weather.cloud_cover|%}
Dew Point: {@current_weather.dewpoint|° F|° C}
AQI: {#color_aqi|@current_weather.aqi}".to_string();
    return s;
}
