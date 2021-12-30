use std::ffi::OsStr;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

use minify_html::{Cfg, minify};
use walkdir::WalkDir;

use clap::{App, SubCommand};

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("render_html")
        .about("Render the HTML files for the site")
}

#[derive(Debug)]
pub enum RenderHtmlError {
    Io(io::Error),
    Walk(walkdir::Error),
    Python(&'static str),
}

impl fmt::Display for RenderHtmlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RenderHtmlError::Io(ref err)   => write!(f, "IO error: {}", err),
            RenderHtmlError::Walk(ref err) => write!(f, "Walkdir error: {}", err),
            RenderHtmlError::Python(ref err) => write!(f, "Python error: {}", err),
        }
    }
}

impl From<io::Error> for RenderHtmlError {
    fn from(err: io::Error) -> RenderHtmlError {
        RenderHtmlError::Io(err)
    }
}

impl From<walkdir::Error> for RenderHtmlError {
    fn from(err: walkdir::Error) -> RenderHtmlError {
        RenderHtmlError::Walk(err)
    }
}

fn read_file(p: &Path) -> Result<Vec<u8>, io::Error> {
    let f = File::open(p)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

fn write_file(p: &Path, bytes: Vec<u8>) -> Result<(), io::Error> {
    let mut f = File::create(p)?;
    f.write_all(bytes.as_slice())?;

    Ok(())
}

/// Minify all the HTML files found anywhere in or under the given directory.
fn minify_html(root: &Path) -> Result<(), RenderHtmlError> {
    let cfg = Cfg::new();

    for entry in WalkDir::new(root) {
        let entry = entry?;

        if entry.path().extension() == Some(OsStr::new("html")) {
            let html = read_file(entry.path())?;
            let minified_html = minify(&html, &cfg);
            write_file(entry.path(), minified_html)?;
        }
    }

    Ok(())
}

pub fn render_html() -> Result<(), RenderHtmlError> {
    let status =
        Command::new("python3")
            .arg("scripts/render_html.py")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()?;

    if !status.success() {
        return Err(RenderHtmlError::Python("Python script did not exit successfully"))
    }

    println!("status = {}", status);

    minify_html(Path::new("_html"))
}
