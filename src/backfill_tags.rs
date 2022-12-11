use std::fs::File;
use std::io::Write;
use std::path::{Path};

use clap::{App, SubCommand};
use serde_yaml;

use crate::{add_review, models, render_html};

pub fn subcommand() -> App<'static> {
    SubCommand::with_name("backfill_tags").about("Apply tags to reviews that pre-date tags")
}

pub fn backfill_tags(root: &Path) -> () {
    let reviews = render_html::get_reviews(root).unwrap();

    for rev in reviews.iter() {
        if rev.book.tags.is_none() {
            let question = format!("How should {:?} be tagged? ({:?})\n>", rev.book.title, rev.path);
            let tags = add_review::get_tags(&question);

            let mut file = File::create(&rev.path).unwrap();

            let front_matter = models::Metadata {
                book: models::Book {
                    tags: Some(tags),
                    ..rev.book.clone()
                },
                review: rev.review.clone(),
            };


            file.write_all(serde_yaml::to_string(&front_matter).unwrap().as_bytes())
                .unwrap();
            file.write("---".as_bytes()).unwrap();
            file.write(rev.text.as_bytes()).unwrap();

            println!("");
        }
    }
}