use std::{
    fs::File,
    io::{BufReader, BufRead},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buffer = BufReader::new(file);

    buffer.lines().map(|line| line.expect("line could not be parsed")).collect()
}
