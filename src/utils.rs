use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Write, Result};
use std::path::Path;

pub(crate) fn read_file (target_file: &str) -> Result<Vec<String>> {
    if !Path::new(target_file).exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }
    let file = File::open(target_file).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader.lines().collect()
}

pub(crate) fn write_file (target_file: &str, content: &str) {
    let mut file = File::create(target_file).unwrap();
    file.write_all(content.as_bytes()).unwrap();
}