use std::process::Command;
use std::time::Instant;

use clap::{App, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("render_html")
        .about("Render the HTML files for the site")
}

pub fn render_html() -> () {
    let start = Instant::now();

    Command::new("python3")
            .arg("scripts/render_html.py")
            .output()
            .unwrap();

    let elapsed = start.elapsed();
    println!("✨ Rendered HTML files to _html in {:?}ms ✨", elapsed.as_millis());
}
