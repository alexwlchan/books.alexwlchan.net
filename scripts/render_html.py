#!/usr/bin/env python

import datetime
import os
import pathlib
import re
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
    cover_desc = attr.ib(default="")

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

    def out_path(self):
        return self.path.relative_to("src").with_suffix("")


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


def render_date(date_value):
    if isinstance(date_value, datetime.date):
        return date_value.strftime("%d %B %Y")

    date_match = re.match(
        r"^(?P<year>\d{4})-(?P<month>\d{2})(?:-(?P<day>\d{2}))?$", date_value
    )
    assert date_match is not None, date_value

    date_obj = datetime.datetime(
        year=int(date_match.group("year")),
        month=int(date_match.group("month")),
        day=int(date_match.group("day") or "1"),
    )

    if date_match.group("day"):
        return date_obj.strftime("%d %B %Y")
    else:
        return date_obj.strftime("%B %Y")


def render_individual_review(env, *, review_entry):
    template = env.get_template("review.html")
    html = template.render(review_entry=review_entry)

    out_name = review_entry.out_path() / "index.html"
    out_path = pathlib.Path("_html") / out_name
    out_path.parent.mkdir(exist_ok=True, parents=True)
    out_path.write_text(html)


if __name__ == "__main__":
    all_reviews = list(get_reviews())
    all_reviews = sorted(
        all_reviews, key=lambda rev: str(rev.review.date_read), reverse=True
    )

    env = Environment(
        loader=FileSystemLoader("templates"),
        autoescape=select_autoescape(["html", "xml"]),
    )

    env.filters["render_markdown"] = render_markdown
    env.filters["render_date"] = render_date

    rsync("src/covers/", "_html/covers/")
    rsync("src/static/", "_html/static/")

    for review_entry in all_reviews:
        render_individual_review(env, review_entry=review_entry)

    template = env.get_template("list_reviews.html")
    html = template.render(all_reviews=all_reviews)

    out_path = pathlib.Path("_html") / "reviews/index.html"
    out_path.write_text(html)
