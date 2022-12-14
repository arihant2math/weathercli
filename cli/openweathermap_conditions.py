import weather_codes


class OpenWeatherMapConditions:
    def __init__(self, data):
        self.id: int = data["id"]
        self.name = data["main"]
        self.description = data["description"]
        self.icon = data["icon"]
        self.sentence = self.get_sentence()

    def get_sentence(self):
        reader = weather_codes.data
        for row in reader:
            if str(row[0]) == str(self.id):
                return row[4]
        return "Unknown Conditions, condition id=" + str(self.id)

