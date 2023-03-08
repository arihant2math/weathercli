import json

import colorama
from core import WeatherFile

from cli.layout import default_layout
from cli.layout.layout_row import LayoutRow


class Layout:
    version = 1

    def __init__(self, file=None, text=None):
        if file is not None:
            f = WeatherFile("layouts/" + file)
            layout = json.loads(f.data)
        elif text is not None:
            if type(text) == dict:
                layout = text
            else:
                layout = json.loads(text)
        else:
            layout = default_layout.layout
        if "version" not in layout:
            print("Invalid Layout, defaulting")
            layout = default_layout.layout
        else:
            if layout["version"] > self.version:
                print(
                    "Version of layout file, "
                    + str(layout["version"])
                    + ", is greater than the highest supported version "
                    + str(self.version)
                )
            elif layout["version"] < 1:
                print("Layout Version too old (version 0 is not supported)")
                exit(0)
        if "defaults" in layout:
            global_settings = layout["defaults"]
        else:
            global_settings = {}
        if "layout" not in layout:
            print("Invalid Layout, defaulting")
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
