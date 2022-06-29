use std::fmt;
use std::io;
use std::path::PathBuf;

use html_minifier::HTMLMinifierError;
use image::error::ImageError;

#[derive(Debug)]
pub enum VfdError {
    Io(io::Error),
    Walk(walkdir::Error),
    Parse(serde_yaml::Error, PathBuf),
    Utf8(std::str::Utf8Error, PathBuf),
    CoverInfo(ImageError, PathBuf),
    Thumbnail(ImageError),
    Template(tera::Error),
    HtmlMinification(HTMLMinifierError),
}

impl fmt::Display for VfdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VfdError::Io(ref err) => write!(f, "IO error: {}", err),
            VfdError::Walk(ref err) => write!(f, "Walkdir error: {}", err),
            VfdError::Parse(ref err, ref path) => {
                write!(f, "Couldn't parse file {:?}: {}", path, err)
            }
            VfdError::Utf8(ref err, ref path) => {
                write!(f, "Couldn't read {:?} as a UTF-8 string: {}", path, err)
            }
            VfdError::CoverInfo(ref err, ref path) => write!(f, "Error getting cover info for {:?}: {}", path, err),
            VfdError::Thumbnail(ref err) => write!(f, "Couldn't create thumbnail: {:?}", err),
            VfdError::Template(ref err) => write!(f, "Error rendering the template: {:?}", err),
            VfdError::HtmlMinification(ref err) => write!(f, "Error minifying the HTML: {:?}", err),
        }
    }
}

impl From<io::Error> for VfdError {
    fn from(err: io::Error) -> VfdError {
        VfdError::Io(err)
    }
}

impl From<walkdir::Error> for VfdError {
    fn from(err: walkdir::Error) -> VfdError {
        VfdError::Walk(err)
    }
}

impl From<tera::Error> for VfdError {
    fn from(err: tera::Error) -> VfdError {
        VfdError::Template(err)
    }
}

impl From<HTMLMinifierError> for VfdError {
    fn from(err: HTMLMinifierError) -> VfdError {
        VfdError::HtmlMinification(err)
    }
}
