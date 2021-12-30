import datetime
import pathlib

import attr
import cattr


cattr.register_structure_hook(pathlib.Path, lambda d, t: pathlib.Path(d))


@attr.s
class CoverImage:
    name = attr.ib()
    tint_color = attr.ib()
    size = attr.ib()


@attr.s
class Book:
    slug = attr.ib()
    title = attr.ib()
    publication_year = attr.ib()
    series = attr.ib(default="")

    author = attr.ib(default="")
    editor = attr.ib(default="")
    narrator = attr.ib(default="")
    illustrator = attr.ib(default="")
    retold_by = attr.ib(default="")

    cover = attr.ib(default="")

    isbn10 = attr.ib(default="")
    isbn13 = attr.ib(default="")


@attr.s
class Review:
    date_read = attr.ib()
    text = attr.ib()
    date_order = attr.ib(default=1)
    format = attr.ib(default=None)
    rating = attr.ib(default=None)
    did_not_finish = attr.ib(default=False)

    @property
    def finished(self):
        return not self.did_not_finish


@attr.s
class ReviewEntry:
    path: pathlib.Path = attr.ib()
    book: Book = attr.ib()
    review: Review = attr.ib()

    def out_path(self):
        name = self.path.with_suffix("").name
        return pathlib.Path(f"reviews/{name}")


@attr.s
class CurrentlyReading:
    text = attr.ib(default="")


@attr.s
class CurrentlyReadingEntry:
    path: pathlib.Path = attr.ib()
    book: Book = attr.ib()
    reading: CurrentlyReading = attr.ib()


def _parse_date(value):
    if isinstance(value, datetime.date):
        return value
    else:
        return datetime.datetime.strptime(value, "%Y-%m-%d").date()


@attr.s
class Plan:
    date_added = attr.ib(converter=_parse_date)
    text = attr.ib(default="")


@attr.s
class PlanEntry:
    path: pathlib.Path = attr.ib()
    book: Book = attr.ib()
    plan: Plan = attr.ib()
