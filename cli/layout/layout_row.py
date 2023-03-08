from cli.layout.layout_item import to_layout_item
from cli.layout.util import Util


class LayoutRow:
    def __init__(self, row_data):
        self.items = []
        for item in row_data:
            self.items.append(to_layout_item(item))

    def to_string(self, data, variable_color, text_color, unit_color, metric):
        s = ""
        kwargs = {
            "text_color": text_color,
            "variable_color": variable_color,
            "unit_color": unit_color,
            "metric": metric,
            "data": data,
            "function_class": Util,
        }
        for i in self.items:
            s += i.to_string(**kwargs)
        return s
