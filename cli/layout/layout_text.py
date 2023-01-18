import colorama

from cli.layout.layout_item import LayoutItem


class LayoutText(LayoutItem):
    def __init__(self, item_data):
        self.text = item_data["text"]
        if "color" in item_data:
            self.color = getattr(colorama.Fore, item_data["color"])
        else:
            self.color = ""

    def to_string(self, color):
        return color + self.color + self.text + colorama.Fore.RESET
