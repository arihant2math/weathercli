from cli.layout import temp_constants


class Util:
    @staticmethod
    def add(first, second):
        return first + second

    @staticmethod
    def subtract(first, second):
        return first - second

    @staticmethod
    def negate(first):
        return -first

    @staticmethod
    def multiply(first, second):
        return first * second

    @staticmethod
    def divide(first, second):
        return first / second

    @staticmethod
    def divide_i(first, second):
        return first // second

    @staticmethod
    def rainbow(text):
        return text

    @staticmethod
    def to_list(super_list, attribute):
        return [getattr(item, attribute) for item in super_list]

    @staticmethod
    def pretty_list(li, delimiter=","):
        return delimiter.join(li)

    @staticmethod
    def round(number, digits):
        return round(number, digits)

    @staticmethod
    def replace(string, target, substitution):
        return string.replace(target, substitution)

    @staticmethod
    def format_string(string: str, *args, **kwargs):
        return string.format(*args, **kwargs)

    @staticmethod
    def to_ascii(string: str):
        return "\n".join(temp_constants.WEATHER_SYMBOL_WEGO[string])


class LayoutException(Exception):
    def __init__(self, message="", row=None, item=None):
        self.message = ""
        if row is not None:
            self.message += "Error in row {}, ".format(row)
            if item is not None:
                self.message += "item {}. ".format(item)
        self.message += message
        self.row = row
        self.item = item
        super().__init__(self.message)
