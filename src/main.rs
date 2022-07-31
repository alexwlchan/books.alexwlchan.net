#![deny(warnings)]

/// This is a tool for generating the static files for my book tracker.
///
/// It needs four input directories:
///
///   - `reviews` contains Markdown files with YAML front matter, one for each
///     book I've read.  The front matter should match the `Metadata` model
///     in `models.rs`, and the Markdown is free text that will be used for
///     the body of the review.
///
///   - `covers` contains cover images for each of the books.  These will be resized
///     to the appropriate size (e.g. 480px high for thumbnails), to avoid sending
///     unnecessarily large images to the user's browser.
///
///   - `static` contains files that should be copied unmodified, e.g. CSS styles.
///
///   - `templates` contains HTML templates for the site, which use the Tera template
///     engine (similar to Jinja2 and Django).
///
/// The code is somewhat scrappy Rust and shouldn't be taken as an example of how
/// to write Rust, but it works well enough.
use std::io::{ErrorKind, Write};
use std::path::Path;
use std::process::Command;
use std::time::Instant;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;

mod add_review;
mod colours;
mod create_favicon;
mod create_shelf;
mod errors;
mod fs_helpers;
mod models;
mod render_html;
mod serve;
mod templates;
mod text_helpers;
mod urls;
mod version;

use clap::{App, AppSettings, Arg, SubCommand};
use inquire::error::InquireError;

use crate::render_html::{create_thumbnails, render_html, sync_static_files, HtmlRenderMode};

fn print_elapsed(start: Instant) -> () {
    let elapsed = start.elapsed();

    if elapsed.as_secs() == 0 && elapsed.as_millis() == 0 {
        println!("done in <1ms");
    } else if elapsed.as_secs() == 0 {
        println!("done in {:?}ms", elapsed.as_millis());
    } else {
        println!("done in {:.1}s", elapsed.as_secs_f32());
    }
}

pub fn create_html_pages(mode: HtmlRenderMode) {
    let start = Instant::now();
    print!("Building HTML pages... ");

    // This was an idea where I'd cache the templates between runs, so I could
    // detect whether the templates/source data had changed and skip re-reading
    // the unchanged data.  I haven't finished it yet, but it's still here as
    // an option.
    let cached_templates = templates::get_templates().unwrap();

    match render_html(
        &cached_templates,
        Path::new("reviews"),
        Path::new("_html"),
        mode,
    ) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 {}", err),
    };

    print_elapsed(start);
}

fn create_static_files() {
    let start = Instant::now();
    print!("Syncing static files... ");

    match sync_static_files(Path::new("_html")) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 Error syncing static files: {}", err),
    };

    print_elapsed(start);
}

fn create_images() {
    let start = Instant::now();
    print!("Creating thumbnail images... ");

    // We flush stdout here because it can sometimes take a long time
    // to generate thumbnail images (if we're starting from cold); if so,
    // we want the message above to appear immediately.
    std::io::stdout().flush().unwrap();

    match create_thumbnails(Path::new("_html")) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 Error creating thumbnail images: {}", err),
    };

    print_elapsed(start);
}

pub fn build_subcommand() -> App<'static> {
    SubCommand::with_name("build").about("Build the HTML pages for the site")
}

pub fn serve_subcommand() -> App<'static> {
    SubCommand::with_name("serve")
        .about("Run a local web server with the site and live changes")
        .arg(
            Arg::with_name("host")
                .long("host")
                .value_parser(["127.0.0.1", "0.0.0.0"])
                .default_value("127.0.0.1")
                .help("Specify an address to bind to")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .default_value("5959")
                .help("Specify a port to bind to"),
        )
}

pub fn deploy_subcommand() -> App<'static> {
    SubCommand::with_name("deploy").about("Deploy a new version of the site to Netlify")
}

#[tokio::main]
async fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    // Before we start up, check if we're running the latest version.
    //
    // This logic is a bit rough, e.g. it'll warn if you're running a
    // newer version than what's on GitHub, but it's good enough for
    // my purposes.
    match version::get_latest_version().await {
        Ok(Some(latest_version)) if latest_version.strip_prefix("v") != Some(VERSION) => {
            println!("\x1b[96mA newer version of vfd is available, please update:\nhttps://github.com/alexwlchan/books.alexwlchan.net/releases/tag/{latest_version}\x1b[0m");
        }
        _ => (),
    };

    let app = App::new("vfd")
        .version(VERSION)
        .author("Alex Chan <alex@alexwlchan.net>")
        .about("Generates the HTML files for books.alexwlchan.net")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(add_review::subcommand())
        .subcommand(build_subcommand())
        .subcommand(deploy_subcommand())
        .subcommand(serve_subcommand());

    let matches = app.get_matches();

    if matches.subcommand_name() == Some("add_review") {
        match add_review::add_review() {
            Ok(_) => std::process::exit(0),
            Err(InquireError::OperationInterrupted) => {
                eprintln!("^C");
                std::process::exit(1);
            }
            Err(InquireError::OperationCanceled) => {
                eprintln!("<esc>");
                std::process::exit(1);
            }
            Err(e) => {
                eprintln!("Error adding review: {}", e);
                std::process::exit(1);
            }
        }
    }

    // Whatever the command is, we always want to build a fresh copy of the
    // site before doing anything else.

    create_html_pages(HtmlRenderMode::Full);
    create_static_files();
    create_images();

    match matches.subcommand() {
        Some(("build", _)) => {
            std::process::exit(0);
        }

        Some(("serve", sub_m)) => {
            let host = sub_m.value_of("host").unwrap();

            // Get the port as a number.
            // See https://github.com/clap-rs/clap/blob/v2.33.1/examples/12_typed_values.rs
            let port = value_t!(sub_m, "port", u16).unwrap_or_else(|e| e.exit());

            crate::serve::run_server(host, port).await;
        }

        Some(("deploy", _)) => {
            println!("Deploying to Netlify...");

            let status = match Command::new("netlify")
                .args(vec!["deploy", "--prod"])
                .status()
            {
                Ok(result) => (result),
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => {
                        eprintln!("💥 Could not find the Netlify CLI; is it installed?");
                        std::process::exit(1);
                    }
                    _ => {
                        eprintln!("💥 Error deploying to Netlify: {}", err);
                        std::process::exit(1);
                    }
                },
            };

            if !status.success() {
                eprintln!("Could not deploy to Netlify!");
                std::process::exit(2);
            }
        }

        _ => {}
    };
}
