#!/usr/bin/env python3

import random

from PIL import Image, ImageDraw


if __name__ == "__main__":
    x_coord = 0
    max_width = 2000
    min_height = 30
    max_height = 45

    im = Image.new("RGBA", size=(max_width, max_height))

    draw = ImageDraw.Draw(im)

    while x_coord <= max_width:
        width = random.randint(5, 25)
        height = random.randint(min_height, max_height)
        grey = random.randint(10, 110)

        draw.rectangle(
            [(x_coord, 0), (x_coord + width, height)], fill=(grey, grey, grey)
        )
        x_coord += width

    im.save("static/bookshelf.png")
