# books.alexwlchan.net

This is the source code for <https://books.alexwlchan.net>, a site I use to track the books I've read.
This repo contains both the scripts that build the site, and the source data used by the scripts.

![](books_screenshot.png)

## How it works

Each book is a text file, with a bit of metadata at the top, and Markdown text in the body.
Here's an example:

```
---
book:
  title: Trans Like Me
  author: C. N. Lester
  publication_year: 2017
  cover_image: trans-like-me.jpg
  cover_desc: >
    The words “trans like me” in lowercased, large letters, set on a slightly off-white background.
    The words “trans” and “me” are in red; the word “like” is in black.
  isbn13: 9780349008608

review:
  date_read: 2020-01-27
  rating: 5
  format: paperback
---

I put off reading this for ages, but I’m glad I finally got to reading it.
```

When I run the build script, it reads all these files, and turns them into a set of HTML files.
I upload a copy of those HTML files to my web server, where they're served by nginx.

## Why...?

-   **…not Goodreads?**
    I've used Goodreads in the past, but I'm not a massive fan -- they're owned by Amazon, which isn't the greatest company.
    Their warehouse conditions are disgusting, and I've heard enough stories of questionable behaviour in the publishing industry to make me uncomfortable giving them my time or attention.

    I don't like the idea of having my data in a proprietary database, and I never used the social features.

    (Also, I find the Goodreads site quite difficult to use -- it's the de facto leader, and I think improvements stalled years ago.)

-   **…not another service?**
    I know there are other services for tracking your books, but this is a sufficiently simple problem space that I decided to just build my own thing, rather than have my data in somebody else's database.

    And doing it myself means I can be super picky, and design it exactly how I like, rather than live with somebody else's design decisions.

-   **…a static site?**
    Because static sites are great!

    There's no database to maintain, no login security to manage, and plain HTML and CSS serves blisteringly fast.
    And plain text source means it's easy to edit my data, and will remain that way for a long time.
