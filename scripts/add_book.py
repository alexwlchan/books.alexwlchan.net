#!/usr/bin/env python

import datetime
import os
import re
import subprocess
from urllib.request import urlretrieve

import frontmatter
import hyperlink
import inquirer
from unidecode import unidecode


def slugify(u):
    """Convert Unicode string into blog slug."""
    # https://leancrew.com/all-this/2014/10/asciifying/
    u = re.sub("[–—/:;,.]", "-", u)  # replace separating punctuation
    a = unidecode(u).lower()  # best ASCII substitutions, lowercased
    a = re.sub(r"[^a-z0-9 -]", "", a)  # delete any other characters
    a = a.replace(" ", "-")  # spaces to hyphens
    a = re.sub(r"-+", "-", a)  # condense repeated hyphens
    return a


def get_book_info():
    questions = [
        inquirer.List(
            "entry_type",
            message="What type of book is this?",
            choices=[
                "one I’ve read",
                "one I’m currently reading",
                "one I want to read",
            ],
        ),
        inquirer.Text("title", message="What’s the title of the book?"),
        inquirer.Text("author", message="Who’s the author?"),
        inquirer.Text("publication_year", message="When was it published?"),
        inquirer.Text("cover_image_url", message="What’s the cover URL?"),
        inquirer.Text("cover_desc", message="What’s the cover?"),
        inquirer.Text("isbn10", message="Do you know the ISBN-10?"),
        inquirer.Text("isbn13", message="Do you know the ISBN-13?"),
    ]

    answers = inquirer.prompt(questions)

    answers["entry_type"] = {
        "one I’ve read": "reviews",
        "one I’m currently reading": "currently_reading",
        "one I want to read": "plans",
    }[answers["entry_type"]]

    return answers


def get_review_info():
    date_read_question1 = [
        inquirer.List(
            "date_read",
            message="When did you finish reading it?",
            choices=["today", "yesterday", "another day"],
        )
    ]

    date_read = inquirer.prompt(date_read_question1)["date_read"]

    today = datetime.datetime.now()

    if date_read == "today":
        date_read = today.strftime("%Y-%m-%d")
    elif date_read == "yesterday":
        yesterday = today - datetime.timedelta(days=1)
        date_read = yesterday.strftime("%Y-%m-%d")
    else:
        date_read_question2 = [
            inquirer.Text("date_read", message="When did you finish reading it?")
        ]

        date_read = inquirer.prompt(date_read_question2)["date_read"]

        if re.match(r"^\d{4}-\d{2}-\d{2}$", date_read.strip()):
            date_read = datetime.datetime.strptime(date_read, "%Y-%m-%d").date()
        elif re.match(r"^\d{1,2} [A-Z][a-z]+ \d{4}$", date_read.strip()):
            date_read = datetime.datetime.strptime(date_read, "%d %B %Y").date()
        else:
            sys.exit("Unrecognised date: {date_read}")

    other_questions = [
        inquirer.List(
            "rating",
            message="When’s your rating?",
            choices=["★★★★★", "★★★★☆", "★★★☆☆", "★★☆☆☆", "★☆☆☆☆"],
        ),
        inquirer.Text("format", message="What format did you read it in?"),
    ]

    answers = inquirer.prompt(other_questions)
    format = answers["format"]

    rating = int(answers["rating"].count("★"))
    assert 1 <= rating <= 5

    if rating > 3:
        did_not_finish = False
    else:
        questions = [
            inquirer.List(
                "did_you_finish",
                message="Did you finish the book?",
                choices=["yes", "no"],
            ),
        ]

        did_not_finish = (inquirer.prompt(questions)["did_you_finish"] == "no")

    return {
        "date_read": date_read,
        "rating": rating,
        "format": format,
        "did_not_finish": did_not_finish,
    }


def save_cover(slug, cover_image_url):
    filename, _ = urlretrieve(cover_image_url)

    url_path = hyperlink.URL.from_text(cover_image_url).path
    extension = os.path.splitext(url_path[-1])[-1]
    cover_name = f"{slug}{extension}"
    os.rename(filename, f"src/covers/{cover_name}")

    return cover_name


if __name__ == "__main__":
    book_info = get_book_info()

    slug = slugify(book_info["title"])

    cover_name = save_cover(slug=slug, cover_image_url=book_info["cover_image_url"])

    new_entry = {
        "book": {
            "title": book_info["title"],
            "author": book_info["author"],
            "publication_year": book_info["publication_year"],
            "cover_image": cover_name,
        }
    }

    for key in ("cover_desc", "isbn10", "isbn13"):
        if book_info[key]:
            new_entry["book"][key] = book_info[key]

    if book_info["entry_type"] == "reviews":
        review_info = get_review_info()

        new_entry["review"] = {
            "date_read": review_info["date_read"],
            "rating": review_info["rating"],
            "format": review_info["format"],
        }

        if review_info["did_not_finish"]:
            new_entry["review"]["did_not_finish"] = True

        year = review_info["date_read"].year
        out_dir = f"reviews/{year}"
    else:
        out_dir = book_info["entry_type"]

    out_path = os.path.join("src", out_dir, f"{slug}.md")
    os.makedirs(os.path.dirname(out_path), exist_ok=True)

    with open(out_path, "wb") as out_file:
        frontmatter.dump(frontmatter.Post(content="", **new_entry), out_file)
        out_file.write(b"\n")

    subprocess.check_call(["open", out_path])

    from render_html import main
    main()
