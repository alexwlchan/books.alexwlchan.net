use clap::{Arg, App, SubCommand};
use regex::Regex;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("create_shelf")
        .about("Creates the shelf image used as a header on review pages")
        .arg(
            Arg::with_name("COLOUR")
                .help("The hex colour to use as the base of this shelf")
                .required(true)
                .validator(is_hex_string)
        )
}

// Creates a shelf file.
//
// This function assumes the hex string is correctly formatted.
pub fn create_shelf(hex_string: &str) -> () {
    let (r, g, b) = parse_hex_string(hex_string);
    println!("r, g, b = {}, {}, {}", r, g, b);
}

// Checks whether a given string is a valid hex string.
//
// See https://docs.rs/clap/latest/clap/struct.Arg.html#method.validator
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