#![deny(warnings)]

mod create_shelf;

use clap::{App, AppSettings};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let app =
        App::new("bookish")
            .version(VERSION)
            .author("Alex Chan <alex@alexwlchan.net>")
            .about("Generates the HTML files for books.alexwlchan.net")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(create_shelf::subcommand());

    let matches = app.get_matches();

    match matches.subcommand() {
        ("create_shelf", Some(sub_m)) => {
            // We can safely call .unwrap() here because this argument is required;
            // if it wasn't supplied then Clap has already bailed out.
            let hex_string = sub_m.value_of("COLOUR").unwrap();

            create_shelf::create_shelf(&hex_string);
        },
        _ => unreachable!(),
    };
}
