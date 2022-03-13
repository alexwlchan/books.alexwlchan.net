use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;

pub fn read_file(p: &Path) -> Result<Vec<u8>, io::Error> {
    let f = File::open(p)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}
