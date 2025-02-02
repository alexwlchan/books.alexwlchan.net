#!/usr/bin/env python3
"""
Convert an image to a new image at a given size.

This takes one or more arguments, each of which must be a JSON-formatted
object with four keys: ``in_path``, ``out_path``, ``max_width``, ``max_height``.

Example:

    $ python3 convert_image.py '{"in_path": "cat.jpg",
                                 "out_path": "cat_500w.webp",
                                 "max_width": 500,
                                 "max_height": 300}'

"""

import io
import json
import os
import sys

from PIL import Image
from PIL import ImageCms
import pytest


def get_profile_description(im):
    icc_profile = im.info.get("icc_profile")

    if icc_profile is None:
        return None

    f = io.BytesIO(icc_profile)
    prf = ImageCms.ImageCmsProfile(f)
    return prf.profile.profile_description


def clamp_size(
    im_width: int, im_height: int, max_width: int, max_height: int
) -> tuple[int, int]:
    new_width = max_width
    new_height = int(im_height * max_width / im_width)

    if new_width <= max_width and new_height <= max_height:
        return (new_width, new_height)

    if new_height > max_height:
        new_height = max_height
        new_width = int(im_width * max_height / im_height)

        return (new_width, new_height)


@pytest.mark.parametrize(
    ["im_width", "im_height", "max_width", "max_height", "expected"],
    [(300, 475, 360, 480, (303, 480))],
)
def test_clamp_size(im_width, im_height, max_width, max_height, expected):
    assert clamp_size(im_width, im_height, max_width, max_height) == expected


if __name__ == "__main__":
    request = json.loads(sys.argv[1])

    im = Image.open(request["in_path"])

    profile_name = get_profile_description(im)
    if profile_name is not None and profile_name not in {
        "sRGB",
        "sRGB built-in",
        "sRGB IEC61966-2.1",
        "Generic Gray Gamma 2.2 Profile",
        "Adobe RGB (1998)",
    }:
        raise ValueError(
            f"Got image with non-sRGB profile: {request['in_path']} ({profile_name!r})"
        )

    new_width, new_height = clamp_size(
        im_width=im.width,
        im_height=im.height,
        max_width=request["max_width"],
        max_height=request["max_height"],
    )

    assert new_width <= request["max_width"], (new_width, request["max_width"])
    assert new_height <= request["max_height"], (new_height, request["max_height"])

    im = im.resize((new_width, new_height))

    os.makedirs(os.path.dirname(request["out_path"]), exist_ok=True)

    with open(request["out_path"], "xb") as fp:
        im.save(fp)

    print(request["out_path"])
