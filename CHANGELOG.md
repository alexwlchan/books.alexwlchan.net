# CHANGELOG

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
