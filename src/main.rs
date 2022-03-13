// #![deny(warnings)]

#[macro_use]
extern crate lazy_static;

use std::path::Path;
use std::thread;
use std::time::Duration;

mod errors;
mod fs_helpers;
mod models;
mod render_html;
mod templates;

use render_html::render_html;

#[tokio::main]
async fn main() {
    render_html(Path::new("reviews"), Path::new("_html"));

    tokio::task::spawn_blocking(move || {
        println!("listening for changes: reviews");
        let mut hotwatch = hotwatch::Hotwatch::new().expect("hotwatch failed to initialize!");

        // TODO: covers, templates
        hotwatch
            .watch("reviews", |_| {
                println!("Rebuilding site");
                render_html(Path::new("reviews"), Path::new("_html"));
            })
            .expect("failed to watch content folder!");
        loop {
            thread::sleep(Duration::from_secs(1));
        }
    });
}
