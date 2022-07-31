# CHANGELOG

## v1.4.3 - 2022-07-31

Improve the error message when the user presses Ctrl+C while using `vfd add_review`.

## v1.4.2 - 2022-07-24

Improve the error message when there's an issue with the cover image specified in a review's metadata.

## v1.4.1 - 2022-07-23

Improve the accuracy of reported time taken; in particular `<1ms` rather than `0ms`.

## v1.4.0 - 2022-07-06

Allow setting `--host` and `--port` as arguments to `vfd serve`.

Also remove the automatic web server start from `add_review`.

## v1.3.14 - 2022-07-03

Fix a bug where images would look misshapen if re-cropped while running `vfd serve`.

## v1.3.13 - 2022-06-29

Fetch the dimensions of cover images and pass it to templates, so this information can be passed in the CSS on the `/review` page to simplify page layout.

In particular, now we can give exact dimensions for images *before* the browser loads them, so the page doesn't need to be continually re-laid out as images are loaded.

## v1.3.12 - 2022-06-29

Tweak the way shelves are generated, so there's more variety of light/dark books on different pages.

## v1.3.11 - 2022-06-29

Add support for favicons.  vfd will now look for files `static/favicon_16.png` and `static/favicon_32.png` which are fully black images with transparency (i.e. every pixel is `rgba(0, 0, 0, alpha)`), and then create appropriately tinted versions for each review page.

## v1.3.10 - 2022-06-29

Fix a bug where `vfd build` would panic when trying to check the latest version if it was unable to connect to the GitHub API, e.g. if the Internet connection was offline.

## v1.3.9 - 2022-06-23

When adding a review, vfd now opens a pre-filled Google Images search for possible covers to help me pick a cover image.

## v1.3.8 - 2022-05-28

Use the [`image` crate](https://crates.io/crates/image) instead of ImageMagick to create thumbnails. In informal testing, this is ~25% faster.

## v1.3.7 - 2022-05-19

Expose the last Git commit to modify the CSS to the templates, so I can serve CSS files with long-lasting cache headers but also be able to modify them and have browsers fetch up-to-date CSS.

## v1.3.6 - 2022-05-02

Remove some now-unused code for tracking the size of cover images.

## v1.3.5 - 2022-05-02

Improve the error message when you run `vfd deploy` and something goes wrong when deploying to Netlify.

## v1.3.4 - 2022-04-11

Make the black shelves in the header of the homepage/list of reviews a little less black, so it looks more like a collection of books.

## v1.3.3 - 2022-03-30

Fix a bug where sometimes covers wouldn't download correctly when adding a review.

## v1.3.2 - 2022-03-30

Fix a bug where the Netlify builds process wasn't picking up the `_redirects` file.

## v1.3.1 - 2022-03-30

Fix a couple of bugs that happen when building the HTML from scratch, i.e. without a pre-existing local copy of the HTML.

## v1.3.0 - 2022-03-30

Add a `build` command that does a one-off build of the HTML.
This is meant for doing automated builds in CI.

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
