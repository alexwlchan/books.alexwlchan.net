use std::io::{BufReader, Read, Write};
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use std::time::Instant;

use minify_html::{Cfg, minify};
use walkdir::WalkDir;

use clap::{App, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("render_html")
        .about("Render the HTML files for the site")
}

fn read_file(p: &Path) -> Vec<u8> {
    let f = File::open(p).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();

    buffer
}

fn write_file(p: &Path, bytes: Vec<u8>) -> () {
    let mut f = File::create(p).unwrap();
    f.write_all(bytes.as_slice()).unwrap();
}

/// Minify all the HTML files found anywhere in or under the given directory.
fn minify_html(root: &Path) -> () {
    let cfg = Cfg::new();

    for entry in WalkDir::new(root) {
        let entry = entry.unwrap();

        if entry.path().extension() == Some(OsStr::new("html")) {
            let html = read_file(entry.path());
            let minified_html = minify(&html, &cfg);
            write_file(entry.path(), minified_html);
        }
    }
}

pub fn render_html() -> () {
    let start = Instant::now();

    Command::new("python3")
            .arg("scripts/render_html.py")
            .output()
            .unwrap();

    minify_html(Path::new("_html"));

    let elapsed = start.elapsed();

    if elapsed.as_secs() == 0 {
        println!("✨ Rendered HTML files to _html in {:?}ms ✨", elapsed.as_millis());
    } else {
        println!("✨ Rendered HTML files to _html in {:.1}s ✨", elapsed.as_secs_f32());
    }
}
