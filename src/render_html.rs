use std::cmp::Ordering;
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str;

use chrono::Datelike;
use html_minifier::HTMLMinifier;
use image::imageops::FilterType;
use image::GenericImageView;
use tera::Tera;
use walkdir::WalkDir;

use crate::create_favicon::create_favicon;
use crate::errors::VfdError;
use crate::fs_helpers::IsNewerThan;
use crate::{fs_helpers, models};

/// Returns a list of all the reviews and the review text under a given path.
pub fn get_reviews(root: &Path) -> Result<Vec<models::Review>, VfdError> {
    let mut result = vec![];

    for entry in WalkDir::new(root) {
        let entry = entry?;

        if entry.path().extension() == Some(OsStr::new("md")) {
            let buf = fs_helpers::read_file(entry.path())?;

            let md = match str::from_utf8(&buf) {
                Ok(md) => md,
                Err(err) => return Err(VfdError::Utf8(err, entry.path().to_owned())),
            };

            let parts: Vec<&str> = md.split("---").collect();

            let metadata: models::Metadata = match serde_yaml::from_str(&parts[1]) {
                Ok(r) => r,
                Err(err) => return Err(VfdError::Parse(err, entry.path().to_owned())),
            };

            let text = parts[2].to_string();
            let path = entry.path();
            let slug = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .replace(".md", "");

            let review = metadata.review;
            let book = metadata.book;

            let cover_path = PathBuf::from("covers").join(&book.cover.name);
            let (width, height) = match image::image_dimensions(&cover_path) {
                Ok(dim) => dim,
                Err(e) => return Err(VfdError::CoverInfo(entry.path().to_owned(), e, cover_path)),
            };
            let derived_cover_info = models::DerivedCoverInfo { width, height };

            result.push(models::Review {
                book,
                review,
                slug,
                text,
                path: path.to_owned(),
                derived_cover_info,
            });
        }
    }

    Ok(result)
}

pub fn sync_static_files(dst: &Path) -> io::Result<()> {
    // Copy the _redirects file into place.  This is a Netlify-specific feature.
    //
    // See https://docs.netlify.com/routing/redirects/
    fs::copy("_redirects", &dst.join("_redirects"))?;

    fs_helpers::sync_files(Path::new("static"), &dst.join("static"))
}

fn write_html(p: &Path, html: String) -> Result<(), VfdError> {
    let mut html_minifier = HTMLMinifier::new();
    html_minifier.digest(&html)?;
    let minified_html = html_minifier.get_html();

    fs_helpers::write_file(p, minified_html.to_vec())?;

    Ok(())
}

/// This describes the two modes of rendering HTML: a full rebuild will rebuild
/// everything, an incremental build will skip building pages that haven't changed.
#[derive(Debug, PartialEq, Eq)]
pub enum HtmlRenderMode {
    Incremental,
    Full,
}

