#!/usr/bin/env python3

import base64
import datetime
import functools
import hashlib
import itertools
import json
import os
import pathlib
import re
import subprocess
import sys
import typing

import cattr

from models import Book, Review, ReviewEntry


def get_review_entry_from_path(path):
    import frontmatter
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


def get_entries(T, name, dirpath, constructor):
    """
    Look up the source data for all the entries in the given dirpath.

    It turns out actually parsing dozens of Markdown files is moderately expensive,
    and makes the script feels a bit sluggish -- so all the metadata gets cached
    as JSON in the .cache folder.  We only have to parse a single file, not every
    individual Markdown file.

    Files are purged from the cache based on their last modified time, but this
    function will repopulate the cache each time.
    """
    try:
        entries = {
            pathlib.Path(p): entry
            for p, entry in json.load(
                open(os.path.join(".cache", f"{name}.json"))
            ).items()
        }
    except (FileNotFoundError, ValueError):
        entries = {}

    for dirpath, _, filenames in os.walk(dirpath):
        for f in filenames:
            if not f.endswith(".md"):
                continue

            path = pathlib.Path(dirpath) / f

            if os.stat(path).st_mtime <= entries.get(path, {}).get("mtime", 0):
                continue

            try:
                entries[path] = {
                    "mtime": os.stat(path).st_mtime,
                    "data": cattr.unstructure(constructor(path)),
                }
            except Exception:
                print(f"Error parsing {path}", file=sys.stderr)
                raise

    os.makedirs(".cache", exist_ok=True)

    class CustomEncoder(json.JSONEncoder):
        def default(self, obj):
            if isinstance(obj, pathlib.PosixPath):
                return str(obj)
            elif isinstance(obj, datetime.date):
                return obj.isoformat()
            else:
                return super().default(obj)

    with open(os.path.join(".cache", f"{name}.json"), "w") as outfile:
        outfile.write(
            json.dumps({str(k): v for k, v in entries.items()}, cls=CustomEncoder)
        )

    return {path: cattr.structure(entry["data"], T) for path, entry in entries.items()}


def render_markdown(text):
    import markdown
    from markdown.extensions.smarty import SmartyExtension

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


def save_html(*, depends_on, template_name, out_name="", **kwargs):
    out_path = pathlib.Path("_html") / out_name / "index.html"

    if (
        out_path.exists()
        and depends_on
        and os.stat(out_path).st_mtime > max(os.stat(p).st_mtime for p in depends_on)
    ):
        return

    env = get_environment()
    template = env.get_template(template_name)
    html = template.render(**kwargs)
    out_path.parent.mkdir(exist_ok=True, parents=True)

    for name in ("Mar Hicks", "Thomas S. Mullaney", "Benjamin Peters", "Kavita Philip"):
        html = html.replace(name, name.replace(" ", "&nbsp;"))

    out_path.write_text(html)


def create_shelf_data_uri(tint_color):
    if max(tint_color) <= 13:
        tint_color = (13, 13, 13)

    out_name = "_shelves/%02x%02x%02x.png" % tint_color

    try:
        f = open(out_name, "rb")
    except FileNotFoundError:
        subprocess.check_call(["bookish", "create_shelf", "#%02x%02x%02x" % tint_color])
        f = open(out_name, "rb")

    b64_string = base64.b64encode(f.read()).decode("utf8")
    return f"data:image/png;base64,{b64_string}"


CSS_HASH = hashlib.md5(open("static/style.css", "rb").read()).hexdigest()


def css_hash(_):
    return f"md5:{CSS_HASH}"


def count_finished_books(review_entries: typing.List[ReviewEntry]):
    return len([r for r in review_entries if r.review.finished])


@functools.lru_cache
def from_hex(hs):
    return (int(hs[1:3], 16), int(hs[3:5], 16), int(hs[5:7], 16))


def as_rgba(hs, alpha):
    r, g, b = from_hex(hs)
    return f"rgb({r / 255}, {g / 255}, {b / 255}, {alpha})"


@functools.lru_cache
def get_environment():
    from jinja2 import Environment, FileSystemLoader, select_autoescape
    import smartypants

    env = Environment(
        loader=FileSystemLoader("templates"),
        autoescape=select_autoescape(["html", "xml"]),
    )

    env.filters["render_markdown"] = render_markdown
    env.filters["render_date"] = render_date
    env.filters["smartypants"] = smartypants.smartypants
    env.filters["css_hash"] = css_hash
    env.filters["create_shelf_data_uri"] = create_shelf_data_uri
    env.filters["cap_rgb"] = lambda v: min([v, 255])
    env.filters["count_finished_books"] = count_finished_books
    env.filters["from_hex"] = from_hex
    env.filters["as_rgba"] = as_rgba

    return env


def main():
    # Render the "all reviews page"

    all_reviews = get_entries(
        ReviewEntry,
        name="reviews",
        dirpath="src/reviews",
        constructor=get_review_entry_from_path,
    )

    for review_path, review_entry in all_reviews.items():
        save_html(
            depends_on=[review_path],
            template_name="review.html",
            out_name=review_entry.out_path(),
            review_entry=review_entry,
            title=f"My review of {review_entry.book.title}",
        )

    sorted_reviews = sorted(
        all_reviews.values(),
        key=lambda rev: f"{rev.review.date_read}/{rev.review.date_order}",
        reverse=True,
    )

    save_html(
        depends_on=all_reviews.keys(),
        template_name="list_reviews.html",
        out_name="reviews",
        all_reviews=[
            (year, list(reviews))
            for (year, reviews) in itertools.groupby(
                sorted_reviews, key=lambda rev: str(rev.review.date_read)[:4]
            )
        ],
        title="books i’ve read",
        this_year=str(datetime.datetime.now().year),
    )

    # Render the front page

    save_html(
        depends_on=all_reviews.keys(),
        template_name="index.html",
        text=open("src/index.md").read(),
        reviews=sorted_reviews[:5],
    )


if __name__ == "__main__":
    main()
