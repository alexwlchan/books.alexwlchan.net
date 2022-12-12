use std::ffi::OsStr;
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
    // Ensure the parent directory exists before we write to a file,
    // otherwise we get errors like:
    //
    //    Error rendering HTML: IO error: No such file or directory (os error 2)
    //
    fs::create_dir_all(p.parent().unwrap())?;

    let mut f = File::create(p)?;
    f.write_all(bytes.as_slice())?;

    Ok(())
}

pub fn is_ds_store(p: &Path) -> bool {
    p.file_name() == Some(&OsStr::new(".DS_Store"))
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

            _ => Ok(true),
        }
    }
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
pub fn sync_files(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();

        if is_ds_store(&src_path) {
            continue;
        }

        if src_path.file_name() == Some(OsStr::new("tests")) {
            continue;
        }

        let dst_path = dst.join(src_path.file_name().unwrap());

        if src_path.is_newer_than(&dst_path)? {
            fs::copy(src_path, dst_path)?;
        }
    }

    Ok(())
}
