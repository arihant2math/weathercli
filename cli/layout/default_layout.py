layout = {
    "layout": [
        [
            {"type": "text", "data": {"text": "Weather for "}},
            {"type": "variable", "data": {"name": "region"}},
            {"type": "text", "data": {"text": ", "}},
            {"type": "variable", "data": {"name": "country"}},
        ],
        [
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.condition_sentence",
                    "color": "LIGHTMAGENTA_EX",
                },
            }
        ],
        [
            {
                "type": "variable",
                "data": {"name": "forecast_sentence", "color": "LIGHTMAGENTA_EX"},
            }
        ],
        [
            {"type": "text", "data": {"text": "Temperature: "}},
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.temperature",
                    "metric": "° C",
                    "imperial": "° F",
                },
            },
            {"type": "text", "data": {"text": " with a low of "}},
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.min_temp",
                    "metric": "° C",
                    "imperial": "° F",
                },
            },
            {"type": "text", "data": {"text": " and a high of "}},
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.max_temp",
                    "metric": "° C",
                    "imperial": "° F",
                },
            },
            {"type": "text", "data": {"text": ", feels like "}},
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.feels_like",
                    "metric": "° C",
                    "imperial": "° F",
                },
            },
        ],
        [
            {"type": "text", "data": {"text": "Wind: "}},
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.wind.speed",
                    "metric": " km/h",
                    "imperial": " mph",
                },
            },
            {"type": "text", "data": {"text": " at "}},
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.wind.heading",
                    "metric": "°",
                    "imperial": "°",
                },
            },
        ],
        [
            {"type": "text", "data": {"text": "Cloud Cover: "}},
            {
                "type": "variable",
                "data": {
                    "name": "current_weather.cloud_cover",
                    "metric": "%",
                    "imperial": "%",
                },
            },
        ],
        [
            {"type": "text", "data": {"text": "Dew Point: "}},
            {"type": "variable", "data": {"name": "current_weather.dewpoint"}},
        ],
        [
            {"type": "text", "data": {"text": "AQI: "}},
            {
                "type": "variable",
                "data": {"name": "current_weather.aqi"},
            },
        ],
    ],
}
