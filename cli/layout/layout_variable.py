import colorama

from cli.layout.layout_item import LayoutItem


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

    def get_variable_value(self, data):
        split = self.name.split(".")
        current = data
        while len(split) != 0:
            if split[0][0] == "[":  # list item
                current = current[int(split[0][1 : len(split[0]) - 1])]
            else:  # normal variable
                current = getattr(current, split[0])
            split.pop(0)
        return current

    def to_string(self, data, metric, variable_color, unit_color):
        value = self.get_variable_value(data)
        if metric:
            return (
                variable_color
                + self.color
                + str(value)
                + unit_color
                + self.unit_color
                + self.metric_unit
                + colorama.Fore.RESET
            )
        else:
            return (
                variable_color
                + self.color
                + str(value)
                + unit_color
                + self.unit_color
                + self.imperial_unit
                + colorama.Fore.RESET
            )
