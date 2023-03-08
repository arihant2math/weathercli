import colorama


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
        return first*second

    @staticmethod
    def divide(first, second):
        return first/second

    @staticmethod
    def divide_i(first, second):
        return first//second

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
    def color_aqi(aqi):
        aqi = str(aqi)
        if not aqi.isdigit():
            return aqi
        if int(aqi) < 3:
            return colorama.Fore.LIGHTGREEN_EX + aqi + colorama.Fore.RESET
        elif int(aqi) < 5:
            return colorama.Fore.LIGHTYELLOW_EX + aqi + colorama.Fore.RESET
        else:
            return colorama.Fore.RED + aqi + colorama.Fore.RESET

    @staticmethod
    def round(number, digits):
        return round(number, digits)

    @staticmethod
    def replace(string, target, substitution):
        return string.replace(target, substitution)

    @staticmethod
    def format_string(string: str, **kwargs):
        return string.format(**kwargs)
