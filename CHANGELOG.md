# CHANGELOG

## v1.2.2 - 2022-03-26

Add a startup check that you're running the latest version.
If not, vfd will prompt you to download the newest version.

## v1.2.1 - 2022-03-26

No-op release to test the auto-release mechanism.

## v1.2.0 - 2022-03-17

Better handling of books that were read before I started writing down when I read books ("books read at another time").

-  Don't generate individual review pages for them, because they don't have anything useful
-  Omit the `review:` entry entirely in their YAML frontmatter, rather than having it there with empty values

## v1.1.1 - 2022-03-17

Internal refactoring that should have no user-visible effect.

## v1.1.0 - 2022-03-17

Store the publication year as a number, not a string.

This should prevent `null` or other non-years sneaking through as a publication year in the rendered version of the site.

## v1.0.4 - 2022-03-16

Internal refactoring that should speed up HTML generation when running `vfd serve`.

In particular, this patch adds incremental compilation of the individual review pages.
When I'm working locally and editing individual review files, it skips rebuilding the HTML page if the original review file hasn't change.
This makes updates ~40% faster.

## v1.0.3 - 2022-03-14

Make a small tweak to hopefully make HTML generation a bit faster.

## v1.0.2 - 2022-03-14

Minify the generated HTML.

## v1.0.1 - 2022-03-14

Remember to include the [Netlify redirects file](https://docs.netlify.com/routing/redirects/) in the `_html` directory.

## v1.0.0 - 2022-03-14

Initial release.
