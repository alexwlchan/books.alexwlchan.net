/// This file helps me add a new review.
///
/// It asks a series of interactive questions that are used to populate the
/// YAML front matter in a review file, and download a cover image to the
/// right directory.
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::Datelike;
use clap::{App, SubCommand};
use inquire::error::InquireResult;
use inquire::validator::Validation;
use inquire::{DateSelect, Select, Text};
use regex::Regex;
use serde::Serialize;
use url::Url;

use crate::{models, tags, text_helpers, urls};

pub fn subcommand() -> App<'static> {
    SubCommand::with_name("add_review").about("Start a review of a new book")
}

/// Asks the user an optional question.
///
/// Returns either their answer (as text) or None (if they don't answer).
///
fn ask_optional_question(question: &str) -> InquireResult<Option<String>> {
    let result = Text::new(question).prompt()?;

    if result.len() > 0 {
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

fn get_non_empty_string_value(question: &str) -> InquireResult<String> {
    let non_empty_validator = |input: &str| {
        if input.chars().count() == 0 {
            Ok(Validation::Invalid("You need to enter a value!".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let answer = Text::new(question)
        .with_validator(non_empty_validator)
        .prompt()?;

    Ok(answer.trim().to_owned())
}

pub fn get_url_value(question: &str) -> InquireResult<Url> {
    let url_validator = |input: &str| {
        if !urls::is_url(input) {
            Ok(Validation::Invalid("You need to enter a URL!".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let response = Text::new(question).with_validator(url_validator).prompt()?;

    // We know calling .unwrap() is safe here because the validator ensures
    // the user has entered a valid URL.
    Ok(Url::parse(&response).unwrap())
}

fn get_year_value(question: &str) -> InquireResult<u16> {
    let validator = |input: &str| {
        let year_regex = Regex::new(r"^[0-9]{4}$").unwrap();

        if !year_regex.is_match(input) {
            Ok(Validation::Invalid("You need to enter a year!".into()))
        } else {
            Ok(Validation::Valid)
        }
    };

    let answer = Text::new(question).with_validator(validator).prompt()?;

    // I know this .unwrap() is safe because the regex is ensuring that
    // the user enters a 4-digit numeric value.
    Ok(answer.parse::<u16>().unwrap())
}

#[derive(Serialize)]
struct FrontMatter {
    book: models::Book,
    review: models::ReviewMetadata,
}

fn save_review(root: &Path, year: i32, slug: &str, book: models::Book, metadata: models::ReviewMetadata) -> () {
    let out_dir = root.join(year.to_string());
    fs::create_dir_all(&out_dir).unwrap();

    let out_path = out_dir.join(slug).with_extension("md");

    let front_matter = FrontMatter {
        book,
        review: metadata,
    };

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&out_path)
        .unwrap();
    file.write_all(serde_yaml::to_string(&front_matter).unwrap().as_bytes())
        .unwrap();
    file.write("---\n\n".as_bytes()).unwrap();

    Command::new("open").arg(out_path).output().unwrap();
}

pub fn add_review(root: &Path) -> InquireResult<()> {
    let title = get_non_empty_string_value("What's the title of the book?")?;
    let author = get_non_empty_string_value("Who's the author?")?;
    let publication_year = get_year_value("When was it published?")?;
    let series = ask_optional_question("Is the book part of a series?")?;

    let format = Select::new(
        "What format did you read it in?",
        vec!["audiobook", "paperback", "hardback", "ebook"],
    )
    .prompt()?
    .to_string();

    let narrator = if format == "audiobook" {
        Some(get_non_empty_string_value("Who was the narrator?")?)
    } else {
        None
    };

    let [isbn10, isbn13] = if format == "paperback" || format == "hardback" {
        [
            ask_optional_question("Do you know the ISBN-10?")?,
            ask_optional_question("Do you know the ISBN-13?")?,
        ]
    } else {
        [None, None]
    };

    let did_finish =
        Select::new("Did you finish reading it?", vec!["yes", "no"]).prompt()? == "yes";

    let finished_date_message = format!(
        "When did you {} reading it?",
        if did_finish { "finish" } else { "stop" }
    );

    let date_read = DateSelect::new(&finished_date_message).prompt()?;

    let rating = if did_finish {
        Some(
            Select::new(
                "What's your rating?",
                vec!["★★★★★", "★★★★", "★★★", "★★", "★"],
            )
            .prompt()?
            .len()
                / 3, // to account for the width of a ★ character = 3 bytes
        )
    } else {
        None
    };

    // Open a pre-filled image search for the book in question, to help
    // me find suitable covers.
    let search_query = format!("{} by {}", title, author);
    let search_url = format!(
        "https://www.google.co.uk/search?safe=images&tbm=isch&as_q={}&tbs=isz%3Al",
        urlencoding::encode(&search_query)
    );
    let _ = webbrowser::open(&search_url);

    let cover_url = get_url_value("What's the URL of the cover image?")?;

    let slug = text_helpers::slugify(&title);

    let download_path: PathBuf = ["covers", &slug].iter().collect();

    let cover_path = match urls::download_url(&cover_url, download_path) {
        Ok(path) => path,
        Err(e) => {
            // If we can't download the cover, retrieve it from a local
            // download.
            // TODO: Add more validation here that the path exists, use a
            // Suggester for path, etc.
            eprintln!("{}", e);
            let local_cover_path =
                get_non_empty_string_value("What's the path to the cover image?")?;

            let base_path: PathBuf = ["covers", &slug].iter().collect();
            let download_path = match PathBuf::from(&local_cover_path).extension() {
                Some(ext) => base_path.with_extension(ext),
                None => base_path,
            };
            std::fs::rename(local_cover_path, &download_path).unwrap();
            download_path
        }
    };

    let cover_name = cover_path.file_name().unwrap().to_str().unwrap();

    let output = String::from_utf8(
        Command::new("dominant_colours")
            .arg(&cover_path)
            .arg("--max-colours=12")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    let hex_strings = output.split("\n").collect();

    let hs = Select::new("What's the tint colour?", hex_strings).prompt()?;
    let tint_colour = hs
        .split(" ")
        .collect::<Vec<&str>>()
        .last()
        .unwrap()
        .replace("\x1B[0m", "");

    let used_tags = tags::get_used_tags(root);
    let tags = Text::new("What's the book about?")
        .with_autocomplete(tags::TagCompleter::new(used_tags.into_iter().collect()))
        .prompt()
        .unwrap()
        .trim()
        .split(" ")
        .map(|s| s.to_owned())
        .collect();

    let cover = models::Cover {
        name: cover_name.to_string(),
        tint_color: tint_colour.to_string(),
    };

    let book = models::Book {
        author: Some(author),
        author_names: None,
        narrator,
        publication_year,
        title,
        series,
        isbn10,
        isbn13,
        cover,
        editor: None,
        illustrator: None,
        retold_by: None,
        translated_by: None,
        tags: Some(tags),
    };

    let metadata = models::ReviewMetadata {
        date_read: date_read.format("%Y-%m-%d").to_string(),
        format: format,
        rating: rating,
        did_not_finish: !did_finish,
        date_order: None,
    };

    save_review(root, date_read.year(), &slug, book, metadata);

    Ok(())
}
