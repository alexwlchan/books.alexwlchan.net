/// This file helps me add a new review.
///
/// It asks a series of interactive questions that are used to populate the
/// YAML front matter in a review file, and download a cover image to the
/// right directory.

use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

use chrono::Datelike;
use clap::{App, SubCommand};
use inquire::{DateSelect, Text, Select};
use inquire::validator::StringValidator;
use palette::{RelativeContrast, Srgb};
use serde::Serialize;
use regex::Regex;
use url::Url;

use crate::colours;
use crate::models;
use crate::text_helpers;
use crate::urls;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("add_review")
        .about("Start a review of a new book")
}

/// Asks the user an optional question.
///
/// Returns either their answer (as text) or None (if they don't answer).
///
fn ask_optional_question(question: &str) -> Option<String> {
    let result =
        Text::new(question)
            .prompt()
            .unwrap();

    if result.len() > 0 { Some(result) } else { None }
}

fn get_non_empty_string_value(question: &str) -> String {
    let non_empty_validator: StringValidator = &|input| if input.chars().count() == 0 {
        Err(String::from("You need to enter a value!"))
    } else {
        Ok(())
    };

    Text::new(question)
        .with_validator(non_empty_validator)
        .prompt()
        .unwrap()
        .trim()
        .to_owned()
}

pub fn get_url_value(question: &str) -> Url {
    let url_validator: StringValidator = &|input| if !urls::is_url(input) {
        Err(String::from("You need to enter a URL!"))
    } else {
        Ok(())
    };

    let response = Text::new(question)
         .with_validator(url_validator)
         .prompt()
         .unwrap();

    // We know calling .unwrap() is safe here because the validator ensures
    // the user has entered a valid URL.
    Url::parse(&response).unwrap()
}

fn get_year_value(question: &str) -> u16 {
    let year_regex = Regex::new(r"^[0-9]{4}$").unwrap();

    let validator: StringValidator = &|input| if !year_regex.is_match(input) {
        Err(String::from("You need to enter a year!"))
    } else {
        Ok(())
    };

    let answer = Text::new(question)
        .with_validator(validator)
        .prompt()
        .unwrap();

    // I know this .unwrap() is safe because the regex is ensuring that
    // the user enters a 4-digit numeric value.
    answer.parse::<u16>().unwrap()
}

#[derive(Serialize)]
struct FrontMatter {
    book: models::Book,
    review: models::ReviewMetadata,
}

fn save_review(year: i32, slug: &str, book: models::Book, metadata: models::ReviewMetadata) -> () {
    let out_dir = format!("reviews/{}", year);
    fs::create_dir_all(&out_dir).unwrap();

    let out_path = format!("{}/{}.md", out_dir, slug);

    let front_matter = FrontMatter { book, review: metadata };

    let mut file = OpenOptions::new().write(true).create_new(true).open(&out_path).unwrap();
    file.write_all(serde_yaml::to_string(&front_matter).unwrap().as_bytes()).unwrap();
    file.write("---\n\n".as_bytes()).unwrap();

    Command::new("open")
            .arg(out_path)
            .output()
            .unwrap();
}

pub fn add_review() -> () {
    let title = get_non_empty_string_value("What's the title of the book?");
    let author = get_non_empty_string_value("Who's the author?");
    let publication_year = get_year_value("When was it published?");
    let series = ask_optional_question("Is the book part of a series?");

    let format = Select::new("What format did you read it in?", vec!["audiobook", "paperback", "hardback", "ebook"])
        .prompt()
        .unwrap()
        .to_string();

    let narrator = if format == "audiobook" {
        Some(get_non_empty_string_value("Who was the narrator?"))
    } else {
        None
    };

    let [isbn10, isbn13] = if format == "paperback" || format == "hardback" {
        [
            ask_optional_question("Do you know the ISBN-10?"),
            ask_optional_question("Do you know the ISBN-13?"),
        ]
    } else {
        [None, None]
    };

    let did_finish =
        Select::new("Did you finish reading it?", vec!["yes", "no"])
            .prompt()
            .unwrap() == "yes";

    let finished_date_message = format!("When did you {} reading it?", if did_finish { "finish" } else { "stop" });

    let date_read =
        DateSelect::new(&finished_date_message)
            .prompt()
            .unwrap();

    let rating = if did_finish {
        Some(Select::new("What's your rating?", vec!["★★★★★", "★★★★", "★★★", "★★", "★"])
            .prompt()
            .unwrap()
            .len() / 3  // to account for the width of a ★ character = 3 bytes
        )
    } else {
        None
    };

    let cover_url = get_url_value("What's the URL of the cover image?");

    let slug = text_helpers::slugify(&title);

    let download_path: PathBuf = ["covers", &slug].iter().collect();

    let cover_path = match urls::download_url(&cover_url, download_path) {
        Ok(path) => (path),
        Err(e) => {
            // If we can't download the cover, retrieve it from a local
            // download.
            // TODO: Add more validation here that the path exists, use a
            // Suggester for path, etc.
            eprintln!("{}", e);
            let local_cover_path = get_non_empty_string_value("What's the path to the cover image?");

            let base_path: PathBuf = ["covers", &slug].iter().collect();
            let download_path = match PathBuf::from(&local_cover_path).extension() {
                Some(ext) => base_path.with_extension(ext),
                None      => base_path,
            };
            std::fs::rename(local_cover_path, &download_path).unwrap();
            download_path
        }
    };

    let cover_name = cover_path.file_name().unwrap().to_str().unwrap();

    let cover_size = fs::metadata(&cover_path).unwrap().len();

    let output = String::from_utf8(Command::new("dominant_colours")
        .arg(&cover_path)
        .arg("--max-colours=12")
        .arg("--no-palette")
        .output()
        .unwrap()
        .stdout).unwrap();

    let dominant_colours = output
        .trim()
        .split_ascii_whitespace()
        .map(|line| colours::parse_hex_string(line));

    let white_background: Srgb<f32> = Srgb::new(1.0, 1.0, 1.0);

    let usable_colours = dominant_colours.filter( |rgb| {
        let f32_c = Srgb::<f32>::new(rgb.red as f32 / 255.0, rgb.green as f32 / 255.0, rgb.blue as f32 / 255.0);
        white_background.has_min_contrast_text(&f32_c)
    }).collect::<Vec<Srgb<u8>>>();

    let tint_colour = match usable_colours.len() {
        0 => String::from("#ffffff"),
        1 => {
            let c = usable_colours[0];
            format!("#{:02x}{:02x}{:02x}", c.red, c.green, c.blue)
        },
        _ => {
            let hex_strings = usable_colours.into_iter().map(|c| {
                let hs = format!("#{:02x}{:02x}{:02x}", c.red, c.green, c.blue);
                format!("\x1B[38;2;{};{};{}m▇ {}\x1B[0m", c.red, c.green, c.blue, hs)
                }).collect::<Vec<String>>();
            let hs = Select::new("What's the tint colour?", hex_strings)
                .prompt()
                .unwrap();
            hs.split(" ").collect::<Vec<&str>>().last().unwrap().replace("\x1B[0m", "")
        },
    };

    let cover = models::Cover {
        name: cover_name.to_string(),
        size: cover_size as i32,
        tint_color: tint_colour.to_string(),
    };

    let book = models::Book {
        author: Some(author),
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
    };

    let metadata = models::ReviewMetadata {
        date_read: date_read.format("%Y-%m-%d").to_string(),
        format: format,
        rating: rating,
        did_not_finish: !did_finish,
        date_order: None,
    };

    save_review(date_read.year(), &slug, book, metadata);
}
