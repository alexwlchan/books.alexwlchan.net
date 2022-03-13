#![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;
use std::thread;
use std::time::Duration;

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

use axum::{http::StatusCode, service, Router};
use clap::{App, SubCommand, AppSettings};
use tower_http::services::ServeDir;

use render_html::{create_thumbnails, render_html, sync_static_files};

fn create_html_pages() {
    // This was an idea where I'd cache the templates between runs, so I could
    // detect whether the templates/source data had changed and skip re-reading
    // the unchanged data.  I haven't finished it yet, but it's still here as
    // an option.
    let cached_templates = templates::get_templates().unwrap();

    match render_html(&cached_templates, Path::new("reviews"), Path::new("_html")) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 Error rendering HTML: {}", err),
    };
}

fn create_static_files() {
    match sync_static_files(Path::new("_html")) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 Error syncing static files: {}", err),
    };
}

fn create_images() {
    match create_thumbnails(Path::new("_html")) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 Error creating thumbnail images: {}", err),
    };
}

pub fn serve_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("serve")
        .about("Render the HTML files for the site")
}

#[tokio::main]
async fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");

    let app =
        App::new("vfd")
            .version(VERSION)
            .author("Alex Chan <alex@alexwlchan.net>")
            .about("Generates the HTML files for books.alexwlchan.net")
            .setting(AppSettings::SubcommandRequired)
            .subcommand(add_review::subcommand())
            .subcommand(serve_subcommand());

    let matches = app.get_matches();

    if matches.subcommand_name() == Some("add_review") {
        add_review::add_review();
    }

    // otherwise we're in serve or add_review, in either case start serving
    // the site locally

    create_html_pages();
    create_static_files();
    create_images();

    tokio::task::spawn_blocking(move || {
        let mut hotwatch = hotwatch::Hotwatch::new()
            .expect("hotwatch failed to initialize!");

        hotwatch
            .watch("covers", |_| { create_images(); })
            .expect("failed to watch covers folder!");
        hotwatch
            .watch("reviews", |_| { create_html_pages(); })
            .expect("failed to watch reviews folder!");
        hotwatch
            .watch("static", |_| { create_static_files(); })
            .expect("failed to watch static folder!");
        hotwatch
            .watch("templates", |_| { create_html_pages(); })
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
    println!("serving site on http://localhost:5959");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
