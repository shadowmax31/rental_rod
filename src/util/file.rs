
use std::fs::{OpenOptions, File};
use std::io::{BufReader, prelude::*};
use std::path::Path;
use std::io;

pub enum FileError {
    FileNotFound,
    IoError(io::Error)
}

pub fn read(path: &str) -> Result<Vec<String>, FileError>  {
    let file = match File::open(path) {
        Ok(v) => v,
        Err(_) => return Err(FileError::FileNotFound)
    };

    let buf = BufReader::new(file);
    Ok(buf.lines().map(|l| l.expect("Cannot parse line")).collect())
}

pub fn insert(path: &str, line: &str) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    writeln!(file, "{}", line)?;

    Ok(())
}

pub fn write(path: &str, version: &str, lines: &Vec<String>) -> Result<(), io::Error> {
    remove_file(path)?;

    insert(path, version)?;
    for line in lines {
        insert(path, line)?;
    }

    Ok(())
}

pub fn remove_file(s_path: &str) -> Result<(), io::Error> {
    let path = Path::new(s_path);
    if path.exists() {
        std::fs::remove_file(s_path)?;
    }

    Ok(())
}

