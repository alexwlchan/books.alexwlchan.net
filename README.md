# books.alexwlchan.net

This is the source code for <https://books.alexwlchan.net>, a site I use to track the books I've read.
It creates bright, colourful cards for each book, with a tint colour based on the book's cover.
Each card links to a longer, more detailed review.

![A screenshot of the homepage, which has a brief introductory paragraph and a list of three recent books.](books_screenshot.png)

This repo contains my reviews, the cover images, and the Rust tool that creates the site.



## How I store my reviews

Individual reviews are stored as plaintext files in the `reviews` directory, one per book I've read.
Here's an example:

```
---
book:
  author: Susanna Clarke
  cover:
    name: piranesi.jpg
    size: 1992812
    tint_color: "#916540"
  publication_year: "2020"
  title: Piranesi
review:
  date_read: 2022-01-20
  rating: 5
---

This is a weird but delightful book.
```

Each book also has a cover image in the `covers` directory, which is linked in the `book.cover.name` metadata field.

I have a CLI tool that helps me create these two files.
It asks me a series of questions about the book, then creates the review file, downloads the cover image, and extracts the tint colour.

The same CLI tool will generate the site.
It reads the Markdown files, renders them as HTML, and resizes the cover images so they're the right size to use on the site.
These files then get deployed to [Netlify].

[Netlify]: https://www.netlify.com/



## How I manage my reviews (the VFD CLI tool)

This repo includes a CLI tool called "vfd", which stands for "Vivid Folio Deliberations".
It's named after the [secret organisation of the same acronym][vfd] from the Lemony Snicket books.

It has three commands:

-   `vfd add_review` helps me create a new review.
    It asks a series of questions as interactive prompts in my terminal, including the title, author, and publication year of the book.
    It uses my answers to populate the Markdown file in the `reviews` directory.

-   `vfd serve` builds the site locally, and serves it on <http://localhost:5959>.
    When the source files change, it rebuilds the site.

-   `vfd deploy` builds the HTML pages, and uploads them to Netlify.

The tool is very [situated] and unlikely to be useful to anybody else, but there might be some ideas that you can use elsewhere.

To find out how vfd works, read the comments [in the source code](./src/main.rs).

[vfd]: https://snicket.fandom.com/wiki/Volunteer_Fire_Department
[situated]: https://www.drmaciver.com/2018/11/situated-software/



## Interesting ideas

*   Sylvain Kerkour's blog post [**Building a static site generator in 100 lines of Rust**](https://kerkour.com/rust-static-site-generator) helped me get the static site generator up and running.
    The code for serving files and hot reloading was particularly useful.

*   The Rust crate [**inquire**](https://crates.io/crates/inquire) for building interactive prompts allowed me to build some really nice interactive prompts for the `add_review` command.
    It includes free text fields, selecting from a fixed list, and even a calendar picker:

    ![Screenshot of a terminal with an inline calendar picker.](inquire_screenshot.png)

    I customise some of the questions based on the answers; for example, it only asks "Who was the narrator?" if I read the book as an audiobook.

*   I use [the **hotwatch** crate](https://crates.io/crates/hotwatch) to watch for changes in the source folder, and rebuild the HTML.
    Because the source files are split across several directories, I listen to each directory individually and only rebuild the relevant parts of the site.

    For example, if there's a change in the `covers` directory, I only need to re-run the image processing, and not rebuild the HTML files.

*   Dr Drang's blog post [**ASCIIfying**](http://www.leancrew.com/all-this/2014/10/asciifying/) continues to be my go-to when I need to turn arbitrary text (book titles) into a URL-safe slug.

*   The coloured bookshelf graphics at the top of every page use the [dominant colour](https://github.com/alexwlchan/dominant_colours) of the book's cover, and I've written [a blog post](https://alexwlchan.net/2022/01/rusty-shelves/) about how they're generated.


## Motivation

I want a good way to track my books -- both to help me remember what I've read, and so I think more about why I like the books I do.

I've tried Goodreads and a couple of other sites, but they don't really work for me -- they emphasise more social features than book tracking, and I can't be as nit picky with the details as I'd like.

Building my own site allows me to be very picky, which is particularly useful for book covers.
I find covers easy to remember -- I may not know a book if you tell me the title, but show me the cover and you'll get instant recognition.
Being able to pick the covers (and then tint the site around them) really works for me.

This was also a good chance to try Rust in a larger project.
It's big enough to be interesting, but not so big that it's overwhelming.
