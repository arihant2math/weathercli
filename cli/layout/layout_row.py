from cli.layout.layout_function import LayoutFunction
from cli.layout.layout_text import LayoutText
from cli.layout.layout_variable import LayoutVariable
from cli.layout.util import Util


class LayoutRow:
    def __init__(self, row_data):
        self.items = []
        for item in row_data:
            if item["type"] == "text":
                self.items.append(LayoutText(item["data"]))
            elif item["type"] == "variable":
                self.items.append(LayoutVariable(item["data"]))
            elif item["type"] == "function":
                self.items.append(LayoutFunction(item["data"]))

    def to_string(self, data, variable_color, text_color, unit_color, metric):
        s = ""
        for i in self.items:
            if type(i) == LayoutText:
                s += i.to_string(text_color)
            elif type(i) == LayoutVariable:
                s += i.to_string(data, metric, variable_color, unit_color)
            elif type(i) == LayoutFunction:
                s += i.to_string(data, Util)
        return s
