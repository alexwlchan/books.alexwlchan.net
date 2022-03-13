#![deny(warnings)]

#[macro_use]
extern crate lazy_static;

use std::ffi::OsStr;
use std::path::Path;
use std::str;

use walkdir::WalkDir;

mod errors;
mod fs_helpers;
mod models;
mod templates;

use errors::VfdError;

fn get_reviews(root: &Path) -> Result<Vec<(models::ReviewEntry, String)>, errors::VfdError> {
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

            let review: models::ReviewEntry = match serde_yaml::from_str(&parts[1]) {
                Ok(r)    => r,
                Err(err) => return Err(VfdError::Parse(err, entry.path().to_owned())),
            };

            let review_text = parts[2];

            result.push((review, review_text.to_owned()));
        }
    }

    Ok(result)
}

fn main() {
    let reviews = get_reviews(Path::new("reviews")).unwrap();
    // println!("{:?}", reviews);

    let mut context = tera::Context::new();
    let titles: Vec<String> = reviews.into_iter().map(|(r, _)| r.book.title).collect();
    context.insert("titles", &titles);

    println!("{:?}", templates::render("base.html", &context));
}
