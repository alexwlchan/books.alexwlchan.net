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
use std::convert::Infallible;
use std::io::Write;
use std::net::SocketAddr;
use std::path::Path;
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

#[macro_use]
extern crate lazy_static;

mod add_review;
mod colours;
mod create_shelf;
mod errors;
mod fs_helpers;
mod models;
mod render_html;
mod templates;
mod text_helpers;
mod urls;
mod version;

use axum::{http::StatusCode, service, Router};
use clap::{App, AppSettings, SubCommand};
use tower_http::services::ServeDir;

use render_html::{create_thumbnails, render_html, sync_static_files, HtmlRenderMode};

fn create_html_pages(mode: HtmlRenderMode) {
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
        Err(err) => eprintln!("💥 Error rendering HTML: {}", err),
    };

    let elapsed = start.elapsed();
    if elapsed.as_secs() == 0 {
        println!("done in {:?}ms", elapsed.as_millis());
    } else {
        println!("done in {:.1}s", elapsed.as_secs_f32());
    }
}

fn create_static_files() {
    let start = Instant::now();
    print!("Syncing static files... ");

    match sync_static_files(Path::new("_html")) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 Error syncing static files: {}", err),
    };

    let elapsed = start.elapsed();
    if elapsed.as_secs() == 0 {
        println!("done in {:?}ms", elapsed.as_millis());
    } else {
        println!("done in {:.1}s", elapsed.as_secs_f32());
    }
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

    let elapsed = start.elapsed();
    if elapsed.as_secs() == 0 {
        println!("done in {:?}ms", elapsed.as_millis());
    } else {
        println!("done in {:.1}s", elapsed.as_secs_f32());
    }
}

pub fn build_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("build").about("Build the HTML pages for the site")
}

pub fn serve_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("serve").about("Run a local web server with the site and live changes")
}

pub fn deploy_subcommand() -> App<'static, 'static> {
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
        add_review::add_review();
    }

    // Whatever the command is, we always want to build a fresh copy of the
    // site before doing anything else.

    create_html_pages(HtmlRenderMode::Full);
    create_static_files();
    create_images();

    if matches.subcommand_name() == Some("build") {
        std::process::exit(0);
    }

    if matches.subcommand_name() == Some("add_review") || matches.subcommand_name() == Some("serve")
    {
        tokio::task::spawn_blocking(move || {
            let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");

            hotwatch
                .watch("covers", |_| {
                    create_images();
                })
                .expect("failed to watch covers folder!");
            hotwatch
                .watch("reviews", |_| {
                    create_html_pages(HtmlRenderMode::Incremental);
                })
                .expect("failed to watch reviews folder!");
            hotwatch
                .watch("static", |_| {
                    create_static_files();
                })
                .expect("failed to watch static folder!");
            hotwatch
                .watch("templates", |_| {
                    create_html_pages(HtmlRenderMode::Full);
                })
                .expect("failed to watch templates folder!");

            loop {
                thread::sleep(Duration::from_secs(1));
            }
        });

        let app = Router::new().nest(
            "/",
            service::get(ServeDir::new("_html")).handle_error(|error: std::io::Error| {
                Ok::<_, Infallible>((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {}", error),
                ))
            }),
        );

        let addr = SocketAddr::from(([127, 0, 0, 1], 5959));
        println!("🚀 Serving site on http://localhost:5959");
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    if matches.subcommand_name() == Some("deploy") {
        println!("Deploying to Netlify...");

        let status = Command::new("netlify")
            .args(vec!["deploy", "--prod"])
            .status()
            .unwrap();

        if !status.success() {
            eprintln!("Could not deploy to Netlify!");
            std::process::exit(2);
        }
    }
}
