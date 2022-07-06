use std::convert::Infallible;
use std::net::SocketAddr;
use std::thread;
use std::time::Duration;

use axum::{http::StatusCode, service, Router};
use tower_http::services::ServeDir;

use crate::render_html::HtmlRenderMode;

pub async fn run_server(host: &str, port: u16) -> () {
    tokio::task::spawn_blocking(move || {
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");

        hotwatch
            .watch("covers", |_| {
                crate::create_images();

                // We need to recreate the HTML because the dimensions
                // of the cover images get baked into the HTML; if we
                // don't re-render then crops/dimensions may not update
                // correctly.
                crate::create_html_pages(HtmlRenderMode::Full);
            })
            .expect("failed to watch covers folder!");
        hotwatch
            .watch("reviews", |_| {
                crate::create_html_pages(HtmlRenderMode::Incremental);
            })
            .expect("failed to watch reviews folder!");
        hotwatch
            .watch("static", |_| {
                crate::create_static_files();
            })
            .expect("failed to watch static folder!");
        hotwatch
            .watch("templates", |_| {
                crate::create_html_pages(HtmlRenderMode::Full);
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

    let (label, addr) = match host {
        "127.0.0.1" => ("localhost", SocketAddr::from(([127, 0, 0, 1], port))),
        "0.0.0.0" => ("0.0.0.0", SocketAddr::from(([0, 0, 0, 0], port))),
        _ => {
            eprintln!("💥 Unrecognised host: {}", host);
            std::process::exit(1);
        }
    };

    println!("🚀 Serving site on http://{}:{}", label, port);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
