// This file contains the logic for adding a new review.
//
// It asks me a series of questions that are used to populate the YAML
// frontmatter of the Markdown file.

use std::fs;
use std::io::{Read, Cursor};

use clap::{App, SubCommand};
use inquire::{DateSelect, Text, Select};
use inquire::validator::StringValidator;
use regex::Regex;

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
    // let title = get_non_empty_string_value("What's the title of the book?");
    // let author = get_non_empty_string_value("Who's the author?");
    // let publication_year = get_year_value("When was it published?");
    // let cover_url = get_non_empty_string_value("What's the cover URL?");
    //
    // let format = Select::new("What format did you read it in?", vec!["audiobook", "paperback", "hardback", "ebook"])
    //     .prompt()
    //     .unwrap();
    //
    // let narrator = if format == "audiobook" {
    //     Some(get_string_value("Who was the narrator?"))
    // } else {
    //     None
    // };
    //
    // let [isbn10, isbn13] = if format == "paperback" || format == "hardback" {
    //     let input10 = get_string_value("Do you know the ISBN-10?");
    //     let input13 = get_string_value("Do you know the ISBN-13?");
    //
    //     [
    //         if input10.len() > 0 { Some(input10) } else { None },
    //         if input13.len() > 0 { Some(input13) } else { None },
    //     ]
    // } else {
    //     [None, None]
    // };
    //
    // let did_finish =
    //     Select::new("Did you finish reading it?", vec!["yes", "no"])
    //         .prompt()
    //         .unwrap() == "yes";
    //
    // let finished_date_message = format!("When did you {} reading it?", if did_finish { "finish" } else { "stop" });
    //
    // let finished_date =
    //     DateSelect::new(&finished_date_message)
    //         .prompt()
    //         .unwrap();
    //
    // let rating = if did_finish {
    //     Some(Select::new("What's your rating?", vec!["★★★★★", "★★★★☆", "★★★☆☆", "★★☆☆☆", "★☆☆☆☆"])
    //         .prompt()
    //         .unwrap())
    // } else {
    //     None
    // };
    //
    // println!("The finished date = {:?}", finished_date);
    // println!("The rating = {:?}", rating);
    //
    // let isbns = [isbn10, isbn13, narrator];
    // println!("The slug is {:?}", isbns);
    //
    // let answers = [author, publication_year];
    // println!("The slug is {:?}", slugify(&title));
    //
    // println!("The answers are {:?}", answers);
    // println!("Add review!");

    let title = "Lemoiny Snicket";
    let cover_url = "https://books.alexwlchan.net/thumbnails/cosmogramma.jpg";

    let extension = cover_url.split(".").last().unwrap();

    let mut resp = reqwest::blocking::get(cover_url).unwrap();

    let cover_name = format!("{}.{}", slugify(title), extension);
    let cover_path = format!("src/covers/{}", cover_name);

    let mut file = std::fs::File::create(&cover_path).unwrap();
    let mut content =  Cursor::new(resp.bytes().unwrap());
    std::io::copy(&mut content, &mut file).unwrap();

    let cover_size = fs::metadata(cover_path).unwrap().len();
    println!("cover_size = {}", cover_size);
}
