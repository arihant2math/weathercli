import colorama

from cli.layout.layout_item import LayoutItem


class LayoutFunction(LayoutItem):
    def __init__(self, item_data):
        self.name = item_data["name"]
        if "args" in item_data:
            self.args = item_data["args"]
        if "kwargs" in item_data:
            self.kwargs = item_data["kwargs"]
        if "color" in item_data:
            self.color = getattr(colorama.Fore, item_data["color"])
        else:
            self.color = ""

    def to_string(self, data, function_class):
        func = getattr(function_class, self.name)
        args_proper = []
        for arg in self.args:
            split = arg.split(".")
            current = data
            while len(split) != 0:
                if split[0][0] == "[":  # list item
                    current = current[int(split[0][1 : len(split[0]) - 1])]
                else:  # normal variable
                    current = getattr(current, split[0])
                split.pop(0)
            args_proper.append(current)
        args_t = tuple(args_proper)
        return self.color + func(*args_t)
