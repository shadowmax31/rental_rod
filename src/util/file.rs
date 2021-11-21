
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io;

pub fn insert(path: &str, line: &str) -> Result<(), io::Error > {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", line)?;

    Ok(())
}