pub fn render_html(
    templates: &Tera,
    src: &Path,
    dst: &Path,
    mode: HtmlRenderMode,
) -> Result<(), VfdError> {
    let mut written_paths: HashSet<PathBuf> = HashSet::new();

    // Write the "all reviews" page
    let mut reviews = get_reviews(src)?;
    reviews.sort_by(|a, b| match (a.review.as_ref(), b.review.as_ref()) {
        (None, None) => a.book.publication_year.cmp(&b.book.publication_year),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (Some(a_rev), Some(b_rev)) => {
            if a_rev.date_read == b_rev.date_read {
                a_rev.date_order.cmp(&b_rev.date_order)
            } else {
                a_rev.date_read.cmp(&b_rev.date_read)
            }
        }
    });
    reviews.reverse();

    // Finds the last Git commit where the CSS file was modified.
    //
    // This gets embedded in the URL to the CSS file, so I can set a
    // very long Cache-Control header but still have browsers fetch
    // the file fresh whenever it changes.
    let css_commit = String::from_utf8(
        Command::new("git")
            .arg("log")
            .arg("--max-count=1")
            .arg("--pretty=format:%H")
            .arg("--")
            .arg("static/style.css")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let this_year = chrono::offset::Utc::now().year();

    let mut context = tera::Context::new();
    context.insert("reviews", &reviews);
    context.insert("tint_colour", "#191919");
    context.insert("this_year", &this_year.to_string());
    context.insert("css_commit", &css_commit.trim().to_string());
    let html = templates.render("list_reviews.html", &context)?;
    create_favicon("#191919");

    let out_path = dst.join("reviews/index.html");
    write_html(&out_path, html)?;
    written_paths.insert(out_path);

    // Write the homepage
    let mut context = tera::Context::new();
    context.insert("reviews", &reviews);
    context.insert("tint_colour", "#191919");
    context.insert("is_homepage", &true);
    context.insert("css_commit", &css_commit.trim().to_string());
    let html = templates.render("index.html", &context)?;

    let out_path = dst.join("index.html");
    write_html(&out_path, html)?;
    written_paths.insert(out_path);

    // Write individual HTML pages for each of the reviews.
    for rev in reviews {
        // Don't bother writing individual review pages for books I read at
        // another time; there's nothing useful there.
        if rev.review.is_none() {
            continue;
        }

        let out_dir = dst.join("reviews").join(&rev.slug);
        fs::create_dir_all(&out_dir)?;

        let out_path = out_dir.join("index.html");

        // To speed up incremental rebuilds, we skip writing individual review
        // pages if:
        //
        //    - the build mode is incremental, and
        //    - the existing HTML file is newer than the source file
        //
        // If for some reason we can't get the modified time of one of the files,
        // we assume we need to rebuild, to be on the safe side.
        //
        if mode == HtmlRenderMode::Full || rev.path.is_newer_than(&out_path).unwrap_or(true) {
            let mut context = tera::Context::new();

            context.insert("review", &rev.review);
            context.insert("book", &rev.book);
            context.insert("slug", &rev.slug);
            context.insert("text", &rev.text);

            context.insert("title", &rev.book.title);
            context.insert("tint_colour", &rev.book.cover.tint_color);

            context.insert("css_commit", &css_commit.trim().to_string());

            let html = templates.render("review.html", &context).unwrap();
            write_html(&out_path, html)?;

            create_favicon(&rev.book.cover.tint_color);
        }

        written_paths.insert(out_path);
    }

    // Clean up any HTML files that were written by a previous version of the
    // site, but which we no longer need.
    //
    // Note: I'm deliberately discarding the result of the `remove_file()`; if
    // I was being picky I might want to look for an ENOENT error result
    // and throw if I don't get that, but this is fine for now.
    for entry in WalkDir::new(&dst) {
        let entry = entry?;

        if entry.path().extension() == Some(OsStr::new("html")) {
            if !written_paths.contains(entry.path()) {
                let _ = fs::remove_file(entry.path());
            }
        }
    }

    Ok(())
}

pub fn create_thumbnails(dst: &Path) -> Result<(), VfdError> {
    fs::create_dir_all(&dst.join("squares"))?;
    fs::create_dir_all(&dst.join("thumbnails"))?;

    for entry in fs::read_dir("covers")? {
        let entry = entry?;
        let src_path = entry.path();

        if fs_helpers::is_ds_store(&src_path) {
            continue;
        }

        let name = src_path.file_name().unwrap();

        let thumbnail_path = dst.join("thumbnails").join(name);
        let square_path = dst.join("squares").join(name);

        if src_path.is_newer_than(&thumbnail_path)? || src_path.is_newer_than(&square_path)? {
            let src_img = match image::open(&src_path) {
                Ok(im) => im,
                Err(e) => return Err(VfdError::Thumbnail(e)),
            };

            // Thumbnails are 240x240 max, then 2x for retina displays
            let thumbnail_img = src_img.resize(480, 480, FilterType::Gaussian);

            match thumbnail_img.save(&thumbnail_path) {
                Ok(_) => (),
                Err(e) => return Err(VfdError::Thumbnail(e)),
            };

            let mut square_img =
                image::ImageBuffer::from_pixel(480, 480, image::Rgba([255, 255, 255, 1]));

            let (x_offset, y_offset) =
                // Portrait image
                if thumbnail_img.width() < thumbnail_img.height() {
                    let x_offset = (480 - thumbnail_img.width()) / 2;
                    (x_offset, 0)
                }
                // Landscape image
                else if thumbnail_img.width() > thumbnail_img.height() {
                    let y_offset = (480 - thumbnail_img.height()) / 2;
                    (0, y_offset)
                }
                // Square image
                else {
                    (0, 0)
                };

            image::imageops::overlay(&mut square_img, &thumbnail_img, x_offset, y_offset);

            match square_img.save(&square_path) {
                Ok(_) => (),
                Err(e) => return Err(VfdError::Thumbnail(e)),
            };
        }
    }

    Ok(())
}
