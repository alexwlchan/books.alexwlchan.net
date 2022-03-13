use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum VfdError {
    Io(io::Error),
    Walk(walkdir::Error),
    Parse(serde_yaml::Error, PathBuf),
    Utf8(std::str::Utf8Error, PathBuf),
}

impl fmt::Display for VfdError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VfdError::Io(ref err)              => write!(f, "IO error: {}", err),
            VfdError::Walk(ref err)            => write!(f, "Walkdir error: {}", err),
            VfdError::Parse(ref err, ref path) => write!(f, "Couldn't parse file {:?}: {}", path, err),
            VfdError::Utf8(ref err, ref path)  => write!(f, "Couldn't read {:?} as a UTF-8 string: {}", path, err),
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
