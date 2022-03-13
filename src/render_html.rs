use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str;
use std::time::Instant;

use chrono::Datelike;
use tera::Tera;
use walkdir::WalkDir;

use crate::errors::VfdError;
use crate::{fs_helpers, models};

/// Returns a list of all the reviews and the review text under a given path.
fn get_reviews(root: &Path) -> Result<Vec<models::Review>, VfdError> {
    let mut result = vec![];

    for entry in WalkDir::new(root) {
        let entry = entry?;

        if entry.path().extension() == Some(OsStr::new("md")) {
            let buf = fs_helpers::read_file(entry.path())?;

            let md = match str::from_utf8(&buf) {
                Ok(md)   => md,
                Err(err) => return Err(VfdError::Utf8(err, entry.path().to_owned())),
            };

            let parts: Vec<&str> = md.split("---").collect();

            let metadata: models::Metadata = match serde_yaml::from_str(&parts[1]) {
                Ok(r)    => r,
                Err(err) => return Err(VfdError::Parse(err, entry.path().to_owned())),
            };

            let text = parts[2].to_string();
            let slug = entry.path()
                .file_name().unwrap()
                .to_str().unwrap()
                .replace(".md", "");

            let review = metadata.review;
            let book = metadata.book;

            result.push(models::Review { book, review, slug, text });
        }
    }

    Ok(result)
}

pub fn sync_static_files(dst: &Path) -> io::Result<()> {
    println!("Syncing static files...");
    fs_helpers::sync_files(Path::new("static"), &dst.join("static"))
}

pub fn render_html(templates: &Tera, src: &Path, dst: &Path) -> Result<(), VfdError> {
    let start = Instant::now();
    print!("Building HTML... ");

    let mut written_paths: Vec<PathBuf> = vec![];

    // Write the "all reviews" page
    let mut reviews = get_reviews(src).unwrap();
    reviews.sort_by(|a, b|
        if a.review.date_read == b.review.date_read {
            if a.review.date_read == "" {
                a.book.publication_year.cmp(&b.book.publication_year)
            } else {
                a.review.date_order.cmp(&b.review.date_order)
            }
        } else {
            a.review.date_read.cmp(&b.review.date_read)
        }
    );
    reviews.reverse();

    let this_year = chrono::offset::Utc::now().year();

    let mut context = tera::Context::new();
    context.insert("reviews", &reviews);
    context.insert("tint_colour", "#000000");
    context.insert("this_year", &this_year.to_string());
    let html = templates.render("list_reviews.html", &context).unwrap();

    let out_path = dst.join("reviews/index.html");
    fs_helpers::write_file(&out_path, html.into_bytes())?;
    written_paths.push(out_path);

    for rev in reviews {
        // Write individual HTML pages for each of the reviews.
        let out_dir = dst.join("reviews").join(&rev.slug);
        fs::create_dir_all(&out_dir)?;

        let out_path = out_dir.join("index.html");

        let mut context = tera::Context::new();

        context.insert("review", &rev.review);
        context.insert("book", &rev.book);
        context.insert("slug", &rev.slug);
        context.insert("text", &rev.text);

        context.insert("title", &rev.book.title);
        context.insert("tint_colour", &rev.book.cover.tint_color);

        let html = templates.render("review.html", &context).unwrap();

        fs_helpers::write_file(&out_path, html.into_bytes())?;
        written_paths.push(out_path);
    }

    let elapsed = start.elapsed();
    if elapsed.as_secs() == 0 {
        println!("done in {:?}ms", elapsed.as_millis());
    } else {
        println!("done in {:.1}s", elapsed.as_secs_f32());
    }

    Ok(())
}
