#![deny(warnings)]

use clap::{App, AppSettings, Arg, SubCommand};
use regex::Regex;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn create_shelf(r: u8, g: u8, b: u8) -> String {
    String::from(format!("r = {}, g = {}, b = {}", r, g, b))
}

fn is_hex_string(s: String) -> Result<(), String> {
    let hex_regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    if hex_regex.is_match(&s) {
        Ok(())
    } else {
        Err(format!("Expected a hex string, e.g. #d01c11, got {}", s))
    }
}

// Parses a hex string as an RGB tuple, e.g. #d01c11 ~> (208, 28, 17)
//
// This function assumes the hex string is correctly formatted.
fn parse_hex_string(s: &str) -> (u8, u8, u8) {
    assert_eq!(s.len(), 7);
    let r = u8::from_str_radix(&s[1..3], 16).unwrap();
    let g = u8::from_str_radix(&s[3..5], 16).unwrap();
    let b = u8::from_str_radix(&s[5..7], 16).unwrap();
    (r, g, b)
}

fn main() {
    let create_shelf_subc =
        SubCommand::with_name("create_shelf")
            .about("Creates the shelf image used as a header on review pages")
            .arg(
                Arg::with_name("COLOUR")
                    .help("The hex colour to use as the base of this shelf")
                    .required(true)
                    .validator(is_hex_string)
            );

    let app =
        App::new("bookish")
            .version(VERSION)
            .author("Alex Chan <alex@alexwlchan.net>")
            .about("Generates the HTML files for books.alexwlchan.net")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(create_shelf_subc);

    let matches = app.get_matches();

    match matches.subcommand() {
        ("create_shelf", Some(sub_m)) => {
            // We can safely call .unwrap() here because this argument is required;
            // if it wasn't supplied then Clap has already bailed out.
            let hex_string = sub_m.value_of("COLOUR").unwrap();

            // Clap validates that this is a valid hex string for us, so it's safe
            // to pass to parse_hex_string();
            let (r, g, b) = parse_hex_string(hex_string);

            println!("{:?}", create_shelf(r, g, b));
        },
        _ => unreachable!(),
    };
}
