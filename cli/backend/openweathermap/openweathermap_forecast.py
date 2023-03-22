import weather_core

from cli.backend.openweathermap.openweathermap_current import OpenWeatherMapCurrent
from cli.backend.openweathermap.openweathermap_future import OpenWeatherMapFuture
from cli.backend.weather_forecast import WeatherForecast


class OpenWeatherMapForecast(WeatherForecast):
    def __init__(self, coordinates, metric, settings):
        if (
            settings.open_weather_map_api_key is not None
            and settings.open_weather_map_api_key != ""
        ):
            data = weather_core.backend.open_weather_map_get_combined_data_formatted(
                "https://api.openweathermap.org/data/2.5/",
                settings.open_weather_map_api_key,
                coordinates,
                metric,
            )
        else:
            print("No open weather map api key")
            exit(1)
        forecast = [OpenWeatherMapCurrent(data)]
        for t in data.forecast.list:
            forecast.append(OpenWeatherMapFuture(t))
        super().__init__(
            0, data.weather.name, data.weather.sys.country, forecast, "", data
        )  # TODO: Add Proper status
        self.forecast_sentence = self.get_forecast_sentence()

    def get_forecast_sentence(self):
        data = self.forecast.copy()
        rain = []
        snow = []
        for period in data:
            if period.conditions[0].condition_id // 100 == 5:
                rain.append(True)
                snow.append(False)
            elif period.conditions[0].condition_id // 100 == 6:
                snow.append(True)
                rain.append(False)
            else:
                rain.append(False)
                snow.append(False)
        if data[0].conditions[0].condition_id // 100 == 5:
            t = 0
            for i in rain:
                if not i:
                    break
                t += 1
            return "It will continue raining for " + str(t * 3) + " hours."
        elif data[0].conditions[0].condition_id // 100 == 6:
            t = 0
            for i in snow:
                if not i:
                    break
                t += 1
            return "It will continue snowing for " + str(t * 3) + " hours."
        else:
            combined = zip(rain, snow)
            t = 0
            for period in combined:
                t += 1
                if period[0]:
                    return "It will rain in " + str(t * 3) + " hours"
                elif period[1]:
                    return "It will snow in " + str(t * 3) + " hours"
        return "Conditions are predicted to be clear for the next 3 days."
