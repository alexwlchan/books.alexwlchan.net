import colorsys
import json
import os
import subprocess

import wcag_contrast_ratio as contrast


def choose_tint_color_from_dominant_colors(dominant_colors, background_color):
    """
    Given a set of dominant colors (say, from a k-means algorithm) and the
    background against which they'll be displayed, choose a tint color.

    Both ``dominant_colors`` and ``background_color`` should be tuples in [0,1].
    """
    # Clamp colours to the range 0.0 - 1.0; occasionally sklearn has spat out
    # numbers outside this range.
    dominant_colors = [
        (min(max(col[0], 0), 1), min(max(col[1], 0), 1), min(max(col[2], 0), 1))
        for col in dominant_colors
    ]

    # The minimum contrast ratio for text and background to meet WCAG AA
    # is 4.5:1, so discard any dominant colours with a lower contrast.
    sufficient_contrast_colors = [
        col for col in dominant_colors if contrast.rgb(col, background_color) >= 4.5
    ]

    # If none of the dominant colours meet WCAG AA with the background,
    # try again with black and white -- every colour in the RGB space
    # has a contrast ratio of 4.5:1 with at least one of these, so we'll
    # get a tint colour, even if it's not a good one.
    #
    # Note: you could modify the dominant colours until one of them
    # has sufficient contrast, but that's omitted here because it adds
    # a lot of complexity for a relatively unusual case.
    if not sufficient_contrast_colors:
        return choose_tint_color_from_dominant_colors(
            dominant_colors=dominant_colors + [(0, 0, 0), (1, 1, 1)],
            background_color=background_color,
        )

    # Of the colors with sufficient contrast, pick the one with the
    # highest saturation.  This is meant to optimise for colors that are
    # more colourful/interesting than simple greys and browns.
    hsv_candidates = {
        tuple(rgb_col): colorsys.rgb_to_hsv(*rgb_col)
        for rgb_col in sufficient_contrast_colors
    }

    return max(hsv_candidates, key=lambda rgb_col: hsv_candidates[rgb_col][2])


def choose_tint_color(path):
    # This shells out to https://github.com/alexwlchan/dominant_colours to find
    # the dominant colours in the book cover.
    tool_output = subprocess.check_output([
        "dominant_colours", path, '--max-colours=12', "--no-palette"
    ])

    dominant_colors = [
        (int(line[1:3], 16) / 255, int(line[3:5], 16) / 255, int(line[5:7], 16) / 255)
        for line in tool_output.splitlines()
    ]

    return choose_tint_color_from_dominant_colors(
        dominant_colors=dominant_colors, background_color=(1, 1, 1)
    )


def get_tint_color_data():
    try:
        return json.load(open(os.path.join("src", "tint_colors.json")))
    except FileNotFoundError:
        return {}


def get_tint_colors():
    return {path: data["color"] for (path, data) in get_tint_color_data().items()}


def store_tint_color(cover_path):
    tint_colors = get_tint_color_data()

    # If the size of a file has changed since the previous run, we need to
    # recompute the tint colour.
    try:
        if os.path.basename(cover_path) in tint_colors:
            return
    except KeyError:
        print(f"Recomputing tint color for {cover_path}")

    cover_color = choose_tint_color(cover_path)
    tint_colors[os.path.basename(cover_path)] = {
        "color": cover_color,
        "size": os.stat(cover_path).st_size,
    }

    with open(os.path.join("src", "tint_colors.json"), "w") as outfile:
        outfile.write(json.dumps(tint_colors, indent=2, sort_keys=True))
