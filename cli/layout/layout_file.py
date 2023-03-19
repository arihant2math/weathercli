import json
from json import JSONDecodeError
from logging import Logger
from typing import Optional, Union

import colorama
from core import WeatherFile

from cli.layout import default_layout
from cli.layout.layout_row import LayoutRow
from cli.layout.util import LayoutException


def parse_row_string(row_string: str) -> LayoutRow:
    item_list = []
    previous_char = ""
    current = ""
    for c in row_string:
        if c == "{" and previous_char != "\\":
            item_list.append(current)
            current = ""
            previous_char = ""
        elif c == "}" and previous_char != "\\":
            item_list.append(current)
            current = ""
            previous_char = ""
        else:
            current += c
            previous_char = c
    if current != "":
        item_list.append(current)
    return LayoutRow(item_list)


class LayoutFile:
    version = 3

    def __init__(
        self,
        file: Optional[str] = None,
        text: Optional[Union[str, dict]] = None,
        logger: Logger = None,
    ):
        if file is not None:
            f = WeatherFile("layouts/" + file)
            try:
                layout = json.loads(f.data)
            except JSONDecodeError as e:
                print("Invalid Layout, JSON parsing failed, defaulting")
                logger.critical(
                    "Invalid Layout, JSON parsing failed, defaulting, error=" + e.msg
                )
                layout = default_layout.layout
        elif text is not None:
            if type(text) == dict:
                layout = text
            else:
                layout = json.loads(text)
        else:
            layout = default_layout.layout
        self.layout = layout
        if "version" not in layout:
            print("Invalid Layout, missing key 'version', defaulting")
            logger.critical(
                "Invalid Layout, missing Key 'version', add it like this {\n\t... // Your json "
                'here\n\t"version": ' + str(self.version) + "\n}"
            )
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
                print("Layout Version too old (version 0 is not supported), defaulting")
                layout = default_layout.layout
        if "defaults" in layout:
            global_settings = layout["defaults"]
        else:
            global_settings = {}
        if "layout" not in layout:
            print("Invalid Layout, missing key 'layout', defaulting")
            layout = default_layout.layout
        if type(layout["layout"]) != list:
            print("Invalid Layout, type of key 'layout' is not 'list', defaulting")
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
        if "text_bg_color" not in global_settings:
            self.text_bg_color = colorama.Back.RESET
        else:
            self.text_bg_color = getattr(colorama.Back, layout["text_bg_color"])
        self.layout = layout["layout"]
        self._internal_layout = []
        for count, row in enumerate(self.layout):
            try:
                if type(row) == list:
                    self._internal_layout.append(LayoutRow(row))
                elif type(row) == str:
                    self._internal_layout.append(parse_row_string(row))
                else:
                    raise LayoutException("Type of row is not a list or string")
            except LayoutException as e:
                raise LayoutException(e.message, count, e.item)

    def to_string(self, data, metric: bool) -> str:
        s = []
        for count, row in enumerate(self._internal_layout):
            try:
                s.append(
                    row.to_string(
                        data,
                        self.variable_color,
                        self.text_color,
                        self.text_bg_color,
                        self.unit_color,
                        metric,
                    )
                )
            except LayoutException as e:
                raise LayoutException(e.message, count, e.item)
        return "\n".join(s)

    def compile(self):
        original = self.layout
        new_rows = [r.get_source() for r in self._internal_layout]
        original["layout"] = new_rows
        return original
