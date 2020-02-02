#!/usr/bin/env python

import os
import pathlib
import subprocess
import sys

import attr
import frontmatter
import markdown
from jinja2 import Environment, FileSystemLoader, select_autoescape


def rsync(dir1, dir2):
    subprocess.check_call(["rsync", "--recursive", "--delete", dir1, dir2])


@attr.s
class Book:
    title = attr.ib()
    author = attr.ib()
    publication_year = attr.ib()
    cover_image = attr.ib()
    cover_desc = attr.ib()

    isbn_13 = attr.ib(default="")


@attr.s
class Review:
    date_read = attr.ib()
    rating = attr.ib()
    text = attr.ib()


@attr.s
class ReviewEntry:
    path = attr.ib()
    book = attr.ib()
    review = attr.ib()


def get_review_entry_from_path(path):
    post = frontmatter.load(path)

    book = Book(**post["book"])
    review = Review(**post["review"], text=post.content)

    return ReviewEntry(path=path, book=book, review=review)


def get_reviews():
    for dirpath, _, filenames in os.walk("src/reviews"):
        for f in filenames:
            if not f.endswith(".md"):
                continue

            path = pathlib.Path(dirpath) / f

            try:
                yield get_review_entry_from_path(path)
            except Exception:
                print(f"Error parsing {path}", file=sys.stderr)
                raise


def render_markdown(text):
    return markdown.markdown(text)


if __name__ == "__main__":
    all_reviews = list(get_reviews())
    all_reviews = sorted(
        all_reviews, key=lambda rev: rev.review.date_read, reverse=True
    )

    env = Environment(
        loader=FileSystemLoader("templates"),
        autoescape=select_autoescape(["html", "xml"]),
    )

    env.filters["render_markdown"] = render_markdown

    rsync("src/covers/", "_html/covers/")
    rsync("src/static/", "_html/static/")

    for review_entry in all_reviews:
        template = env.get_template("review.html")
        html = template.render(review_entry=review_entry)

        out_name = review_entry.path.relative_to("src").with_suffix(".html")
        out_path = pathlib.Path("_html") / out_name
        out_path.parent.mkdir(exist_ok=True, parents=True)
        out_path.write_text(html)
