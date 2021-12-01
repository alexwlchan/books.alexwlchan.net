#!/usr/bin/env python3

import datetime
import hashlib
import itertools
import os
import pathlib
import re
import subprocess
import sys
import typing

import attr
import cssmin
import htmlmin
import frontmatter
from jinja2 import Environment, FileSystemLoader, select_autoescape
import markdown
from markdown.extensions.smarty import SmartyExtension
import smartypants

from generate_bookshelf import create_shelf_data_uri
from models import *
from tint_colors import get_tint_colors, store_tint_color


def rsync(dir1, dir2):
    subprocess.check_call(["rsync", "--recursive", "--delete", dir1, dir2])


def git(*args):
    return subprocess.check_output(["git"] + list(args)).strip().decode("utf8")


def set_git_timestamps():
    """
    For everything in the covers/ directory, set the last modified timestamp to
    the last time it was modified in Git.  This should make tint colour computations
    stable across machines.
    """
    root = git("rev-parse", "--show-toplevel")

    now = datetime.datetime.now().timestamp()

    for f in os.listdir("src/covers"):
        path = os.path.join("src/covers", f)

        if not os.path.isfile(path):
            continue

        stat = os.stat(path)

        # If the modified time is >7 days ago, skip setting the modified time.  This means
        # the script stays pretty fast when doing a regular sync.
        if now - stat.st_mtime > 7 * 24 * 60 * 60 and "--reset" not in sys.argv:
            continue

        revision = git("rev-list", "--max-count=1", "HEAD", path)

        if not revision:
            continue

        timestamp, *_ = git("show", "--pretty=format:%ai", "--abbrev-commit", revision).splitlines()
        modified_time = datetime.datetime.strptime(timestamp, "%Y-%m-%d %H:%M:%S %z").timestamp()

        access_time = stat.st_atime

        os.utime(path, times=(access_time, modified_time))


def get_review_entry_from_path(path):
    post = frontmatter.load(path)

    kwargs = {}
    for attr_name in Book.__attrs_attrs__:
        try:
            kwargs[attr_name.name] = post["book"][attr_name.name]
        except KeyError:
            pass

    kwargs["slug"] = os.path.basename(os.path.splitext(path)[0])
    book = Book(**kwargs)

    review = Review(**post["review"], text=post.content)

    return ReviewEntry(path=path, book=book, review=review)


def get_reading_entry_from_path(path):
    post = frontmatter.load(path)

    slug = os.path.basename(os.path.splitext(path)[0])
    book = Book(slug=slug, **post["book"])

    reading = CurrentlyReading(text=post.content)

    return CurrentlyReadingEntry(path=path, book=book, reading=reading)


def get_plan_entry_from_path(path):
    post = frontmatter.load(path)

    slug = os.path.basename(os.path.splitext(path)[0])
    book = Book(slug=slug, **post["book"])

    plan = Plan(date_added=post["plan"]["date_added"], text=post.content)

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
        return date_value.strftime("%-d %B %Y")

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
        return render_date(date_obj)
    else:
        return date_obj.strftime("%B %Y")


def save_html(template, out_name="", **kwargs):
    html = template.render(**kwargs)
    out_path = pathlib.Path("_html") / out_name / "index.html"
    out_path.parent.mkdir(exist_ok=True, parents=True)

    for s in list(re.finditer(r"<style>([^<]+)</style>", html)):
        html = html.replace(s.group(1), cssmin.cssmin(s.group(1)))

    html = htmlmin.minify(html, remove_comments=True)

    for name in ("Mar Hicks", "Thomas S. Mullaney", "Benjamin Peters", "Kavita Philip"):
        html = html.replace(name, name.replace(" ", "&nbsp;"))

    out_path.write_text(html)


def _create_new_thumbnail(src_path, dst_path):
    dst_path.parent.mkdir(exist_ok=True, parents=True)

    # Thumbnails are 240x240 max, then 2x for retina displays
    subprocess.check_call([
        "convert", src_path, "-resize", "480x480>", dst_path
    ])


def thumbnail_1x(name):
    pth = pathlib.Path(name)
    return pth.stem + "_1x" + pth.suffix


def _create_new_square(src_path, square_path):
    square_path.parent.mkdir(exist_ok=True, parents=True)

    subprocess.check_call([
        "convert",
        src_path, "-resize", "240x240", "-gravity", "center", "-background", "white", "-extent", "240x240", square_path
    ])


