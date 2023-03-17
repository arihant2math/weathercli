from PIL import Image

ESCAPE = "\033["


def rgb(red: int, green: int, blue: int):
    return ESCAPE + "38;2;" + str(red) + ";" + str(green) + ";" + str(blue) + "m"


def image_to_text(path: str, scale: float, super_scale=False):
    img = Image.open(path)
    image = img.resize(
        (int(img.height * scale), int(scale * img.width // 3)), Image.NEAREST
    )
    out = ""
    for x in range(0, image.height):
        for y in range(0, image.width):
            colors = image.getpixel((y, x))
            out += rgb(colors[0], colors[1], colors[2])
            if len(colors) > 3:
                if colors[3] > 0.7:
                    out += "█"
                elif colors[3] > 0.2:
                    out += "#"
                else:
                    out += "."
            else:
                out += "█"
        out += "\n"
    return out
