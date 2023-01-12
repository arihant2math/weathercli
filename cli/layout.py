import colorama

from cli import WeatherData, default_layout
from cli.local.weather_file import WeatherFile


class LayoutItem:
    pass


class LayoutText(LayoutItem):
    def __init__(self, item_data):
        self.text = item_data["text"]
        if "color" in item_data:
            self.color = getattr(colorama.Fore, item_data["color"])
        else:
            self.color = ""

    def to_string(self, color):
        return color + self.color + self.text + colorama.Fore.RESET


class LayoutVariable(LayoutItem):
    def __init__(self, item_data):
        self.name = item_data["name"]
        if "color" in item_data:
            self.color = getattr(colorama.Fore, item_data["color"])
        else:
            self.color = ""
        if "metric" in item_data:
            self.metric_unit = item_data["metric"]
        else:
            self.metric_unit = ""
        if "imperial" in item_data:
            self.imperial_unit = item_data["imperial"]
        else:
            self.imperial_unit = ""
        if "unit_color" in item_data:
            self.unit_color = getattr(colorama.Fore, item_data["unit_color"])
        else:
            self.unit_color = ""

    def to_string(self, value, metric, unit_color):
        if metric:
            return (
                self.color
                + str(value)
                + unit_color
                + self.unit_color
                + self.metric_unit
                + colorama.Fore.RESET
            )
        else:
            return (
                self.color
                + str(value)
                + unit_color
                + self.unit_color
                + self.imperial_unit
                + colorama.Fore.RESET
            )


class LayoutRow:
    def __init__(self, row_data):
        self.items = []
        self.variables = []
        for item in row_data:
            if item["type"] == "text":
                self.items.append(LayoutText(item["data"]))
            elif item["type"] == "variable":
                v = LayoutVariable(item["data"])
                self.items.append(v)
                self.variables.append(v.name)

    def to_string(self, data, variable_color, text_color, unit_color, metric):
        s = ""
        for i in self.items:
            if type(i) == LayoutText:
                s += text_color + i.to_string(text_color)
            elif type(i) == LayoutVariable:
                split = i.name.split(".")
                if len(split) == 1:
                    s += variable_color + i.to_string(
                        getattr(data, split[0]), metric, unit_color
                    )
                else:
                    current = getattr(data, split[0])
                    split.pop(0)
                    while len(split) != 0:
                        current = getattr(current, split[0])
                        split.pop(0)
                    s += variable_color + i.to_string(current, metric, unit_color)
        return s


class Layout:
    def __init__(self, file="default.json"):
        f = WeatherFile("layouts/" + file)
        layout = f.data
        if "variable_color" not in layout:
            self.variable_color = colorama.Fore.LIGHTGREEN_EX
        else:
            self.variable_color = getattr(colorama.Fore, layout["variable_color"])
        if "text_color" not in layout:
            self.text_color = colorama.Fore.LIGHTBLUE_EX
        else:
            self.text_color = getattr(colorama.Fore, layout["text_color"])
        if "unit_color" not in layout:
            self.unit_color = colorama.Fore.MAGENTA
        else:
            self.unit_color = getattr(colorama.Fore, layout["unit_color"])
        if "layout" not in layout:
            f.data = default_layout.layout
            f.write()
            layout = f.data
        self.layout = layout["layout"]
        self._internal_layout = [LayoutRow(row) for row in self.layout]

    def to_string(self, data: WeatherData, metric: bool):
        s = []
        for row in self._internal_layout:
            s.append(
                row.to_string(
                    data, self.variable_color, self.text_color, self.unit_color, metric
                )
            )
        return "\n".join(s)
