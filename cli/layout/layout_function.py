import colorama

from cli.layout.layout_item import LayoutItem
from cli.layout.layout_text import LayoutText
from cli.layout.layout_variable import LayoutVariable
from cli.layout.util import Util


class LayoutFunction(LayoutItem):
    def __init__(self, item_data):
        self.name = item_data["name"]
        self.args_items = []
        self.kwargs_items = {}
        if "args" in item_data:
            self.args = item_data["args"]
            for item in self.args:
                if item["type"] == "text":
                    self.args_items.append(LayoutText(item["data"]))
                elif item["type"] == "variable":
                    self.args_items.append(LayoutVariable(item["data"]))
                elif item["type"] == "function":
                    self.args_items.append(LayoutFunction(item["data"]))
        if "kwargs" in item_data:
            self.kwargs = item_data["kwargs"]
            for item in self.kwargs:
                if item["type"] == "text":
                    self.kwargs_items[item["arg_name"]] = LayoutText(item["data"])
                elif item["type"] == "variable":
                    self.kwargs_items[item["arg_name"]] = LayoutVariable(item["data"])
                elif item["type"] == "function":
                    self.kwargs_items[item["arg_name"]] = LayoutFunction(item["data"])
        if "color" in item_data:
            self.color = getattr(colorama.Fore, item_data["color"])
        else:
            self.color = ""

    def to_string(self, data, function_class):
        func = getattr(function_class, self.name)
        args_proper = []
        for i in self.args_items:
            if type(i) == LayoutText:
                args_proper.append(i.to_string(""))
            elif type(i) == LayoutVariable:
                args_proper.append(i.to_string(data, False, "", ""))
            elif type(i) == LayoutFunction:
                args_proper.append(i.to_string(data, Util))
        kwargs_proper = {}
        for i in self.kwargs_items:
            if type(self.kwargs_items[i]) == LayoutText:
                kwargs_proper[i] = i.to_string("")
            elif type(self.kwargs_items[i]) == LayoutVariable:
                kwargs_proper[i] = i.to_string(data, False, "", "")
            elif type(self.kwargs_items[i]) == LayoutFunction:
                kwargs_proper[i] = i.to_string(data, Util)
        args_t = tuple(args_proper)
        return self.color + func(*args_t, **kwargs_proper)
