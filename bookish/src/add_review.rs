// This file contains the logic for adding a new review.
//
// It asks me a series of questions that are used to populate the YAML
// frontmatter of the Markdown file.

use std::fs;
use std::fs::OpenOptions;
use std::io::{Cursor, Write};
use std::process::Command;

use chrono::Datelike;
use clap::{App, SubCommand};
use inquire::{DateSelect, Text, Select};
use inquire::validator::StringValidator;
use palette::{RelativeContrast, Srgb};
use regex::Regex;

use crate::colours;
use crate::models;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("add_review")
        .about("Start a review of a new book")
}

fn get_string_value(question: &str) -> String {
    Text::new(question)
        .prompt()
        .unwrap()
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
}

fn get_year_value(question: &str) -> String {
    let year_regex = Regex::new(r"^[0-9]{4}$").unwrap();

    let validator: StringValidator = &|input| if !year_regex.is_match(input) {
        Err(String::from("You need to enter a year!"))
    } else {
        Ok(())
    };

    Text::new(question)
        .with_validator(validator)
        .prompt()
        .unwrap()
}

fn slugify(s: &str) -> String {
    // Replace separating punctuation
    let punctuation_regex = Regex::new("[–—/:;,.]").unwrap();
    let s = punctuation_regex.replace_all(s, "-").to_string();

    // Best ASCII substitutions, lowercased
    let s = unidecode::unidecode(&s).to_lowercase();

    // Delete any other characters
    let non_ascii_regex = Regex::new("[^a-z0-9 -]").unwrap();
    let s = non_ascii_regex.replace_all(&s, "").to_string();

    // Convert spaces to hyphens
    let s = s.replace(" ", "-");

    // Condense repeated hyphens
    let repeated_hyphen_regex = Regex::new("-+").unwrap();
    let s = repeated_hyphen_regex.replace_all(&s, "-");

    s.to_string()
}

pub fn add_review() -> () {
    let title = get_non_empty_string_value("What's the title of the book?");
    let author = get_non_empty_string_value("Who's the author?");
    let publication_year = get_year_value("When was it published?");

    let format = Select::new("What format did you read it in?", vec!["audiobook", "paperback", "hardback", "ebook"])
        .prompt()
        .unwrap()
        .to_string();

    let narrator = if format == "audiobook" {
        Some(get_string_value("Who was the narrator?"))
    } else {
        None
    };

    let [isbn10, isbn13] = if format == "paperback" || format == "hardback" {
        let input10 = get_string_value("Do you know the ISBN-10?");
        let input13 = get_string_value("Do you know the ISBN-13?");

        [
            if input10.len() > 0 { Some(input10) } else { None },
            if input13.len() > 0 { Some(input13) } else { None },
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

    let cover_url = get_non_empty_string_value("What's the cover URL?");

    let resp = reqwest::blocking::get(&cover_url).unwrap();

    let extension = cover_url.split(".").last().unwrap();
    let slug = slugify(&title);
    let cover_name = format!("{}.{}", slug, extension);
    let cover_path = format!("src/covers/{}", &cover_name);

    let mut file = std::fs::File::create(&cover_path).unwrap();
    let mut content =  Cursor::new(resp.bytes().unwrap());
    std::io::copy(&mut content, &mut file).unwrap();

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
        name: cover_name.to_owned(),
        size: cover_size as i32,
        tint_color: tint_colour.to_string(),
    };

    let book = models::Book {
        author: author,
        narrator: narrator,
        publication_year: publication_year,
        title: title,
        isbn10: isbn10,
        isbn13: isbn13,
        cover: cover,
    };

    let review = models::Review {
        date_read: date_read.format("%Y-%m-%d").to_string(),
        format: format,
        rating: rating,
        did_not_finish: !did_finish
    };

    let review_entry = models::ReviewEntry {
        book: book,
        review: review,
    };

    let out_dir = format!("src/reviews/{}", date_read.year());
    fs::create_dir_all(&out_dir).unwrap();

    let out_path = format!("src/reviews/{}/{}.md", date_read.year(), slug);

    let mut file = OpenOptions::new().write(true).create_new(true).open(&out_path).unwrap();
    file.write_all(serde_yaml::to_string(&review_entry).unwrap().as_bytes()).unwrap();
    file.write("---\n\n".as_bytes()).unwrap();

    Command::new("open")
            .arg(out_path)
            .output()
            .unwrap();
}
