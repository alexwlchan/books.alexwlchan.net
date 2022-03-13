// #![deny(warnings)]

use std::convert::Infallible;
use std::net::SocketAddr;
use std::path::Path;
use std::thread;
use std::time::Duration;

#[macro_use]
extern crate lazy_static;

mod colours;
mod create_shelf;
mod errors;
mod fs_helpers;
mod models;
mod render_html;
mod templates;
mod text_helpers;

use axum::{http::StatusCode, service, Router};
use tower_http::services::ServeDir;

use render_html::{create_thumbnails, render_html, sync_static_files};

#[tokio::main]
async fn main() {
    let cached_templates = templates::get_templates().unwrap();

    match render_html(&cached_templates, Path::new("reviews"), Path::new("_html")) {
        Ok(_) => (),
        Err(err) => eprintln!("💥 Error rendering HTML: {}", err),
    };
    sync_static_files(Path::new("_html"));
    create_thumbnails(Path::new("_html"));

    tokio::task::spawn_blocking(move || {
        println!("listening for changes: reviews");
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");

        hotwatch
            .watch("covers", |_| {
                create_thumbnails(Path::new("_html"));
            })
            .expect("failed to watch covers folder!");
        hotwatch
            .watch("reviews", |_| {
                let cached_templates = templates::get_templates().unwrap();
                render_html(&cached_templates, Path::new("reviews"), Path::new("_html")); })
            .expect("failed to watch reviews folder!");
        hotwatch
            .watch("static", |_| { sync_static_files(Path::new("_html")); })
            .expect("failed to watch static folder!");
        hotwatch
            .watch("templates", |_| {
                // TODO: Can I skip getting templates when other dirs change?
                let cached_templates = templates::get_templates().unwrap();
                render_html(&cached_templates, Path::new("reviews"), Path::new("_html"));
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
    println!("serving site on http://localhost:5959");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
