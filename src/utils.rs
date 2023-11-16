use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub(crate) fn read_file (target_file: &str) -> Vec<String> {
    let file = File::open(target_file).expect("Unable to open file");
    let reader = BufReader::new(file);

    reader.lines().map(|l| l.unwrap()).collect()
}

pub(crate) fn write_file (target_file: &str, content: &str) {
    // Create file if it does not exist, ignore if it does
    let mut file = File::create(target_file).unwrap();

    // Write the content to the file
    file.write_all(content.as_bytes()).unwrap();
}