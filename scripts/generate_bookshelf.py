#!/usr/bin/env python3

import base64
import colorsys
import functools
import io
import os
import random

from PIL import Image, ImageDraw


R = random.Random(0)


def get_bins(total_width, min_height, max_height):
    x_coord = 0

    while x_coord <= total_width:
        width = R.randint(4, 28)
        height = R.randint(min_height, max_height)

        yield [(x_coord, 0), (x_coord + width, height)]
        x_coord += width


@functools.lru_cache()
def get_repeatable_bins(**kwargs):
    """
    Get a set of bins which is always the same.
    """
    return list(get_bins(**kwargs))


def get_tint_colors(tint_color):
    r, g, b = tint_color
    h, s, v = colorsys.rgb_to_hsv(r / 255, g / 255, b / 255)

    v = min(v, 0.45)

    while True:
        new_brightness = R.uniform(max(v * 3 / 4, 0), min(v * 4 / 3, 1))
        yield colorsys.hsv_to_rgb(h, s, new_brightness)


def create_shelf(tint_color):
    # Shelves go from 30px to 45px height, then 2x for retina displays
    bins = get_repeatable_bins(total_width=2000, min_height=60, max_height=90)
    colors = get_tint_colors(tint_color=tint_color)

    im = Image.new("RGBA", size=(2000, 90))

    draw = ImageDraw.Draw(im)

    for bin_xy, bin_color in zip(bins, colors):
        r, g, b = bin_color
        draw.rectangle(bin_xy, (int(r * 255), int(g * 255), int(b * 255)))

    return im


def create_shelf_data_uri(tint_color):
    r, g, b = tint_color

    if [r, g, b] == [0, 0, 0] or r <= 0.02 and g <= 0.02 and b <= 0.02:
        tint_color = [0.2, 0.2, 0.2]
        r, g, b = tint_color

    try:
        f = open(f"_shelves/{r}_{g}_{b}.png", "rb")
    except FileNotFoundError:
        im = create_shelf(tint_color)

        os.makedirs("_shelves", exist_ok=True)

        im.save(f"_shelves/{r}_{g}_{b}.png")
        f = open(f"_shelves/{r}_{g}_{b}.png", "rb")

    b64_string = base64.b64encode(f.read()).decode("utf8")
    return f"data:image/png;base64,{b64_string}"


if __name__ == "__main__":
    im = get_shelf(tint_color=[
        0.23529411764705882,
        0.2627450980392157,
        0.5294117647058824
    ])

    im.save("static/bookshelf_blue.png")
