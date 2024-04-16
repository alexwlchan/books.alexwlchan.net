#!/usr/bin/env python3
"""
Convert the social sharing icon for a book cover.
"""

import os
import sys

from PIL import Image


if __name__ == "__main__":
    path = sys.argv[1]
    out_path = sys.argv[2]

    im = Image.open(path)

    icon_im = Image.new("RGB", (480, 480), "white")

    im.thumbnail((480, 480))
    position = ((480 - im.width) // 2, (480 - im.height) // 2)
    icon_im.paste(im, position)

    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    icon_im.save(out_path)
