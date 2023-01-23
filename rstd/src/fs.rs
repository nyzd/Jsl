use std::fs::File;
use std::io::{self, Read, Write};
use std::path::PathBuf;

pub fn read_file(path: PathBuf) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn create_file(path: PathBuf, content: String) -> io::Result<()> {
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;

    Ok(())
}
