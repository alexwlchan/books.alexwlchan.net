#!/usr/bin/env python3

import datetime
import pathlib
import subprocess


if __name__ == "__main__":
    title = input("What's the title of the book? ")
    slug = input("What's the slug of the MD file? ")
    author = input("Who wrote the book? ")
    publication_year = input("When was it published? ")

    today = datetime.date.today()
    year = str(today.year)

    out_path = pathlib.Path("src") / year / (slug + ".md")

    with open(out_path, "w") as out_file:
        out_file.write(
            f"""---
layout: review
book:
  contributors:
    - name: {author}
  cover:
    tint_color: "???"
  title: "{title}"
  publication_year: {publication_year}
  tags:
    - ???
  isbn13: "???"
review:
  date_read: ???
  format: ???
  rating: ???
---
"""
        )

    subprocess.check_call(["open", str(out_path)])
