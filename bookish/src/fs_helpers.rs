use std::fs::{self, File};
use std::io::{self, BufReader, Read, Write};
use std::path::Path;

pub fn read_file(p: &Path) -> Result<Vec<u8>, io::Error> {
    let f = File::open(p)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}

pub fn write_file(p: &Path, bytes: Vec<u8>) -> io::Result<()> {
    let mut f = File::create(p)?;
    f.write_all(bytes.as_slice())?;

    Ok(())
}

/// This is used when we generate a derivative file from a source file, where
/// we only want to regenerate the derivative if the source has changed.

pub trait IsNewerThan<T: ?Sized, E> {
    fn is_newer_than(&self, other: &T) -> Result<bool, E>;
}

impl IsNewerThan<Path, io::Error> for Path {
    fn is_newer_than(self: &Path, other: &Path) -> io::Result<bool> {
        match fs::metadata(&other) {
            Ok(other_metadata) => {
                let self_metadata = fs::metadata(&self)?;
                Ok(self_metadata.modified()? > other_metadata.modified()?)
            }

            _ => Ok(false)
        }
    }
}