def create_thumbnails():
    for image_name in os.listdir("src/covers"):
        if image_name == ".DS_Store":
            continue

        src_path = pathlib.Path("src/covers") / image_name
        dst_path = pathlib.Path("_html/thumbnails") / image_name

        if not dst_path.exists():
            _create_new_thumbnail(src_path, dst_path)
        elif src_path.stat().st_mtime > dst_path.stat().st_mtime:
            _create_new_thumbnail(src_path, dst_path)

        square_path = pathlib.Path("_html/squares") / image_name

        if not square_path.exists():
            _create_new_square(src_path, square_path)
        elif src_path.stat().st_mtime > square_path.stat().st_mtime:
            _create_new_square(src_path, square_path)

        store_tint_color(dst_path)


CSS_HASH = hashlib.md5(open('static/style.css', 'rb').read()).hexdigest()


def css_hash(_):
    return f"md5:{CSS_HASH}"


def count_finished_books(review_entries: typing.List[ReviewEntry]):
    return len([r for r in review_entries if r.review.finished])


def main():
    set_git_timestamps()

    env = Environment(
        loader=FileSystemLoader("templates"),
        autoescape=select_autoescape(["html", "xml"]),
    )

    env.filters["render_markdown"] = render_markdown
    env.filters["render_date"] = render_date
    env.filters["smartypants"] = smartypants.smartypants
    env.filters["thumbnail_1x"] = thumbnail_1x
    env.filters["css_hash"] = css_hash
    env.filters["create_shelf_data_uri"] = create_shelf_data_uri
    env.filters["cap_rgb"] = lambda v: min([v, 255])
    env.filters["count_finished_books"] = count_finished_books

    create_thumbnails()

    tint_colors = get_tint_colors()

    rsync("src/covers/", "_html/covers/")
    rsync("static/", "_html/static/")

    # Render the "all reviews page"

    all_reviews = list(
        get_entries(dirpath="src/reviews", constructor=get_review_entry_from_path)
    )
    all_reviews = sorted(
        all_reviews, key=lambda rev: f"{rev.review.date_read}/{rev.review.date_order}", reverse=True
    )

    review_template = env.get_template("review.html")

    for review_entry in all_reviews:
        save_html(
            template=review_template,
            out_name=review_entry.out_path(),
            review_entry=review_entry,
            title=f"My review of {review_entry.book.title}",
            tint_colors=tint_colors
        )

    save_html(
        template=env.get_template("list_reviews.html"),
        out_name="reviews",
        all_reviews=[
            (year, list(reviews))
            for (year, reviews) in itertools.groupby(
                all_reviews, key=lambda rev: str(rev.review.date_read)[:4]
            )
        ],
        title="books i’ve read",
        this_year=str(datetime.datetime.now().year),
        tint_colors=tint_colors
    )

    # Render the "currently reading" page

    all_reading = list(
        get_entries(
            dirpath="src/currently_reading", constructor=get_reading_entry_from_path
        )
    )

    save_html(
        template=env.get_template("list_reading.html"),
        out_name="reading",
        all_reading=all_reading,
        title="books i’m currently reading",
        tint_colors=tint_colors
    )

    # Render the "want to read" page

    all_plans = list(
        get_entries(dirpath="src/plans", constructor=get_plan_entry_from_path)
    )

    save_html(
        template=env.get_template("list_plans.html"),
        out_name="to-read",
        all_plans=all_plans,
        title="books i want to read",
        tint_colors=tint_colors,
    )

    # Render the "never going to read this page"

    all_retired = list(
        get_entries(dirpath="src/will_never_read", constructor=get_plan_entry_from_path)
    )

    all_retired = sorted(
        all_retired, key=lambda plan: plan.plan.date_added, reverse=True
    )

    save_html(
        template=env.get_template("list_will_never_read.html"),
        out_name="will-never-read",
        all_retired=all_retired,
        title="books i&rsquo;m never going to read",
        tint_colors=tint_colors
    )

    # Render the front page

    save_html(
        template=env.get_template("index.html"),
        text=open("src/index.md").read(),
        reviews=all_reviews[:5],
        tint_colors=tint_colors
    )

    print("✨ Rendered HTML files to _html ✨")


if __name__ == "__main__":
    main()
