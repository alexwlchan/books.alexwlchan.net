#!/usr/bin/env python

import datetime
import itertools
import os
import pathlib
import re
import subprocess
import sys

import attr
import frontmatter
import markdown
from markdown.extensions.smarty import SmartyExtension
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

    isbn10 = attr.ib(default="")
    isbn13 = attr.ib(default="")


@attr.s
class Review:
    date_read = attr.ib()
    rating = attr.ib()
    text = attr.ib()
    format = attr.ib()
    did_not_finish = attr.ib(default=False)


@attr.s
class ReviewEntry:
    path = attr.ib()
    book = attr.ib()
    review = attr.ib()

    def out_path(self):
        name = self.path.with_suffix("").name
        return pathlib.Path(f"reviews/{name}")


def get_review_entry_from_path(path):
    post = frontmatter.load(path)

    book = Book(**post["book"])
    review = Review(**post["review"], text=post.content)

    return ReviewEntry(path=path, book=book, review=review)


@attr.s
class CurrentlyReading:
    text = attr.ib()


@attr.s
class CurrentlyReadingEntry:
    path = attr.ib()
    book = attr.ib()
    reading = attr.ib()


def get_reading_entry_from_path(path):
    post = frontmatter.load(path)

    book = Book(**post["book"])
    reading = CurrentlyReading(text=post.content)

    return CurrentlyReadingEntry(path=path, book=book, reading=reading)


@attr.s
class Plan:
    text = attr.ib()


@attr.s
class PlanEntry:
    path = attr.ib()
    book = attr.ib()
    plan = attr.ib()


def get_plan_entry_from_path(path):
    post = frontmatter.load(path)

    book = Book(**post["book"])
    plan = Plan(text=post.content)

    return PlanEntry(path=path, book=book, plan=plan)


def get_entries(dirpath, constructor):
    for dirpath, _, filenames in os.walk(dirpath):
        for f in filenames:
            if not f.endswith(".md"):
                continue

            path = pathlib.Path(dirpath) / f

            try:
                yield constructor(path)
            except Exception:
                print(f"Error parsing {path}", file=sys.stderr)
                raise


def render_markdown(text):
    return markdown.markdown(text, extensions=[SmartyExtension()])


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
        return date_obj.strftime("%-d %B %Y")
    else:
        return date_obj.strftime("%B %Y")


def render_individual_review(env, *, review_entry):
    template = env.get_template("review.html")
    html = template.render(
        review_entry=review_entry, title=f"My review of {review_entry.book.title}"
    )

    out_name = review_entry.out_path() / "index.html"
    out_path = pathlib.Path("_html") / out_name
    out_path.parent.mkdir(exist_ok=True, parents=True)
    out_path.write_text(html)


if __name__ == "__main__":
    env = Environment(
        loader=FileSystemLoader("templates"),
        autoescape=select_autoescape(["html", "xml"]),
    )

    env.filters["render_markdown"] = render_markdown
    env.filters["render_date"] = render_date

    rsync("src/covers/", "_html/covers/")
    rsync("static/", "_html/static/")

    # Render the "all reviews page"

    all_reviews = list(
        get_entries(dirpath="src/reviews", constructor=get_review_entry_from_path)
    )
    all_reviews = sorted(
        all_reviews, key=lambda rev: str(rev.review.date_read), reverse=True
    )

    for review_entry in all_reviews:
        render_individual_review(env, review_entry=review_entry)

    template = env.get_template("list_reviews.html")
    html = template.render(
        all_reviews=[
            (year, list(reviews))
            for (year, reviews) in itertools.groupby(
                all_reviews, key=lambda rev: str(rev.review.date_read)[:4]
            )
        ],
        title="books i’ve read",
        this_year=str(datetime.datetime.now().year),
    )

    out_path = pathlib.Path("_html") / "reviews/index.html"
    out_path.write_text(html)

    # Render the "currently reading" page

    all_reading = list(
        get_entries(
            dirpath="src/currently_reading", constructor=get_reading_entry_from_path
        )
    )

    template = env.get_template("list_reading.html")
    html = template.render(all_reading=all_reading, title="books i’m currently reading")

    out_path = pathlib.Path("_html") / "reading/index.html"
    out_path.parent.mkdir(exist_ok=True, parents=True)
    out_path.write_text(html)

    # Render the "want to read" page

    all_plans = list(
        get_entries(dirpath="src/plans", constructor=get_plan_entry_from_path)
    )

    template = env.get_template("list_plans.html")
    html = template.render(all_plans=all_plans, title="books i want to read")

    out_path = pathlib.Path("_html") / "to-read/index.html"
    out_path.parent.mkdir(exist_ok=True, parents=True)
    out_path.write_text(html)

    # Render the front page

    index_template = env.get_template("index.html")
    html = index_template.render(text=open("src/index.md").read())

    index_path = pathlib.Path("_html") / "index.html"
    index_path.write_text(html)

    print("✨ Rendered HTML files to _html ✨")
