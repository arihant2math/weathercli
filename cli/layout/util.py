import colorama


class Util:
    def rainbow(self, text):
        return text

    def to_list(self, super_list, attribute):
        return [getattr(item, attribute) for item in super_list]

    def pretty_list(self, li, delimiter=","):
        return delimiter.join(li)

    def color_aqi(self, aqi):
        if aqi < 3:
            return colorama.Fore.LIGHTGREEN_EX + aqi + colorama.Fore.RESET
        elif aqi < 5:
            return colorama.Fore.LIGHTYELLOW_EX + aqi + colorama.Fore.RESET
        else:
            return colorama.Fore.RED + aqi + colorama.Fore.RESET
