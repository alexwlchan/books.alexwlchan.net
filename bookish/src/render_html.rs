use std::ffi::OsStr;
use std::fmt;
use std::fs::{self, File};
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

fn write_file(p: &Path, bytes: Vec<u8>) -> io::Result<()> {
    let mut f = File::create(p)?;
    f.write_all(bytes.as_slice())?;

    Ok(())
}

/// Do a basic sync of files from one directory to another.
///
/// It uses the modified time to decide whether to re-copy a file from the source
/// to the destination; files are only copied if they're newer in the source.
///
/// This function:
///
///     - Does not remove files from the destination that are no longer in
///       the destination
///     - Only looks at files that are directly below the source dir
///
fn sync_files(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;

        if entry.path().file_name() == Some(&OsStr::new(".DS_Store")) {
            continue;
        }

        let src_path = entry.path();
        let dst_path = dst.join(entry.path().file_name().unwrap());

        // If this was Python, I'd try to get the metadata on dst path and react
        // to its absence -- a more `try … catch` style approach.
        //
        // That approach is more robust and not subject to weird races if `dst_path`
        // pops in or out of existence.
        //
        // TODO: Rewrite this code to use the `try … catch` approach.
        let should_copy = if dst_path.exists() {
            let src_metadata = fs::metadata(&src_path)?;
            let dst_metadata = fs::metadata(&dst_path)?;

            src_metadata.modified()? > dst_metadata.modified()?
        } else {
            true
        };

        if should_copy {
            fs::copy(src_path, dst_path)?;
        }
    }

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
    sync_files(Path::new("static"), Path::new("_html/static/"))?;

    let status =
        Command::new("python3")
            .arg("scripts/render_html.py")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .status()?;

    if !status.success() {
        return Err(RenderHtmlError::Python("Python script did not exit successfully"))
    }

    minify_html(Path::new("_html"))
}
