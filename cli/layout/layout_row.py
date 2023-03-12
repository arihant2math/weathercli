from cli.layout.layout_item import to_layout_item
from cli.layout.util import Util, LayoutException


class LayoutRow:
    def __init__(self, row_data):
        if type(row_data) != list:
            raise LayoutException("Type of row_data is not list")
        self.items = []
        for count, item in enumerate(row_data):
            try:
                self.items.append(to_layout_item(item))
            except LayoutException as e:
                raise LayoutException(e.message, item=count + 1)

    def to_string(
        self, data, variable_color, text_color, text_bg_color, unit_color, metric
    ):
        s = ""
        kwargs = {
            "text_color": text_color,
            "text_bg_color": text_bg_color,
            "variable_color": variable_color,
            "unit_color": unit_color,
            "metric": metric,
            "data": data,
            "function_class": Util,
        }
        for count, i in enumerate(self.items):
            try:
                s += i.to_string(**kwargs)
            except LayoutException as e:
                raise LayoutException(e.message, item=count + 1)
        return s
