use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

use walkdir::WalkDir;

use crate::errors::VfdError;
use crate::{fs_helpers, models, templates};

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

            result.push(models::Review { metadata, slug, text });
        }
    }

    Ok(result)
}

pub fn render_html(src: &Path, dst: &Path) -> Result<(), VfdError> {
    let mut written_paths: Vec<PathBuf> = vec![];

    let reviews = get_reviews(src).unwrap();

    for rev in reviews {
        // Write individual HTML pages for each of the reviews.
        let out_dir = dst.join("reviews").join(&rev.slug);
        fs::create_dir_all(&out_dir)?;

        let out_path = out_dir.join("index.html");
        println!("{:?}", out_path);

        let mut context = tera::Context::new();
        context.insert("review", &rev);
        let html = templates::render("review.html", &context).unwrap();

        fs_helpers::write_file(&out_path, html.into_bytes())?;
        written_paths.push(out_path);
    }

    // let mut context = tera::Context::new();
    //
    //
    //
    // let titles: Vec<String> = reviews.into_iter().map(|r| r.slug).collect();
    // context.insert("titles", &titles);

    // println!("{:?}", templates::render("base.html", &context));

    println!("{:?}", dst);

    Ok(())
}
