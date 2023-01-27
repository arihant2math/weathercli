import json

import colorama

from cli.layout import default_layout
from cli.layout.layout_row import LayoutRow
from cli.local.weather_file import WeatherFile


class Layout:
    def __init__(self, file=None, text=None):
        if file is not None:
            f = WeatherFile("layouts/" + file)
            layout = f.data
        elif text is not None:
            if type(text) == dict:
                layout = text
            else:
                layout = json.loads(text)
        else:
            layout = default_layout.layout
        if 'defaults' in layout:
            global_settings = layout['defaults']
        else:
            global_settings = {}
        if "layout" not in layout:
            layout = default_layout.layout
        if "variable_color" not in global_settings:
            self.variable_color = colorama.Fore.LIGHTGREEN_EX
        else:
            self.variable_color = getattr(colorama.Fore, layout["variable_color"])
        if "text_color" not in global_settings:
            self.text_color = colorama.Fore.LIGHTBLUE_EX
        else:
            self.text_color = getattr(colorama.Fore, layout["text_color"])
        if "unit_color" not in global_settings:
            self.unit_color = colorama.Fore.MAGENTA
        else:
            self.unit_color = getattr(colorama.Fore, layout["unit_color"])
        self.layout = layout["layout"]
        self._internal_layout = [LayoutRow(row) for row in self.layout]

    def to_string(self, data, metric: bool):
        s = []
        for row in self._internal_layout:
            s.append(
                row.to_string(
                    data, self.variable_color, self.text_color, self.unit_color, metric
                )
            )
        return "\n".join(s)
