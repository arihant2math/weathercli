import os
from urllib.parse import urlparse

import colorama
import requests

from cli.layout.image_to_text import image_to_text
from cli.layout.util import LayoutException


def parse_string(data: str):
    if data[0] == "@":
        data = data[1: len(data)]
        split = data.split("|")
        imperial = None
        metric = None
        if len(split) == 2:
            metric = imperial = split[1]
        elif len(split) == 3:
            imperial = split[1]
            metric = split[2]
        out = {"type": "variable", "value": split[0]}
        if imperial is not None:
            out["imperial"] = imperial
        if metric is not None:
            out["imperial"] = imperial
        return LayoutItem(out)
    if data[0] == "#":
        data = data[1: len(data)]
        split = data.split("|")
        out = {"type": "function", "value": split.pop(0)}
        args = []
        kwargs: dict = {}
        for item in split:
            if "=" not in item:
                args.append(item)
            else:
                s = item.split("=")
                kwargs[s[0]] = kwargs[s[1]]
        out["args"] = args
        out["kwargs"] = kwargs
        return LayoutItem(out)
    if "\\" == data[0]:
        data = data[1: len(data)]
    return LayoutItem({"type": "text", "value": data})


def to_layout_item(data):
    if type(data) in [str]:
        return parse_string(data)
    elif type(data) in [int, float]:
        return LayoutItem({"type": "text", "value": str(data)})
    elif type(data) != dict:
        raise LayoutException(
            "Type of item is {} not dict, str, int, or float".format(type(data))
        )
    return LayoutItem(data)


def uri_validator(x):
    try:
        result = urlparse(x)
        return all([result.scheme, result.netloc])
    except:
        return False


class LayoutItem:
    def __init__(self, data):
        self.item_data = data
        if "color" in data:
            self.color = getattr(colorama.Fore, data["color"])
        else:
            self.color = ""
        if "bgcolor" in data:
            self.bgcolor = getattr(colorama.Back, data["bgcolor"])
        else:
            self.bgcolor = ""
        if "metric" in data:
            self.metric_unit = data["metric"]
        else:
            self.metric_unit = ""
        if "imperial" in data:
            self.imperial_unit = data["imperial"]
        else:
            self.imperial_unit = ""
        if "unit_color" in data:
            self.unit_color = getattr(colorama.Fore, data["unit_color"])
        else:
            self.unit_color = ""
        if "value" not in data:
            raise LayoutException("Key 'value' not found")
        self.value = data["value"]

        if "type" not in data:
            raise LayoutException("Key 'type' not found")

        if data["type"] == "text":
            self.item_type = 0
        elif data["type"] == "variable":
            self.item_type = 1
        elif data["type"] == "function":
            self.item_type = 2
            self.args_items = []
            self.kwargs_items = {}
            if "args" in data:
                self.args = data["args"]
                for item in self.args:
                    self.args_items.append(to_layout_item(item))
            if "kwargs" in data:
                self.kwargs = data["kwargs"]
                for item in self.kwargs:
                    self.kwargs_items[item["arg_name"]] = to_layout_item(item)
        elif data["type"] == "image":
            self.item_type = 3

    def get_variable_value(self, data):
        split = self.value.split(".")
        current = data
        while len(split) != 0:
            if split[0][0] == "[":  # list item
                current = current[int(split[0][1: len(split[0]) - 1])]
            else:  # normal variable
                if current is not None:
                    current = getattr(current, split[0])
                else:
                    return None
            split.pop(0)
        return current

    def get_function_value(self, **kwargs):
        func = getattr(kwargs["function_class"], self.value)
        args_proper = []
        for i in self.args_items:
            if i.item_type == 0:
                args_proper.append(i.get_value(**kwargs))
            elif i.item_type == 1:
                args_proper.append(i.get_value(**kwargs))
            elif i.item_type == 2:
                args_proper.append(i.get_value(**kwargs))
        kwargs_proper = {}
        for i in self.kwargs_items:
            if self.kwargs_items[i].item_type == 0:
                kwargs_proper[i] = i.get_value("")
            elif self.kwargs_items[i].item_type == 1:
                kwargs_proper[i] = i.get_value(kwargs["data"], False, "", "")
            elif self.kwargs_items[i].item_type == 2:
                kwargs_proper[i] = i.get_value(kwargs["data"], kwargs["function_class"])
        args_t = tuple(args_proper)
        return func(*args_t, **kwargs_proper)

    def get_value(self, **kwargs):
        if self.item_type == 1:
            return self.get_variable_value(kwargs["data"])
        elif self.item_type == 2:
            return self.get_function_value(**kwargs)
        return self.value

    def to_string(self, **kwargs):
        if self.item_type == 0:
            return kwargs["text_color"] + kwargs["text_bg_color"] + self.color + self.bgcolor + self.value
        elif self.item_type == 1:
            try:
                value = self.get_variable_value(kwargs["data"])
            except AttributeError as e:
                raise LayoutException(
                    "Could not get variable value {} in value string {}".format(
                        e.name, self.value
                    )
                ) from e
            if kwargs["metric"]:
                return (
                        kwargs["variable_color"]
                        + self.color
                        + self.bgcolor
                        + str(value)
                        + kwargs["unit_color"]
                        + self.unit_color
                        + self.metric_unit
                )
            else:
                return (
                        kwargs["variable_color"]
                        + self.color
                        + self.bgcolor
                        + str(value)
                        + kwargs["unit_color"]
                        + self.unit_color
                        + self.imperial_unit
                )
        elif self.item_type == 2:
            try:
                value = self.get_function_value(**kwargs)
            except AttributeError as e:
                raise LayoutException(
                    "Could not get function value {} in value string {}".format(
                        e.name, self.value
                    )
                ) from e
            return self.color + self.bgcolor + value

        elif self.item_type == 3:
            source = to_layout_item(self.value).get_value(data=kwargs["data"])
            is_uri = uri_validator(source)
            if is_uri:
                response = requests.get(source)
                f = open("temp.img", "bw")
                f.write(response.content)
                f.close()
            try:
                data = image_to_text("temp.img", self.item_data["scale"])
            except Exception:
                pass
            os.remove("temp.img")
            return data
