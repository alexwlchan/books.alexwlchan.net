#![deny(warnings)]

use std::process::exit;
use std::time::Instant;

use clap::{App, AppSettings};

mod add_review;
mod colours;
mod create_shelf;
mod fs_helpers;
mod models;
mod render_html;
mod text;
mod urls;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let app =
        App::new("bookish")
            .version(VERSION)
            .author("Alex Chan <alex@alexwlchan.net>")
            .about("Generates the HTML files for books.alexwlchan.net")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(add_review::subcommand())
            .subcommand(create_shelf::subcommand())
            .subcommand(render_html::subcommand());

    let matches = app.get_matches();

    match matches.subcommand() {
        ("add_review", _) => add_review::add_review(),
        ("create_shelf", Some(sub_m)) => {
            // We can safely call .unwrap() here because this argument is required;
            // if it wasn't supplied then Clap has already bailed out.
            let hex_string = sub_m.value_of("COLOUR").unwrap();

            create_shelf::create_shelf(&hex_string);
        },

        ("render_html", _) => {
            let start = Instant::now();

            match render_html::render_html() {
                Ok(_) => {
                    let elapsed = start.elapsed();

                    if elapsed.as_secs() == 0 {
                        println!("✨ Rendered HTML files to _html in {:?}ms ✨", elapsed.as_millis());
                    } else {
                        println!("✨ Rendered HTML files to _html in {:.1}s ✨", elapsed.as_secs_f32());
                    }
                }

                Err(e) => {
                    eprintln!("💥 Something went wrong! 💥\n{}", e);
                    exit(1);
                }
            }


        },

        _ => unreachable!(),
    };
}
