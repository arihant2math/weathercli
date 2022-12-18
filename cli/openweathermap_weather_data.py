from cli.openweathermap_conditions import OpenWeatherMapConditions
from cli.weather_data import WeatherData
from core import WindData


class OpenWeatherMapWeatherData(WeatherData):
    def __init__(self, data: dict):
        super().__init__(
            data["main"]["temp"],
            data["main"]["temp_min"],
            data["main"]["temp_max"],
            data["name"],
            WindData(data["wind"]["speed"], data["wind"]["deg"]),
            data,
        )
        if "cod" in data:
            self.status: int = data["cod"]
        else:
            self.status: int = 200
        self.aqi: int = data["air_quality"]["main"]["aqi"]
        self.country: str = data["sys"]["country"]
        self.cloud_cover: int = data["clouds"]["all"]
        self.conditions: list[OpenWeatherMapConditions] = []
        for condition in data["weather"]:
            self.conditions.append(OpenWeatherMapConditions(condition))
        self.condition_ids: list[int] = self.get_condition_ids()
        self.condition_sentence: str = self.get_condition_sentence()
        self.forecast_sentence: str = self.get_forecast_sentence()

    def get_condition_ids(self):
        ids = []
        for condition in self.conditions:
            ids.append(condition.id)
        return ids

    def get_condition_sentence(self):
        data = self.conditions.copy()
        condition_match = data[0].sentence
        out = condition_match
        data.pop(0)
        for condition in data:
            out += ". Also, "
            condition_match = condition.sentence
            out += condition_match.lower()
        out += "."
        return out

    def get_forecast_sentence(self):
        data = self.raw_data["forecast"]
        rain = []
        snow = []
        for period in data:
            if period["weather"][0]["main"] == "Rain":
                rain.append(True)
            elif period["weather"][0]["main"] == "Snow":
                snow.append(True)
            # if period['weather'][1]['main'] == "Rain":
            #     rain.append(True)
            # elif period['weather'][1]['main'] == "Snow":
            #     snow.append(True)
        if self.conditions[0].name == "Rain":
            t = 0
            for i in rain:
                if not i:
                    break
                t += 1
            return "It will continue raining for " + str(t * 3) + " hours."
        elif self.conditions[0].name == "Snow":
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
