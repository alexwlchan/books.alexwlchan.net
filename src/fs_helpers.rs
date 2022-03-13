use std::fs::File;
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
