import colorama


class Util:
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
        if aqi < 3:
            return colorama.Fore.LIGHTGREEN_EX + aqi + colorama.Fore.RESET
        elif aqi < 5:
            return colorama.Fore.LIGHTYELLOW_EX + aqi + colorama.Fore.RESET
        else:
            return colorama.Fore.RED + aqi + colorama.Fore.RESET
