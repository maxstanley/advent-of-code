use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use crate::error::Error;

pub fn read_lines_from_file<F>(filepath: F) -> Result<Vec<String>, Error>
where
    F: AsRef<Path>,
{
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    reader.lines().map(|l| l.map_err(Into::into)).collect()
}
